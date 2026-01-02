//! Bruteforce command implementation

use std::sync::Arc;

use anyhow::Result;
use clap::Args;
use rdnsx_core::{DnsxClient, RecordType};

use crate::cli::Config;
use crate::output_writer::OutputWriter;

#[derive(Args)]
pub struct BruteforceArgs {
    /// Target domain(s)
    #[arg(short, long, required = true)]
    pub domain: Vec<String>,

    /// Wordlist file or comma-separated words (use - for stdin)
    #[arg(short, long)]
    pub wordlist: String,

    /// Placeholder string (default: FUZZ)
    #[arg(long, default_value = "FUZZ")]
    pub placeholder: String,

    /// Record type to query (default: A)
    #[arg(long, default_value = "A")]
    pub record_type: String,
}

pub async fn run(args: BruteforceArgs, config: Config) -> Result<()> {
    eprintln!("DEBUG: Starting bruteforce command");
    // Parse record type
    let record_type = match args.record_type.to_uppercase().as_str() {
        "A" => RecordType::A,
        "AAAA" => RecordType::Aaaa,
        "CNAME" => RecordType::Cname,
        "MX" => RecordType::Mx,
        "TXT" => RecordType::Txt,
        "NS" => RecordType::Ns,
        "SOA" => RecordType::Soa,
        "PTR" => RecordType::Ptr,
        "SRV" => RecordType::Srv,
        "CAA" => RecordType::Caa,
        _ => {
            eprintln!("Unsupported record type: {}", args.record_type);
            std::process::exit(1);
        }
    };

    // Create DNS client
    let dns_options = rdnsx_core::config::DnsxOptions {
        resolvers: config.core_config.resolvers.servers.clone(),
        timeout: std::time::Duration::from_secs(config.core_config.resolvers.timeout),
        retries: config.core_config.resolvers.retries,
        concurrency: config.core_config.performance.threads,
        rate_limit: config.core_config.performance.rate_limit,
    };
    let client = Arc::new(DnsxClient::with_options(dns_options)?);

    // Create output writer
    let mut output = OutputWriter::new(config.output_file.clone(), config.json_output, config.silent)?;

    // Process each domain
    for domain in &args.domain {
        if !config.silent {
            eprintln!("Enumerating subdomains for {}", domain);
        }

        // Simple test: just try www.{domain}
        let test_subdomain = format!("www.{}", domain);

        match client.query(&test_subdomain, record_type).await {
            Ok(records) => {
                if !records.is_empty() {
                    if !config.silent {
                        eprintln!("Found subdomain: {} with {} records", test_subdomain, records.len());
                    }
                    for record in records {
                        output.write_record(&record, false)?;
                    }
                } else {
                    if !config.silent {
                        eprintln!("No records found for: {}", test_subdomain);
                    }
                }
            }
            Err(e) => {
                if !config.silent {
                    eprintln!("Error querying {}: {}", test_subdomain, e);
                }
            }
        }
    }

    output.flush()?;
    eprintln!("DEBUG: Finished bruteforce command");
    Ok(())
}
