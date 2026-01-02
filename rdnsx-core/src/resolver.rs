//! DNS resolver pool implementation

use std::sync::Arc;
use std::time::Duration;

use hickory_resolver::config::{NameServerConfigGroup, ResolverConfig, ResolverOpts};
use hickory_resolver::error::ResolveError;
use hickory_resolver::proto::rr::{RData, RecordType};
use hickory_resolver::proto::xfer::DnsResponse;
use hickory_resolver::TokioAsyncResolver;
use tokio::sync::Semaphore;
use tracing::{debug, trace, warn};

use crate::config::DnsxOptions;
use crate::error::{DnsxError, Result};
use crate::utils;

/// DNS resolver pool
pub struct ResolverPool {
    /// Primary resolver
    resolver: TokioAsyncResolver,
    /// Primary resolver address
    primary_resolver_addr: String,
    /// Backup resolvers
    backup_resolvers: Vec<TokioAsyncResolver>,
    /// Backup resolver addresses
    backup_resolver_addrs: Vec<String>,
    /// Concurrency semaphore
    semaphore: Arc<Semaphore>,
    /// Query timeout
    timeout: Duration,
    /// Number of retries
    retries: u32,
}

impl ResolverPool {
    /// Create a new resolver pool
    pub fn new(options: &DnsxOptions) -> Result<Self> {
        let resolvers = if options.resolvers.is_empty() {
            return Err(DnsxError::validation("At least one resolver is required"));
        } else {
            options.resolvers.clone()
        };

        // Parse and validate resolvers
        let mut resolver_configs = Vec::new();
        for resolver_str in &resolvers {
            let addr = utils::parse_resolver(resolver_str)?;
            resolver_configs.push(addr);
        }

        // Store primary resolver address
        let primary_resolver_addr = resolver_configs[0].clone();

        // Create primary resolver
        let primary_config = create_resolver_config(&resolver_configs[0..1])?;
        let mut resolver_opts = ResolverOpts::default();
        resolver_opts.timeout = options.timeout;
        resolver_opts.attempts = options.retries as usize;
        resolver_opts.validate = false; // Don't validate, just resolve

        let resolver = TokioAsyncResolver::tokio(primary_config, resolver_opts.clone());

        // Create backup resolvers if any
        let mut backup_resolvers = Vec::new();
        let mut backup_resolver_addrs = Vec::new();
        if resolver_configs.len() > 1 {
            for config in &resolver_configs[1..] {
                let backup_config = create_resolver_config(&[config.clone()])?;
                backup_resolvers.push(TokioAsyncResolver::tokio(
                    backup_config,
                    resolver_opts.clone(),
                ));
                backup_resolver_addrs.push(config.clone());
            }
        }

        Ok(Self {
            resolver,
            primary_resolver_addr,
            backup_resolvers,
            backup_resolver_addrs,
            semaphore: Arc::new(Semaphore::new(options.concurrency)),
            timeout: options.timeout,
            retries: options.retries,
        })
    }

    /// Query DNS with a specific record type
    pub async fn query(
        &self,
        domain: &str,
        record_type: HRecordType,
    ) -> Result<(DnsResponse, String)> {
        let _permit = self.semaphore.acquire().await.map_err(|e| {
            DnsxError::Other(format!("Failed to acquire semaphore: {}", e))
        })?;

        let domain_name = domain
            .parse()
            .map_err(|e| DnsxError::invalid_input(format!("Invalid domain name: {}", e)))?;

        // Try primary resolver first
        let result = tokio::time::timeout(self.timeout, self.resolver.query(domain_name.clone(), record_type))
            .await;

        match result {
            Ok(Ok(response)) => {
                trace!("Query successful for {} ({})", domain, record_type);
                Ok((response, self.primary_resolver_addr.clone()))
            }
            Ok(Err(e)) => {
                debug!("Primary resolver failed for {}: {}", domain, e);
                // Try backup resolvers
                self.try_backup_resolvers(&domain_name, record_type).await
            }
            Err(_) => {
                warn!("Query timeout for {} ({})", domain, record_type);
                Err(DnsxError::timeout(self.timeout))
            }
        }
    }

    /// Get primary resolver address
    pub fn primary_resolver(&self) -> &str {
        &self.primary_resolver_addr
    }

    /// Try backup resolvers if primary fails
    async fn try_backup_resolvers(
        &self,
        domain_name: &hickory_resolver::proto::rr::Name,
        record_type: RecordType,
    ) -> Result<DnsResponse> {
        for backup in &self.backup_resolvers {
            let result = tokio::time::timeout(self.timeout, backup.query(*domain_name, record_type))
                .await;
            match result {
                Ok(Ok(response)) => {
                    trace!("Backup resolver succeeded");
                    return Ok(response);
                }
                Ok(Err(e)) => {
                    debug!("Backup resolver failed: {}", e);
                }
                Err(_) => {
                    debug!("Backup resolver timeout");
                }
            }
        }

        Err(DnsxError::resolve("All resolvers failed"))
    }

    /// Lookup A records (IPv4)
    pub async fn lookup_ipv4(&self, domain: &str) -> Result<Vec<std::net::Ipv4Addr>> {
        let (response, _) = self.query(domain, HRecordType::A).await?;
        let mut ips = Vec::new();

        for record in response.records() {
            if let Some(RData::A(ipv4)) = record.data() {
                ips.push(*ipv4);
            }
        }

        Ok(ips)
    }

    /// Lookup AAAA records (IPv6)
    pub async fn lookup_ipv6(&self, domain: &str) -> Result<Vec<std::net::Ipv6Addr>> {
        let (response, _) = self.query(domain, HRecordType::AAAA).await?;
        let mut ips = Vec::new();

        for record in response.records() {
            if let Some(RData::AAAA(ipv6)) = record.data() {
                ips.push(*ipv6);
            }
        }

        Ok(ips)
    }
}

/// Create resolver config from resolver addresses
fn create_resolver_config(addrs: &[String]) -> Result<ResolverConfig> {
    use hickory_resolver::config::{NameServerConfig, Protocol};
    use std::net::{SocketAddr, ToSocketAddrs};

    let mut name_servers = NameServerConfigGroup::new();

    for addr in addrs {
        let socket_addr: SocketAddr = addr
            .to_socket_addrs()
            .map_err(|e| DnsxError::ResolverConfig(format!("Invalid resolver address {}: {}", addr, e)))?
            .next()
            .ok_or_else(|| DnsxError::ResolverConfig(format!("Failed to resolve {}", addr)))?;

        name_servers.push(NameServerConfig {
            socket_addr,
            protocol: Protocol::Udp, // Default to UDP
            tls_dns_name: None,
            trust_negative_responses: false,
            bind_addr: None,
        });
    }

    let mut config = ResolverConfig::new();
    config.add_name_server_config(name_servers);
    Ok(config)
}
