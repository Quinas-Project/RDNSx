//! CLI argument parsing

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commands::{bruteforce, ptr, query};
use crate::config::Config;

#[derive(Parser)]
#[command(name = "rdnsx")]
#[command(about = "Fast and multi-purpose DNS toolkit", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Custom resolver list (file or comma-separated)
    #[arg(short, long, global = true)]
    pub resolvers: Option<String>,

    /// Concurrency level
    #[arg(short, long, global = true, default_value_t = 100)]
    pub threads: usize,

    /// Output file
    #[arg(short, long, global = true)]
    pub output: Option<String>,

    /// JSON output format
    #[arg(long, global = true)]
    pub json: bool,

    /// Silent mode (minimal output)
    #[arg(long, global = true)]
    pub silent: bool,

    /// Query timeout in seconds
    #[arg(long, global = true, default_value_t = 5)]
    pub timeout: u64,

    /// Retry attempts
    #[arg(long, global = true, default_value_t = 3)]
    pub retries: u32,

    /// Rate limit (queries per second, 0 = unlimited)
    #[arg(long, global = true, default_value_t = 0)]
    pub rate_limit: u64,

    /// Elasticsearch connection string
    #[arg(long, global = true)]
    pub elasticsearch: Option<String>,

    /// Elasticsearch index name
    #[arg(long, global = true, default_value = "dnsx-records")]
    pub elasticsearch_index: String,

    /// MongoDB connection string
    #[arg(long, global = true)]
    pub mongodb: Option<String>,

    /// MongoDB database name
    #[arg(long, global = true, default_value = "dnsx")]
    pub mongodb_database: String,

    /// MongoDB collection name
    #[arg(long, global = true, default_value = "records")]
    pub mongodb_collection: String,

    /// Cassandra contact points (comma-separated)
    #[arg(long, global = true)]
    pub cassandra: Option<String>,

    /// Cassandra username
    #[arg(long, global = true)]
    pub cassandra_username: Option<String>,

    /// Cassandra password
    #[arg(long, global = true)]
    pub cassandra_password: Option<String>,

    /// Cassandra keyspace name
    #[arg(long, global = true, default_value = "dnsx")]
    pub cassandra_keyspace: String,

    /// Cassandra table name
    #[arg(long, global = true, default_value = "records")]
    pub cassandra_table: String,

    /// Batch size for database exports
    #[arg(long, global = true, default_value_t = 1000)]
    pub export_batch_size: usize,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Query domains from list/stdin
    Query(query::QueryArgs),
    /// Enumerate subdomains (bruteforce)
    Bruteforce(bruteforce::BruteforceArgs),
    /// Reverse DNS lookups (IP ranges, ASN)
    Ptr(ptr::PtrArgs),
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        let config = Config::from_cli(&self)?;

        match self.command {
            Commands::Query(args) => query::run(args, config).await,
            Commands::Bruteforce(args) => bruteforce::run(args, config).await,
            Commands::Ptr(args) => ptr::run(args, config).await,
        }
    }
}
