//! DNS enumeration command implementation

use std::sync::Arc;

use anyhow::Result;
use clap::Args;
use rdnsx_core::{DnsEnumerator, ResolverPool, config::DnsxOptions};

use crate::cli::Config;

/// DNS enumeration command arguments
#[derive(Args)]
pub struct EnumerateArgs {
    /// Enumeration technique to use
    #[arg(short, long, value_enum)]
    pub technique: EnumerationTechnique,

    /// Target domain or ASN for enumeration (use ASN format like AS15169 for ASN enumeration)
    #[arg(short, long)]
    pub target: String,

    /// Custom nameservers for enumeration (comma-separated)
    #[arg(long)]
    pub nameservers: Option<String>,

    /// Maximum concurrent enumeration tasks
    #[arg(long, default_value = "10")]
    pub concurrent: usize,

    /// Timeout for enumeration operations (seconds)
    #[arg(long, default_value = "30")]
    pub timeout: u64,
}

/// Enumeration techniques available
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum EnumerationTechnique {
    /// Attempt DNS zone transfer (AXFR)
    ZoneTransfer,
    /// Enumerate email security records (SPF, DMARC, DKIM)
    EmailSecurity,
    /// Detect CDN usage and configuration
    CdnDetection,
    /// Enumerate IPv6 deployment and addresses
    Ipv6Enumeration,
    /// Fingerprint DNS server capabilities
    ServerFingerprint,
    /// Enumerate DNSSEC configuration and security
    DnssecEnumeration,
    /// Perform DNSSEC zone walking (NSEC enumeration)
    DnssecZoneWalking,
    /// Analyze wildcard DNS configurations and bypass techniques
    WildcardAnalysis,
    /// Perform passive DNS enumeration using historical data
    PassiveDns,
    /// Enumerate ASN information and associated IP ranges
    AsnEnumeration,
    /// Comprehensive enumeration (all techniques)
    Comprehensive,
}

pub async fn run(args: EnumerateArgs, config: Config) -> Result<()> {
    // Create DNS options with custom settings
    let mut dns_options = DnsxOptions {
        resolvers: config.core_config.resolvers.servers.clone(),
        timeout: std::time::Duration::from_secs(args.timeout),
        retries: config.core_config.resolvers.retries,
        concurrency: args.concurrent,
        rate_limit: config.core_config.performance.rate_limit,
    };

    // Override nameservers if specified
    if let Some(nameservers) = &args.nameservers {
        dns_options.resolvers = nameservers
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
    }

    // Create resolver pool and enumerator
    let resolver_pool = Arc::new(ResolverPool::new(&dns_options)?);
    let enumerator = DnsEnumerator::new(resolver_pool.clone());

    match args.technique {
        EnumerationTechnique::ZoneTransfer => {
            perform_zone_transfer(&enumerator, &args.target, &dns_options.resolvers).await?;
        }
        EnumerationTechnique::EmailSecurity => {
            perform_email_security_enumeration(&enumerator, &args.target).await?;
        }
        EnumerationTechnique::CdnDetection => {
            perform_cdn_detection(&enumerator, &args.target).await?;
        }
        EnumerationTechnique::Ipv6Enumeration => {
            perform_ipv6_enumeration(&enumerator, &args.target).await?;
        }
        EnumerationTechnique::ServerFingerprint => {
            perform_server_fingerprinting(&enumerator, &args.target, &dns_options.resolvers).await?;
        }
        EnumerationTechnique::DnssecEnumeration => {
            perform_dnssec_enumeration(&enumerator, &args.target).await?;
        }
        EnumerationTechnique::DnssecZoneWalking => {
            perform_dnssec_zone_walking(&enumerator, &args.target).await?;
        }
        EnumerationTechnique::WildcardAnalysis => {
            perform_wildcard_analysis(&enumerator, &args.target).await?;
        }
        EnumerationTechnique::PassiveDns => {
            perform_passive_dns_enumeration(&enumerator, &args.target).await?;
        }
        EnumerationTechnique::AsnEnumeration => {
            perform_asn_enumeration(&enumerator, &args.target).await?;
        }
        EnumerationTechnique::Comprehensive => {
            perform_comprehensive_enumeration(&enumerator, &args.target, &dns_options.resolvers).await?;
        }
    }

    Ok(())
}

