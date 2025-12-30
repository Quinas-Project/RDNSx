//! Bruteforce command implementation

use std::sync::Arc;

use anyhow::Result;
use clap::Args;
use rdnsx_core::{DnsxClient, RecordType};
use rdnsx_core::bruteforce;

use crate::config::Config;
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
    #[arg(long)]
    pub record_type: Option<String>,
}

pub async fn run(args: BruteforceArgs, config: Config) -> Result<()> {
    // Create DNS client
    let client = Arc::new(DnsxClient::with_options(config.dns_options.clone())?);

    // Create bruteforcer
    let bruteforcer = bruteforce::Bruteforcer::new(client.clone(), config.dns_options.concurrency);

    // Create output writer
    let mut output = OutputWriter::new(config.output_file.clone(), config.json_output, config.silent)?;

    // Determine record type
    let record_type = if let Some(rt_str) = &args.record_type {
        match rt_str.to_uppercase().as_str() {
            "A" => RecordType::A,
            "AAAA" => RecordType::Aaaa,
            "CNAME" => RecordType::Cname,
            "MX" => RecordType::Mx,
            "TXT" => RecordType::Txt,
            "NS" => RecordType::Ns,
            "SOA" => RecordType::Soa,
            "PTR" => RecordType::Ptr,
            "SRV" => RecordType::Srv,
            _ => RecordType::A,
        }
    } else {
        RecordType::A
    };

    // Process each domain
    for domain in &args.domain {
        if !config.silent {
            eprintln!("Bruteforcing subdomains for {}...", domain);
        }

        // Enumerate subdomains
        match bruteforcer
            .enumerate_with_records(domain, &args.wordlist, &args.placeholder, record_type)
            .await
        {
            Ok(results) => {
                for (subdomain, records) in results {
                    for record in records {
                        output.write_record(&record, false)?;
                    }
                }
            }
            Err(e) => {
                if !config.silent {
                    eprintln!("Error bruteforcing {}: {}", domain, e);
                }
            }
        }
    }

    output.flush()?;
    Ok(())
}
