//! PTR command implementation

use std::sync::Arc;

use anyhow::Result;
use clap::Args;
use tokio;
use rdnsx_core::{DnsxClient, DnsEnumerator, RecordType, ResolverPool, parse_ip_range, reverse_ip};

use crate::cli::Config;
use crate::output_writer::OutputWriter;

#[derive(Args)]
pub struct PtrArgs {
    /// IP range (CIDR) or ASN (e.g., 173.0.84.0/24 or AS17012)
    #[arg()]
    pub input: String,
}

pub async fn run(args: PtrArgs, config: Config) -> Result<()> {
    // Create DNS client
    let dns_options = rdnsx_core::config::DnsxOptions {
        resolvers: config.core_config.resolvers.servers.clone(),
        timeout: std::time::Duration::from_secs(config.core_config.resolvers.timeout),
        retries: config.core_config.resolvers.retries,
        concurrency: config.core_config.performance.threads,
        rate_limit: config.core_config.performance.rate_limit,
    };
    let _client = DnsxClient::with_options(dns_options.clone())?;

    // Create output writer
    let mut output = OutputWriter::new(config.output_file.clone(), config.json_output, config.silent)?;

    // Parse input - check if it's an ASN or IP range
    let ips = if args.input.to_uppercase().starts_with("AS") {
        // ASN input - use ASN enumeration to get IP ranges
        if !config.silent {
            eprintln!("🔍 Enumerating ASN {} for IP ranges...", args.input);
        }

        // Create resolver pool and enumerator for ASN lookup
        let resolver_pool = Arc::new(ResolverPool::new(&dns_options)?);
        let enumerator = DnsEnumerator::new(resolver_pool);

        // Get ASN information
        let asn_result = enumerator.asn_enumeration(&args.input).await
            .map_err(|e| anyhow::anyhow!("Failed to enumerate ASN {}: {}", args.input, e))?;

        if !config.silent {
            eprintln!("📊 Found {} IPv4 prefixes and {} IPv6 prefixes for {}",
                     asn_result.ipv4_prefixes.len(),
                     asn_result.ipv6_prefixes.len(),
                     asn_result.asn);
        }

        // Convert prefixes to IP ranges, but limit to reasonable sizes
        let mut all_ips = Vec::new();
        let mut total_ips = 0u64;

        for prefix in &asn_result.ipv4_prefixes {
            match parse_ip_range(prefix) {
                Ok(range_ips) => {
                    // Limit each prefix to prevent excessive lookups
                    let max_per_prefix = 1000; // Limit to 1000 IPs per prefix
                    let limited_ips: Vec<_> = range_ips.into_iter().take(max_per_prefix).collect();
                    total_ips += limited_ips.len() as u64;
                    all_ips.extend(limited_ips);

                    if total_ips >= 10000 { // Overall limit of 10,000 IPs
                        if !config.silent {
                            eprintln!("⚠️  Limiting to 10,000 IPs total (ASN {} has many ranges)", args.input);
                        }
                        break;
                    }
                }
                Err(e) => {
                    if !config.silent {
                        eprintln!("⚠️  Skipping invalid prefix {}: {}", prefix, e);
                    }
                }
            }
        }

        if all_ips.is_empty() {
            anyhow::bail!("No valid IP addresses found for ASN {}", args.input);
        }

        all_ips
    } else if args.input.contains('/') {
        // CIDR notation - add size limits
        let range_ips = parse_ip_range(&args.input)
            .map_err(|e| anyhow::anyhow!("Failed to parse IP range: {}", e))?;

        // Limit large ranges to prevent excessive lookups
        let max_ips = 10000; // 10,000 IP limit for CIDR ranges
        if range_ips.len() > max_ips {
            if !config.silent {
                eprintln!("⚠️  Limiting {}/{} range to {} IPs (was {} total)",
                         args.input, args.input.split('/').nth(1).unwrap_or(""),
                         max_ips, range_ips.len());
            }
            range_ips.into_iter().take(max_ips).collect()
        } else {
            range_ips
        }
    } else {
        // Single IP address
        vec![args.input
            .parse()
            .map_err(|e| anyhow::anyhow!("Invalid IP address: {}", e))?]
    };

    if !config.silent {
        eprintln!("🔍 Performing PTR lookups for {} IP addresses...", ips.len());

        // Show warning for large ranges
        if ips.len() > 1000 {
            eprintln!("⚠️  Large IP range detected - this may take some time. Consider using smaller ranges for faster results.");
        }
    }

    // Use concurrent lookups for better performance
    let concurrency = std::cmp::min(config.core_config.performance.threads, 50); // Cap at 50 concurrent requests
    let semaphore = Arc::new(tokio::sync::Semaphore::new(concurrency));

    // Collect all successful records first
    let mut all_records = Vec::new();
    let mut success_count = 0;
    let mut error_count = 0;

    // Create tasks for concurrent processing
    let mut tasks = Vec::new();

    for ip in ips {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let ptr_domain = reverse_ip(&ip);
        let options = dns_options.clone(); // Clone the options to create new client

        let task = tokio::spawn(async move {
            // Create a new client for this task
            let client_result = match DnsxClient::with_options(options) {
                Ok(client) => client.query(&ptr_domain, RecordType::Ptr).await,
                Err(e) => Err(e),
            };
            drop(permit); // Release the permit
            (ip, ptr_domain, client_result)
        });

        tasks.push(task);
    }

    // Process results as they complete
    for task in tasks {
        match task.await {
            Ok((_ip, _ptr_domain, result)) => {
                match result {
                    Ok(records) if !records.is_empty() => {
                        success_count += 1;
                        all_records.extend(records);
                    }
                    Ok(_) => {
                        // No PTR records found - this is normal, don't count as error
                        error_count += 1;
                    }
                    Err(_) => {
                        error_count += 1;
                    }
                }
            }
            Err(e) => {
                if !config.silent {
                    eprintln!("Task error: {}", e);
                }
            }
        }
    }

    // Write all collected records
    for record in all_records {
        output.write_record(&record, false)?;
    }

    if !config.silent {
        eprintln!("✅ PTR enumeration completed:");
        eprintln!("   • Successful lookups: {}", success_count);
        eprintln!("   • Failed/no records: {}", error_count);
        eprintln!("   • Total processed: {}", success_count + error_count);
    }

    output.flush()?;
    Ok(())
}
