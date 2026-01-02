//! Configuration from CLI arguments

use anyhow::Result;
use std::time::Duration;

use rdnsx_core::config::{DnsxOptions, ExportConfig};
use crate::cli::Cli;

/// Configuration for RDNSx
pub struct Config {
    pub dns_options: DnsxOptions,
    pub export_config: ExportConfig,
    pub output_file: Option<String>,
    pub json_output: bool,
    pub silent: bool,
}

impl Config {
    pub fn from_cli(cli: &Cli) -> Result<Self> {
        // Parse resolvers
        let resolvers = if let Some(resolver_str) = &cli.resolvers {
            if resolver_str.contains(',') {
                resolver_str.split(',').map(|s| s.trim().to_string()).collect()
            } else if std::path::Path::new(resolver_str).exists() {
                // Read from file
                std::fs::read_to_string(resolver_str)?
                    .lines()
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            } else {
                vec![resolver_str.clone()]
            }
        } else {
            rdnsx_core::config::DEFAULT_RESOLVERS
                .iter()
                .map(|s| s.to_string())
                .collect()
        };

        let dns_options = DnsxOptions::default()
            .with_resolvers(resolvers)
            .with_timeout(Duration::from_secs(cli.timeout))
            .with_retries(cli.retries)
            .with_concurrency(cli.threads)
            .with_rate_limit(cli.rate_limit);

        // Parse Cassandra contact points
        let cassandra_contact_points = cli.cassandra.as_ref().map(|points| {
            points.split(',').map(|s| s.trim().to_string()).collect::<Vec<String>>()
        });

        let export_config = ExportConfig {
            elasticsearch_url: cli.elasticsearch.clone(),
            elasticsearch_index: cli.elasticsearch_index.clone(),
            mongodb_url: cli.mongodb.clone(),
            mongodb_database: cli.mongodb_database.clone(),
            mongodb_collection: cli.mongodb_collection.clone(),
            cassandra_contact_points,
            cassandra_username: cli.cassandra_username.clone(),
            cassandra_password: cli.cassandra_password.clone(),
            cassandra_keyspace: cli.cassandra_keyspace.clone(),
            cassandra_table: cli.cassandra_table.clone(),
            batch_size: cli.export_batch_size,
        };

        Ok(Self {
            dns_options,
            export_config,
            output_file: cli.output.clone(),
            json_output: cli.json,
            silent: cli.silent,
        })
    }
}
