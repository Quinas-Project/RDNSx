//! Elasticsearch exporter

use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use elasticsearch::{
    Elasticsearch, http::transport::Transport,
    indices::IndicesCreateParts,
    BulkParts,
};
use serde_json::{json, Value};
use tokio::sync::Mutex;
use tracing::debug;

use crate::error::{DnsxError, Result};
use crate::export::Exporter;
use crate::types::DnsRecord;

/// Elasticsearch exporter
pub struct ElasticsearchExporter {
    client: Arc<Elasticsearch>,
    index: String,
    batch_size: usize,
    buffer: Arc<Mutex<Vec<Value>>>,
}

impl ElasticsearchExporter {
    /// Create a new Elasticsearch exporter
    pub async fn new(url: &str, index: &str, batch_size: usize) -> Result<Self> {
        let transport = Transport::single_node(url)?;
        let client = Arc::new(Elasticsearch::new(transport));

        // Ensure index exists with proper mapping
        ensure_index(&client, index).await?;

        Ok(Self {
            client,
            index: index.to_string(),
            batch_size,
            buffer: Arc::new(Mutex::new(Vec::new())),
        })
    }

    /// Flush buffer to Elasticsearch
    async fn flush_buffer(&self) -> Result<()> {
        let mut buffer = self.buffer.lock().await;
        if buffer.is_empty() {
            return Ok(());
        }

        // Build bulk request body (NDJSON format)
        let mut bulk_body = String::new();
        for doc in buffer.drain(..) {
            // Add index action line
            let action = json!({
                "index": {
                    "_index": self.index
                }
            });
            bulk_body.push_str(&serde_json::to_string(&action)?);
            bulk_body.push('\n');
            // Add document line
            bulk_body.push_str(&serde_json::to_string(&doc)?);
            bulk_body.push('\n');
        }

        let response = self
            .client
            .bulk(BulkParts::Index(&self.index))
            .body(bulk_body.as_bytes())
            .send()
            .await
            .map_err(|e| DnsxError::Export(format!("Elasticsearch bulk error: {}", e)))?;

        if !response.status_code().is_success() {
            let status = response.status_code();
            let text = response.text().await.unwrap_or_default();
            return Err(DnsxError::Export(format!(
                "Elasticsearch bulk failed: {} - {}",
                status, text
            )));
        }

        debug!("Flushed {} documents to Elasticsearch", buffer.len() / 2);
        Ok(())
    }
}

/// Ensure index exists with proper mapping
async fn ensure_index(client: &Elasticsearch, index: &str) -> Result<()> {
        // Try to create index (will fail silently if it already exists)
        // Create index with mapping
        let mapping = json!({
            "mappings": {
                "properties": {
                    "@timestamp": {
                        "type": "date"
                    },
                    "domain": {
                        "type": "keyword"
                    },
                    "record_type": {
                        "type": "keyword"
                    },
                    "value": {
                        "type": "text",
                        "fields": {
                            "keyword": {
                                "type": "keyword"
                            }
                        }
                    },
                    "resolver": {
                        "type": "keyword"
                    },
                    "ttl": {
                        "type": "integer"
                    },
                    "response_code": {
                        "type": "keyword"
                    },
                    "query_time_ms": {
                        "type": "float"
                    }
                }
            }
        });

        let result = client
            .indices()
            .create(IndicesCreateParts::Index(index))
            .body(mapping)
            .send()
            .await;

        // Ignore error if index already exists
        match result {
            Ok(_) => debug!("Created Elasticsearch index: {}", index),
            Err(e) => {
                // Index might already exist, that's okay
                debug!("Index creation result (may already exist): {:?}", e);
            }
        }

    Ok(())
}

#[async_trait]
impl Exporter for ElasticsearchExporter {
    async fn export(&self, record: DnsRecord) -> Result<()> {
        let timestamp = DateTime::<Utc>::from(record.timestamp);

        let doc = json!({
            "@timestamp": timestamp.to_rfc3339(),
            "domain": record.domain,
            "record_type": format!("{}", record.record_type),
            "value": record.value.to_string(),
            "resolver": record.resolver,
            "ttl": record.ttl,
            "response_code": format!("{}", record.response_code),
            "query_time_ms": record.query_time_ms,
        });

        let mut buffer = self.buffer.lock().await;
        buffer.push(doc);

        // Flush if buffer is full
        if buffer.len() >= self.batch_size {
            drop(buffer);
            self.flush_buffer().await?;
        }

        Ok(())
    }

    async fn flush(&self) -> Result<()> {
        self.flush_buffer().await
    }
}