async fn perform_zone_transfer(
    enumerator: &DnsEnumerator,
    domain: &str,
    nameservers: &[String],
) -> Result<()> {
    println!("🔄 Attempting DNS zone transfer for: {}", domain);
    println!("📡 Using nameservers: {:?}", nameservers);
    println!();

    match enumerator.zone_transfer(domain, nameservers).await {
        Ok(result) => {
            println!("📊 Zone Transfer Results for {}", result.domain);
            println!("{}", "=".repeat(50));

            if !result.successful_transfers.is_empty() {
                println!("✅ Successful transfers from:");
                for ns in &result.successful_transfers {
                    println!("  • {}", ns);
                }
            }

            if !result.failed_transfers.is_empty() {
                println!("\n❌ Failed transfers:");
                for (ns, error) in &result.failed_transfers {
                    println!("  • {}: {}", ns, error);
                }
            }

            println!("\n📋 Discovered records: {}", result.records.len());

            if !result.records.is_empty() {
                println!("\n🔍 Record Summary:");
                let mut record_types = std::collections::HashMap::new();

                for record in &result.records {
                    *record_types.entry(record.record_type).or_insert(0) += 1;
                }

                for (record_type, count) in record_types {
                    println!("  • {:?}: {} records", record_type, count);
                }

                println!("\n📄 Detailed Records:");
                for record in result.records.iter().take(20) { // Show first 20 records
                    println!("  {}", record);
                }

                if result.records.len() > 20 {
                    println!("  ... and {} more records", result.records.len() - 20);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Zone transfer failed: {}", e);
        }
    }

    Ok(())
}

async fn perform_email_security_enumeration(
    enumerator: &DnsEnumerator,
    domain: &str,
) -> Result<()> {
    println!("🔐 Enumerating email security for: {}", domain);
    println!();

    match enumerator.email_security_enumeration(domain).await {
        Ok(result) => {
            println!("📧 Email Security Analysis for {}", result.domain);
            println!("{}", "=".repeat(50));

            if !result.spf_records.is_empty() {
                println!("\n📋 SPF Records:");
                for (i, spf) in result.spf_records.iter().enumerate() {
                    println!("  {}. {}", i + 1, spf.content);
                    println!("     (via: {})", spf.resolver);
                }
            } else {
                println!("\n❌ No SPF records found");
            }

            if let Some(dmarc) = &result.dmarc_record {
                println!("\n🔒 DMARC Record:");
                println!("  {}", dmarc.content);
                println!("  (via: {})", dmarc.resolver);
            } else {
                println!("\n❌ No DMARC record found");
            }

            if !result.dkim_selectors.is_empty() {
                println!("\n🔑 DKIM Selectors:");
                for dkim in &result.dkim_selectors {
                    println!("  • {}: {}", dkim.selector, dkim.record);
                }
            } else {
                println!("\n❌ No DKIM selectors found");
            }

            // Provide security recommendations
            println!("\n💡 Security Recommendations:");

            if result.spf_records.is_empty() {
                println!("  • Add SPF record to prevent email spoofing");
            }

            if result.dmarc_record.is_none() {
                println!("  • Add DMARC record for email authentication");
            }

            if result.dkim_selectors.is_empty() {
                println!("  • Configure DKIM for email signing");
            }
        }
        Err(e) => {
            eprintln!("❌ Email security enumeration failed: {}", e);
        }
    }

    Ok(())
}

async fn perform_cdn_detection(
    enumerator: &DnsEnumerator,
    domain: &str,
) -> Result<()> {
    println!("🌐 Detecting CDN usage for: {}", domain);
    println!();

    match enumerator.cdn_detection(domain).await {
        Ok(result) => {
            println!("🚀 CDN Detection Results for {}", result.domain);
            println!("{}", "=".repeat(50));

            if let Some(provider) = &result.cdn_provider {
                println!("✅ CDN Provider Detected: {}", provider);
            } else {
                println!("❌ No CDN provider detected");
            }

            if !result.cname_chain.is_empty() {
                println!("\n🔗 CNAME Chain:");
                for (i, hop) in result.cname_chain.iter().enumerate() {
                    println!("  {}. {} → {}", i + 1, hop.from, hop.to);
                }
            }

            if let Some(ip) = result.origin_ip {
                println!("\n🏠 Origin IP: {}", ip);
            }

            // Provide insights
            if result.cdn_provider.is_some() {
                println!("\n💡 Insights:");
                println!("  • Traffic is likely served through a CDN");
                println!("  • Origin server may be protected from direct access");
                println!("  • Consider CDN-specific enumeration techniques");
            }
        }
        Err(e) => {
            eprintln!("❌ CDN detection failed: {}", e);
        }
    }

    Ok(())
}

async fn perform_ipv6_enumeration(
    enumerator: &DnsEnumerator,
    domain: &str,
) -> Result<()> {
    println!("🌐 Enumerating IPv6 deployment for: {}", domain);
    println!();

    match enumerator.ipv6_enumeration(domain).await {
        Ok(result) => {
            println!("🌐 IPv6 Enumeration Results for {}", result.domain);
            println!("{}", "=".repeat(50));

            if !result.ipv4_addresses.is_empty() {
                println!("\n🔢 IPv4 Addresses:");
                for ip in &result.ipv4_addresses {
                    println!("  • {}", ip);
                }
            }

            if !result.ipv6_addresses.is_empty() {
                println!("\n🔢 IPv6 Addresses:");
                for ip in &result.ipv6_addresses {
                    println!("  • {}", ip);
                }
            }

            println!("\n📊 Deployment Analysis:");
            println!("  • IPv4 addresses: {}", result.ipv4_addresses.len());
            println!("  • IPv6 addresses: {}", result.ipv6_addresses.len());

            if result.dual_stack {
                println!("  • 🌐 Dual-stack deployment (IPv4 + IPv6)");
            } else if result.ipv6_only {
                println!("  • 🆕 IPv6-only deployment");
            } else {
                println!("  • 📡 IPv4-only deployment");
            }

            if result.ipv6_addresses.is_empty() {
                println!("\n💡 Recommendation: Consider enabling IPv6 for better connectivity");
            }
        }
        Err(e) => {
            eprintln!("❌ IPv6 enumeration failed: {}", e);
        }
    }

    Ok(())
}

async fn perform_server_fingerprinting(
    enumerator: &DnsEnumerator,
    domain: &str,
    nameservers: &[String],
) -> Result<()> {
    println!("🔍 Fingerprinting DNS servers for: {}", domain);
    println!();

    for ns in nameservers {
        match enumerator.server_fingerprinting(ns).await {
            Ok(fingerprint) => {
                println!("🖥️  DNS Server: {}", fingerprint.server);
                println!("   Response time: {}ms", fingerprint.response_time_ms);
                println!("   Recursion: {}", if fingerprint.recursion_available { "✅" } else { "❌" });
                println!("   DNSSEC: {}", if fingerprint.dnssec_support { "✅" } else { "❌" });
                println!("   EDNS: {}", if fingerprint.edns_support { "✅" } else { "❌" });

                if let Some(version) = &fingerprint.version_bind {
                    println!("   Version: {}", version);
                }
                println!();
            }
            Err(e) => {
                eprintln!("❌ Failed to fingerprint {}: {}", ns, e);
            }
        }
    }

    Ok(())
}

async fn perform_dnssec_enumeration(
    enumerator: &DnsEnumerator,
    domain: &str,
) -> Result<()> {
    println!("🔒 Enumerating DNSSEC configuration for: {}", domain);
    println!();

    match enumerator.dnssec_enumeration(domain).await {
        Ok(result) => {
            println!("🔐 DNSSEC Analysis for {}", result.domain);
            println!("{}", "=".repeat(50));

            println!("DNSSEC Status: {}", if result.dnssec_enabled { "✅ Enabled" } else { "❌ Disabled" });

            if result.dnssec_enabled {
                if !result.dnskey_records.is_empty() {
                    println!("\n🔑 DNSKEY Records:");
                    for dnskey in &result.dnskey_records {
                        println!("  • Key Tag: {}, Algorithm: {}, Flags: {}", dnskey.key_tag, dnskey.algorithm, dnskey.flags);
                    }
                }

                if !result.ds_records.is_empty() {
                    println!("\n📋 DS Records:");
                    for ds in &result.ds_records {
                        println!("  • Key Tag: {}, Algorithm: {}, Digest Type: {}", ds.key_tag, ds.algorithm, ds.digest_type);
                        println!("    Digest: {}", &ds.digest[..16]); // Show first 16 chars
                    }
                }

                println!("\n📊 Record Counts:");
                println!("  • RRSIG records: {}", result.rrsig_records);
                println!("  • NSEC records: {}", result.nsec_records);
                println!("  • NSEC3 records: {}", result.nsec3_records);

                if !result.security_issues.is_empty() {
                    println!("\n⚠️  Security Issues:");
                    for issue in &result.security_issues {
                        println!("  • {}", issue);
                    }
                } else {
                    println!("\n✅ No security issues detected");
                }
            } else {
                println!("\n💡 Recommendation: Enable DNSSEC for enhanced security");
            }
        }
        Err(e) => {
            eprintln!("❌ DNSSEC enumeration failed: {}", e);
        }
    }

    Ok(())
}

async fn perform_dnssec_zone_walking(
    enumerator: &DnsEnumerator,
    domain: &str,
) -> Result<()> {
    println!("🚶 Performing DNSSEC zone walking for: {}", domain);
    println!();

    match enumerator.dnssec_zone_walking(domain).await {
        Ok(result) => {
            println!("🚶 DNSSEC Zone Walking Results for {}", result.domain);
            println!("{}", "=".repeat(50));

            println!("Enumeration Status: {}", if result.enumeration_successful { "✅ Successful" } else { "❌ Failed" });

            if !result.nsec_chain.is_empty() {
                println!("\n🔗 NSEC Chain:");
                for nsec in &result.nsec_chain {
                    println!("  {} → {}", nsec.owner, nsec.next_domain);
                }
            }

            if !result.discovered_names.is_empty() {
                println!("\n🔍 Discovered Names:");
                for name in &result.discovered_names {
                    println!("  • {}", name);
                }
            }

            if result.enumeration_successful {
                println!("\n💡 Zone walking successful - DNSSEC NSEC records can be enumerated");
            } else {
                println!("\n💡 Zone walking not possible - Domain may not use DNSSEC or NSEC");
            }
        }
        Err(e) => {
            eprintln!("❌ DNSSEC zone walking failed: {}", e);
        }
    }

    Ok(())
}

async fn perform_wildcard_analysis(
    enumerator: &DnsEnumerator,
    domain: &str,
) -> Result<()> {
    println!("🎭 Analyzing wildcard DNS configuration for: {}", domain);
    println!();

    match enumerator.wildcard_analysis(domain).await {
        Ok(analysis) => {
            println!("🎭 Wildcard Analysis Results for {}", analysis.domain);
            println!("{}", "=".repeat(50));

            println!("Wildcard Status: {}", if analysis.has_wildcard { "✅ Detected" } else { "❌ Not Detected" });
            println!("Confidence Score: {:.1}%", analysis.confidence_score * 100.0);

            if !analysis.wildcard_ips.is_empty() {
                println!("\n🌐 Wildcard IP Addresses:");
                for ip in &analysis.wildcard_ips {
                    println!("  • {}", ip);
                }
            }

            if !analysis.wildcard_records.is_empty() {
                println!("\n📋 Sample Wildcard Records:");
                for record in analysis.wildcard_records.iter().take(3) {
                    println!("  • {}", record.domain);
                }
                if analysis.wildcard_records.len() > 3 {
                    println!("  ... and {} more test records", analysis.wildcard_records.len() - 3);
                }
            }

            if !analysis.bypass_attempts.is_empty() {
                println!("\n🛡️ Wildcard Bypass Attempts:");
                let successful_bypasses: Vec<_> = analysis.bypass_attempts.iter()
                    .filter(|attempt| attempt.success)
                    .collect();

                if !successful_bypasses.is_empty() {
                    println!("  ✅ Successful bypass techniques:");
                    for attempt in &successful_bypasses {
                        println!("    • {}: {}", attempt.technique, attempt.test_domain);
                    }
                } else {
                    println!("  ❌ No bypass techniques successful");
                }

                let failed_count = analysis.bypass_attempts.len() - successful_bypasses.len();
                if failed_count > 0 {
                    println!("  📊 {} bypass attempts failed", failed_count);
                }
            }

            // Provide recommendations
            println!("\n💡 Recommendations:");
            if analysis.has_wildcard {
                println!("  • Wildcard DNS is active - consider targeted subdomain enumeration");
                if analysis.confidence_score > 0.8 {
                    println!("  • High confidence wildcard detection - most subdomains will resolve");
                }
                if !analysis.bypass_attempts.is_empty() {
                    println!("  • Some bypass techniques work - can find non-wildcard domains");
                }
            } else {
                println!("  • No wildcard DNS detected - standard enumeration should work well");
            }
        }
        Err(e) => {
            eprintln!("❌ Wildcard analysis failed: {}", e);
        }
    }

    Ok(())
}

async fn perform_passive_dns_enumeration(
    enumerator: &DnsEnumerator,
    domain: &str,
) -> Result<()> {
    println!("📚 Performing passive DNS enumeration for: {}", domain);
    println!("Note: This is a basic implementation. For production use, integrate with passive DNS services.");
    println!();

    match enumerator.passive_dns_enumeration(domain).await {
        Ok(result) => {
            println!("📚 Passive DNS Results for {}", result.domain);
            println!("{}", "=".repeat(50));

            if let Some(last_seen) = result.last_seen {
                println!("Last Seen: {}", last_seen.format("%Y-%m-%d %H:%M:%S UTC"));
            }

            if !result.subdomains.is_empty() {
                println!("\n🔍 Discovered Subdomains:");
                for subdomain in &result.subdomains {
                    println!("  • {} ({}) - First: {}, Last: {}",
                            subdomain.name,
                            subdomain.record_type,
                            subdomain.first_seen.format("%Y-%m-%d"),
                            subdomain.last_seen.format("%Y-%m-%d"));
                }
            } else {
                println!("\n❌ No subdomains discovered");
            }

            if !result.historical_ips.is_empty() {
                println!("\n🏠 Historical IP Addresses:");
                for historical_ip in &result.historical_ips {
                    println!("  • {} - First: {}, Last: {}",
                            historical_ip.ip,
                            historical_ip.first_seen.format("%Y-%m-%d"),
                            historical_ip.last_seen.format("%Y-%m-%d"));
                }
            }

            if !result.data_sources.is_empty() {
                println!("\n📊 Data Sources:");
                for source in &result.data_sources {
                    println!("  • {}", source);
                }
            }

            println!("\n💡 Note: This is a basic active resolution check.");
            println!("   For comprehensive passive DNS, integrate with services like:");
            println!("   • CIRCL Passive DNS");
            println!("   • PassiveTotal");
            println!("   • RiskIQ");
            println!("   • VirusTotal Passive DNS");
        }
        Err(e) => {
            eprintln!("❌ Passive DNS enumeration failed: {}", e);
        }
    }

    Ok(())
}

async fn perform_asn_enumeration(
    enumerator: &DnsEnumerator,
    asn: &str,
) -> Result<()> {
    println!("🏢 Enumerating ASN information for: {}", asn);
    println!();

    match enumerator.asn_enumeration(asn).await {
        Ok(result) => {
            println!("🏢 ASN Enumeration Results for {}", result.asn);
            println!("{}", "=".repeat(50));

            if let Some(name) = &result.name {
                println!("🏷️  Name: {}", name);
            }

            if let Some(description) = &result.description {
                println!("📝 Description: {}", description);
            }

            if let Some(country) = &result.country {
                println!("🌍 Country: {}", country);
            }

            println!("\n📊 Network Summary:");
            println!("  • IPv4 prefixes: {}", result.ipv4_prefixes.len());
            println!("  • IPv6 prefixes: {}", result.ipv6_prefixes.len());
            println!("  • Total IPv4 addresses: {}", result.total_ipv4_addresses);
            println!("  • Total IPv6 addresses: {}", result.total_ipv6_addresses);

            if !result.ipv4_prefixes.is_empty() {
                println!("\n🔢 IPv4 Prefixes:");
                for prefix in result.ipv4_prefixes.iter().take(10) { // Show first 10
                    println!("  • {}", prefix);
                }
                if result.ipv4_prefixes.len() > 10 {
                    println!("  ... and {} more IPv4 prefixes", result.ipv4_prefixes.len() - 10);
                }
            }

            if !result.ipv6_prefixes.is_empty() {
                println!("\n🔢 IPv6 Prefixes:");
                for prefix in result.ipv6_prefixes.iter().take(10) { // Show first 10
                    println!("  • {}", prefix);
                }
                if result.ipv6_prefixes.len() > 10 {
                    println!("  ... and {} more IPv6 prefixes", result.ipv6_prefixes.len() - 10);
                }
            }

            // Provide recommendations
            println!("\n💡 Usage Recommendations:");
            println!("  • Use these IP ranges with PTR enumeration: rdnsx ptr <prefix>");
            println!("  • Combine with subdomain enumeration for comprehensive reconnaissance");
            if result.total_ipv4_addresses > 1000000 {
                println!("  • Large ASN - consider rate limiting for PTR enumeration");
            }
        }
        Err(e) => {
            eprintln!("❌ ASN enumeration failed: {}", e);
            eprintln!("\n💡 Troubleshooting:");
            eprintln!("  • Ensure ASN format is correct (e.g., AS15169 or 15169)");
            eprintln!("  • Currently running in offline mode with limited ASN data");
            eprintln!("  • For full online ASN enumeration, network connectivity is required");
            eprintln!("  • Known ASNs (Google, Amazon, Cloudflare) have detailed information available");
        }
    }

    Ok(())
}

async fn perform_comprehensive_enumeration(
    enumerator: &DnsEnumerator,
    domain: &str,
    nameservers: &[String],
) -> Result<()> {
    println!("🔬 Performing comprehensive DNS enumeration for: {}", domain);
    println!("{}", "=".repeat(60));
    println!();

    // Zone Transfer
    if let Err(e) = perform_zone_transfer(enumerator, domain, nameservers).await {
        eprintln!("Zone transfer enumeration failed: {}", e);
    }

    println!("\n{}\n", "=".repeat(60));

    // DNSSEC Enumeration
    if let Err(e) = perform_dnssec_enumeration(enumerator, domain).await {
        eprintln!("DNSSEC enumeration failed: {}", e);
    }

    println!("\n{}\n", "=".repeat(60));

    // Wildcard Analysis
    if let Err(e) = perform_wildcard_analysis(enumerator, domain).await {
        eprintln!("Wildcard analysis failed: {}", e);
    }

    println!("\n{}\n", "=".repeat(60));

    // Passive DNS Enumeration
    if let Err(e) = perform_passive_dns_enumeration(enumerator, domain).await {
        eprintln!("Passive DNS enumeration failed: {}", e);
    }

    println!("\n{}\n", "=".repeat(60));

    // Email Security
    if let Err(e) = perform_email_security_enumeration(enumerator, domain).await {
        eprintln!("Email security enumeration failed: {}", e);
    }

    println!("\n{}\n", "=".repeat(60));

    // CDN Detection
    if let Err(e) = perform_cdn_detection(enumerator, domain).await {
        eprintln!("CDN detection failed: {}", e);
    }

    println!("\n{}\n", "=".repeat(60));

    // IPv6 Enumeration
    if let Err(e) = perform_ipv6_enumeration(enumerator, domain).await {
        eprintln!("IPv6 enumeration failed: {}", e);
    }

    println!("\n{}\n", "=".repeat(60));

    // Server Fingerprinting
    if let Err(e) = perform_server_fingerprinting(enumerator, domain, nameservers).await {
        eprintln!("Server fingerprinting failed: {}", e);
    }

    println!("🎉 Comprehensive enumeration completed!");

    Ok(())
}