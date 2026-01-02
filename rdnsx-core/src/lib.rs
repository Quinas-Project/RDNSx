//! RDNSx Core - Fast and multi-purpose DNS toolkit library
//!
//! This library provides a high-performance DNS resolution engine with support for
//! multiple record types, wildcard filtering, subdomain enumeration, and database exports.

pub mod cache;
pub mod client;
pub mod concurrency;
pub mod config;
pub mod error;
pub mod export;
pub mod input;
pub mod output;
pub mod query;
pub mod resolver;
pub mod types;
pub mod utils;
pub mod wildcard;
pub mod bruteforce;

pub use cache::{DnsCache, CachedDnsClient, CacheStats, DnsQuery};
pub use client::DnsxClient;
pub use concurrency::{ConcurrentProcessor, ConcurrencyConfig, ProcessingMetrics, DomainStreamer, AdaptiveBatchSizer, RateLimiter};
pub use config::{DnsxOptions, ExportConfig, DEFAULT_RESOLVERS};
pub use error::{DnsxError, Result};
pub use types::{DnsRecord, RecordType, ResponseCode, RecordValue};
pub use export::{Exporter, CassandraExporter, CassandraConfig, CassandraMetrics, ElasticsearchExporter, MongodbExporter};
pub use bruteforce::Bruteforcer;
pub use wildcard::WildcardFilter;
pub use resolver::ResolverPool;
pub use input::{parse_asn, parse_ip_range, reverse_ip};

#[cfg(test)]
mod tests;
