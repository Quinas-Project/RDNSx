//! Database export modules

pub mod cassandra;
pub mod elasticsearch;
pub mod mongodb;

pub use cassandra::CassandraExporter;
pub use elasticsearch::ElasticsearchExporter;
pub use mongodb::MongodbExporter;

use async_trait::async_trait;
use crate::error::Result;
use crate::types::DnsRecord;

/// Common export trait
#[async_trait]
pub trait Exporter: Send + Sync {
    /// Export a DNS record
    async fn export(&self, record: DnsRecord) -> Result<()>;

    /// Flush any pending writes
    async fn flush(&self) -> Result<()>;
}
