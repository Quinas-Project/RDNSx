//! PTR command implementation

use anyhow::Result;
use clap::Args;
use rdnsx_core::{DnsxClient, RecordType};
use rdnsx_core::input::{parse_asn, parse_ip_range, reverse_ip};

use crate::config::Config;
use crate::output_writer::OutputWriter;

#[derive(Args)]
pub struct PtrArgs {
    /// IP range (CIDR) or ASN (e.g., 173.0.84.0/24 or AS17012)
    #[arg()]
    pub input: String,
}

pub async fn run(args: PtrArgs, config: Config) -> Result<()> {
    // Create DNS client
    let client = DnsxClient::with_options(config.dns_options.clone())?;

    // Create output writer
    let mut output = OutputWriter::new(config.output_file.clone(), config.json_output, config.silent)?;

    // Parse input - check if it's an ASN or IP range
    let ips = if args.input.to_uppercase().starts_with("AS") {
        // ASN input - for now, return error as ASN enumeration requires external API
        // In a full implementation, this would query an ASN database
        anyhow::bail!("ASN-based PTR enumeration requires external ASN database access (not yet implemented)")
    } else if args.input.contains('/') {
        // CIDR notation
        parse_ip_range(&args.input)
            .map_err(|e| anyhow::anyhow!("Failed to parse IP range: {}", e))?
    } else {
        // Single IP address
        vec![args.input
            .parse()
            .map_err(|e| anyhow::anyhow!("Invalid IP address: {}", e))?]
    };

    if !config.silent {
        eprintln!("Performing PTR lookups for {} IP addresses...", ips.len());
    }

    // Perform PTR lookups
    for ip in ips {
        let ptr_domain = reverse_ip(&ip);
        match client.query(&ptr_domain, RecordType::Ptr).await {
            Ok(records) => {
                for record in records {
                    output.write_record(&record, false)?;
                }
            }
            Err(e) => {
                if !config.silent {
                    eprintln!("Error querying PTR for {}: {}", ip, e);
                }
            }
        }
    }

    output.flush()?;
    Ok(())
}
