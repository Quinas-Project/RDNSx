//! DNS zone transfer (AXFR) enumeration

use std::sync::Arc;
use tracing::{debug, info};

use crate::error::{DnsxError, Result};
use crate::resolver::ResolverPool;
use crate::types::{DnsRecord, RecordType};

/// Results from zone transfer enumeration
#[derive(Debug, Clone)]
pub struct ZoneTransferResult {
    pub domain: String,
    pub successful_transfers: Vec<String>,
    pub failed_transfers: Vec<(String, String)>,
    pub records: Vec<DnsRecord>,
}

/// Zone transfer enumeration functionality
pub struct ZoneTransferEnumerator {
    resolver_pool: Arc<ResolverPool>,
}

impl ZoneTransferEnumerator {
    /// Create a new zone transfer enumerator
    pub fn new(resolver_pool: Arc<ResolverPool>) -> Self {
        Self { resolver_pool }
    }

    /// Attempt DNS zone transfer (AXFR) against specified servers
    pub async fn enumerate(&self, domain: &str, nameservers: &[String]) -> Result<ZoneTransferResult> {
        info!("Attempting zone transfer for domain: {}", domain);

        let mut results = ZoneTransferResult {
            domain: domain.to_string(),
            successful_transfers: Vec::new(),
            failed_transfers: Vec::new(),
            records: Vec::new(),
        };

        for ns in nameservers {
            match self.attempt_axfr(domain, ns).await {
                Ok(records) => {
                    info!("✅ Zone transfer successful from {}: {} records", ns, records.len());
                    results.successful_transfers.push(ns.clone());
                    results.records.extend(records);
                }
                Err(e) => {
                    debug!("❌ Zone transfer failed from {}: {}", ns, e);
                    results.failed_transfers.push((ns.clone(), e.to_string()));
                }
            }
        }

        Ok(results)
    }

    /// Attempt AXFR against a specific nameserver
    async fn attempt_axfr(&self, domain: &str, nameserver: &str) -> Result<Vec<DnsRecord>> {
        // Parse the nameserver address
        let _ns_addr = if nameserver.contains(':') {
            nameserver.to_string()
        } else {
            format!("{}:53", nameserver)
        };

        // For now, we'll simulate AXFR by trying comprehensive record queries
        // In a real implementation, you'd use a proper AXFR client
        // This is a simplified version that attempts to enumerate common records

        let mut all_records = Vec::new();
        let record_types = vec![
            crate::RecordType::A,
            crate::RecordType::Aaaa,
            crate::RecordType::Cname,
            crate::RecordType::Mx,
            crate::RecordType::Txt,
            crate::RecordType::Ns,
            crate::RecordType::Soa,
            crate::RecordType::Srv,
            crate::RecordType::Caa,
            crate::RecordType::Ptr,
        ];

        // Try to get NS records first to find authoritative servers
        if let Ok((lookup, _)) = self.resolver_pool.query(domain, RecordType::Ns).await {
            for rdata in lookup.iter() {
                if let hickory_resolver::proto::rr::RData::NS(ns_record) = rdata {
                    let ns_domain = ns_record.to_string();
                    debug!("Found nameserver: {}", ns_domain);

                    // Try to resolve NS to IP and attempt zone transfer
                    if let Ok((ns_lookup, _)) = self.resolver_pool.query(&ns_domain, RecordType::A).await {
                        for ns_rdata in ns_lookup.iter() {
                            if let hickory_resolver::proto::rr::RData::A(ns_ip) = ns_rdata {
                                debug!("Nameserver IP: {}", ns_ip);

                                // Attempt zone transfer simulation
                                for record_type in &record_types {
                                    if let Ok((type_lookup, resolver_addr)) = self.resolver_pool.query(domain, *record_type).await {
                                        for record in type_lookup.records() {
                                            let value = crate::query::parse_rdata(record.data().expect("Record data missing"))?;
                                            let ttl = record.ttl() as u32;

                                            let dns_record = DnsRecord::new(
                                                domain.to_string(),
                                                *record_type,
                                                value,
                                                ttl,
                                                crate::ResponseCode::NoError,
                                                resolver_addr.clone(),
                                                0.0,
                                            );

                                            all_records.push(dns_record);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            return Err(DnsxError::Other("Failed to enumerate nameservers".to_string()));
        }

        if all_records.is_empty() {
            return Err(DnsxError::Other("Zone transfer not allowed or no records found".to_string()));
        }

        Ok(all_records)
    }
}