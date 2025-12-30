//! RDNSx Core - Fast and multi-purpose DNS toolkit library
//!
//! This library provides a high-performance DNS resolution engine with support for
//! multiple record types, wildcard filtering, subdomain enumeration, and database exports.

pub mod client;
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

pub use client::DnsxClient;
pub use config::{DnsxOptions, ExportConfig};
pub use error::{DnsxError, Result};
pub use types::{DnsRecord, RecordType, ResponseCode, RecordValue};
pub use export::{Exporter, ElasticsearchExporter, MongodbExporter};
pub use bruteforce::Bruteforcer;
pub use wildcard::WildcardFilter;
pub use resolver::ResolverPool;
