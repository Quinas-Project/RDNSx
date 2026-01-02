//! Cassandra exporter for DNS records

use std::sync::Arc;
use std::collections::HashMap;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use scylla::{Session, SessionBuilder};
use tokio::sync::Mutex;
use tracing::{debug, error, warn, info};

use crate::error::{DnsxError, Result};
use crate::export::Exporter;
use crate::types::{DnsRecord, RecordType};

/// Cassandra exporter for DNS records
pub struct CassandraExporter {
    /// Cassandra session
    session: Arc<Session>,
    /// Keyspace name
    keyspace: String,
    /// Table name
    table: String,
    /// Batch size for bulk inserts
    batch_size: usize,
    /// Buffer for batching records
    buffer: Arc<Mutex<Vec<DnsRecord>>>,
}

impl CassandraExporter {
    /// Create a new Cassandra exporter
    pub async fn new(
        contact_points: &[String],
        username: Option<&str>,
        password: Option<&str>,
        keyspace: &str,
        table: &str,
        batch_size: usize,
    ) -> Result<Self> {
        info!("Connecting to Cassandra cluster: {:?}", contact_points);

        let mut session_builder = SessionBuilder::new()
            .known_nodes(contact_points)
            .compression(Some(scylla::frame::Compression::Lz4));

        if let (Some(user), Some(pass)) = (username, password) {
            session_builder = session_builder.user(user, pass);
        }

        let session = session_builder
            .build()
            .await
            .map_err(|e| DnsxError::Other(format!("Failed to connect to Cassandra: {}", e)))?;

        // Create keyspace if it doesn't exist
        Self::create_keyspace(&session, keyspace).await?;

        // Create table if it doesn't exist
        Self::create_table(&session, keyspace, table).await?;

        info!("Connected to Cassandra, using keyspace: {}, table: {}", keyspace, table);

        Ok(Self {
            session: Arc::new(session),
            keyspace: keyspace.to_string(),
            table: table.to_string(),
            batch_size,
            buffer: Arc::new(Mutex::new(Vec::new())),
        })
    }

    /// Create keyspace if it doesn't exist
    async fn create_keyspace(session: &Session, keyspace: &str) -> Result<()> {
        let cql = format!(
            "CREATE KEYSPACE IF NOT EXISTS {} WITH REPLICATION = {{
                'class': 'SimpleStrategy',
                'replication_factor': 1
            }}",
            keyspace
        );

        session
            .query(cql, &[])
            .await
            .map_err(|e| DnsxError::Other(format!("Failed to create keyspace: {}", e)))?;

        debug!("Keyspace '{}' created or already exists", keyspace);
        Ok(())
    }

    /// Create table if it doesn't exist
    async fn create_table(session: &Session, keyspace: &str, table: &str) -> Result<()> {
        let cql = format!(
            "CREATE TABLE IF NOT EXISTS {}.{} (
                domain text,
                record_type text,
                value text,
                ttl int,
                response_code text,
                resolver text,
                timestamp timestamp,
                query_time_ms double,
                PRIMARY KEY ((domain, record_type), timestamp)
            ) WITH CLUSTERING ORDER BY (timestamp DESC)",
            keyspace, table
        );

        session
            .query(cql, &[])
            .await
            .map_err(|e| DnsxError::Other(format!("Failed to create table: {}", e)))?;

        debug!("Table '{}.{}' created or already exists", keyspace, table);
        Ok(())
    }

    /// Insert a batch of records
    async fn insert_batch(&self, records: Vec<DnsRecord>) -> Result<()> {
        if records.is_empty() {
            return Ok(());
        }

        let prepared = self
            .session
            .prepare(format!(
                "INSERT INTO {}.{} (domain, record_type, value, ttl, response_code, resolver, timestamp, query_time_ms) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
                self.keyspace, self.table
            ))
            .await
            .map_err(|e| DnsxError::Other(format!("Failed to prepare statement: {}", e)))?;

        for record in records {
            let record_type = record.record_type.to_string();
            let response_code = record.response_code.to_string();
            let timestamp = record.timestamp.timestamp_millis();

            let result = self
                .session
                .execute(
                    &prepared,
                    (
                        &record.domain,
                        &record_type,
                        &record.value.to_string(),
                        record.ttl as i32,
                        &response_code,
                        &record.resolver,
                        timestamp,
                        record.query_time_ms,
                    ),
                )
                .await;

            if let Err(e) = result {
                error!("Failed to insert record for {}: {}", record.domain, e);
                return Err(DnsxError::Other(format!("Failed to insert record: {}", e)));
            }
        }

        debug!("Inserted {} records into Cassandra", records.len());
        Ok(())
    }
}

#[async_trait]
impl Exporter for CassandraExporter {
    async fn export(&self, record: DnsRecord) -> Result<()> {
        let mut buffer = self.buffer.lock().await;

        buffer.push(record);

        if buffer.len() >= self.batch_size {
            let records_to_insert = buffer.drain(..).collect();
            drop(buffer); // Release lock before async operation

            if let Err(e) = self.insert_batch(records_to_insert).await {
                error!("Failed to flush Cassandra buffer: {}", e);
                return Err(e);
            }
        }

        Ok(())
    }

    async fn flush(&self) -> Result<()> {
        let mut buffer = self.buffer.lock().await;

        if !buffer.is_empty() {
            let records_to_insert = buffer.drain(..).collect();
            drop(buffer); // Release lock before async operation

            self.insert_batch(records_to_insert).await?;
        }

        info!("Cassandra exporter flushed successfully");
        Ok(())
    }
}