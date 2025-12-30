//! Wildcard DNS filtering

use std::collections::HashMap;
use std::sync::Arc;

use dashmap::DashMap;
use rand::Rng;
use tracing::debug;

use crate::error::{DnsxError, Result};
use crate::resolver::ResolverPool;
use hickory_resolver::proto::rr::RecordType as HRecordType;

use crate::types::DnsRecord;

/// Wildcard filter for DNS records
pub struct WildcardFilter {
    /// Wildcard patterns detected (domain -> is_wildcard)
    patterns: Arc<DashMap<String, bool>>,
    /// Domain for wildcard detection
    base_domain: Option<String>,
    /// Resolver pool for testing wildcards
    resolver_pool: Arc<ResolverPool>,
    /// Threshold for considering IP as wildcard (number of domains pointing to same IP)
    threshold: usize,
}

impl WildcardFilter {
    /// Create a new wildcard filter
    pub fn new(
        base_domain: Option<String>,
        resolver_pool: Arc<ResolverPool>,
        threshold: usize,
    ) -> Self {
        Self {
            patterns: Arc::new(DashMap::new()),
            base_domain,
            resolver_pool,
            threshold,
        }
    }

    /// Generate a random subdomain for testing wildcards
    fn random_subdomain(base: &str) -> String {
        let mut rng = rand::thread_rng();
        let random_str: String = (0..16)
            .map(|_| rng.gen_range(b'a'..=b'z') as char)
            .collect();
        format!("{}.{}", random_str, base)
    }

    /// Test if a domain level has wildcard DNS
    pub async fn test_wildcard(&self, domain: &str) -> Result<bool> {
        // Check cache first
        if let Some(&is_wildcard) = self.patterns.get(domain) {
            return Ok(is_wildcard);
        }

        // Test with a random subdomain that shouldn't exist
        let test_domain = Self::random_subdomain(domain);
        
        match self.resolver_pool.query(&test_domain, HRecordType::A).await {
            Ok(_) => {
                // Random domain resolved, likely a wildcard
                self.patterns.insert(domain.to_string(), true);
                debug!("Detected wildcard DNS for {}", domain);
                Ok(true)
            }
            Err(_) => {
                // Random domain didn't resolve, not a wildcard
                self.patterns.insert(domain.to_string(), false);
                Ok(false)
            }
        }
    }

    /// Check if a domain matches a wildcard pattern
    pub async fn is_wildcard(&self, domain: &str) -> Result<bool> {
        // Extract domain levels to test
        let parts: Vec<&str> = domain.split('.').collect();
        if parts.len() < 2 {
            return Ok(false);
        }

        // Test each level for wildcards
        for i in 1..parts.len() {
            let domain_to_test = parts[i..].join(".");
            if self.test_wildcard(&domain_to_test).await? {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Filter records by grouping by IP and detecting wildcards
    pub async fn filter_by_ip(&self, records: Vec<DnsRecord>) -> Result<Vec<DnsRecord>> {
        // Group records by IP address
        let mut ip_to_domains: HashMap<String, Vec<DnsRecord>> = HashMap::new();
        
        for record in records {
            if let crate::types::RecordValue::Ip(ip) = &record.value {
                let ip_str = ip.to_string();
                ip_to_domains.entry(ip_str).or_insert_with(Vec::new).push(record);
            } else {
                // Non-IP records, keep them with their value as key
                ip_to_domains
                    .entry(record.value.to_string())
                    .or_insert_with(Vec::new)
                    .push(record);
            }
        }

        let mut filtered = Vec::new();

        // Check each IP group for wildcard patterns
        for (ip, domain_records) in ip_to_domains {
            // If many domains point to same IP, likely a wildcard
            if domain_records.len() >= self.threshold {
                // Test if it's a wildcard
                let mut is_wildcard_ip = false;
                if let Some(first_record) = domain_records.first() {
                    match self.is_wildcard(&first_record.domain).await {
                        Ok(true) => is_wildcard_ip = true,
                        _ => {}
                    }
                }

                if !is_wildcard_ip {
                    filtered.extend(domain_records);
                } else {
                    debug!("Filtered {} wildcard records for IP {}", domain_records.len(), ip);
                }
            } else {
                // Small number of domains, likely legitimate
                filtered.extend(domain_records);
            }
        }

        Ok(filtered)
    }

    /// Filter records to remove wildcard matches
    pub async fn filter(&self, records: Vec<DnsRecord>) -> Result<Vec<DnsRecord>> {
        if self.base_domain.is_none() {
            // No base domain specified, skip wildcard filtering
            return Ok(records);
        }

        // Use IP-based filtering for efficiency
        self.filter_by_ip(records).await
    }
}
