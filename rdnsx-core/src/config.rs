//! Configuration types for RDNSx

use std::time::Duration;

use serde::{Deserialize, Serialize};

/// Default resolvers (Google, Cloudflare, Quad9)
pub const DEFAULT_RESOLVERS: &[&str] = &["8.8.8.8", "8.8.4.4", "1.1.1.1", "1.0.0.1", "9.9.9.9"];

/// Default query timeout
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

/// Default number of retries
pub const DEFAULT_RETRIES: u32 = 3;

/// Default concurrency level
pub const DEFAULT_CONCURRENCY: usize = 100;

/// Default rate limit (queries per second, 0 = unlimited)
pub const DEFAULT_RATE_LIMIT: u64 = 0;

/// Default export batch size
pub const DEFAULT_EXPORT_BATCH_SIZE: usize = 1000;

/// DNSx client options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsxOptions {
    /// DNS resolvers to use
    pub resolvers: Vec<String>,
    /// Query timeout
    pub timeout: Duration,
    /// Number of retries for failed queries
    pub retries: u32,
    /// Maximum concurrent queries
    pub concurrency: usize,
    /// Rate limit (queries per second, 0 = unlimited)
    pub rate_limit: u64,
}

impl Default for DnsxOptions {
    fn default() -> Self {
        Self {
            resolvers: DEFAULT_RESOLVERS.iter().map(|s| s.to_string()).collect(),
            timeout: DEFAULT_TIMEOUT,
            retries: DEFAULT_RETRIES,
            concurrency: DEFAULT_CONCURRENCY,
            rate_limit: DEFAULT_RATE_LIMIT,
        }
    }
}

impl DnsxOptions {
    /// Create new options with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set resolvers
    pub fn with_resolvers(mut self, resolvers: Vec<String>) -> Self {
        self.resolvers = resolvers;
        self
    }

    /// Set timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set retries
    pub fn with_retries(mut self, retries: u32) -> Self {
        self.retries = retries;
        self
    }

    /// Set concurrency
    pub fn with_concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    /// Set rate limit
    pub fn with_rate_limit(mut self, rate_limit: u64) -> Self {
        self.rate_limit = rate_limit;
        self
    }
}

/// Export configuration
#[derive(Debug, Clone)]
pub struct ExportConfig {
    /// Elasticsearch connection string (optional)
    pub elasticsearch_url: Option<String>,
    /// Elasticsearch index name
    pub elasticsearch_index: String,
    /// MongoDB connection string (optional)
    pub mongodb_url: Option<String>,
    /// MongoDB database name
    pub mongodb_database: String,
    /// MongoDB collection name
    pub mongodb_collection: String,
    /// Cassandra contact points (optional)
    pub cassandra_contact_points: Option<Vec<String>>,
    /// Cassandra username (optional)
    pub cassandra_username: Option<String>,
    /// Cassandra password (optional)
    pub cassandra_password: Option<String>,
    /// Cassandra keyspace name
    pub cassandra_keyspace: String,
    /// Cassandra table name
    pub cassandra_table: String,
    /// Batch size for exports
    pub batch_size: usize,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            elasticsearch_url: None,
            elasticsearch_index: "dnsx-records".to_string(),
            mongodb_url: None,
            mongodb_database: "dnsx".to_string(),
            mongodb_collection: "records".to_string(),
            cassandra_contact_points: None,
            cassandra_username: None,
            cassandra_password: None,
            cassandra_keyspace: "dnsx".to_string(),
            cassandra_table: "records".to_string(),
            batch_size: DEFAULT_EXPORT_BATCH_SIZE,
        }
    }
}
