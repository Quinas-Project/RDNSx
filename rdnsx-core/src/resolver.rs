//! DNS resolver pool implementation

use std::sync::Arc;
use std::time::Duration;

use hickory_resolver::config::{ResolverConfig, ResolverOpts};
use hickory_resolver::proto::rr::{RData, RecordType};
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
    _backup_resolver_addrs: Vec<String>,
    /// Concurrency semaphore
    semaphore: Arc<Semaphore>,
    /// Query timeout
    timeout: Duration,
    /// Number of retries
    _retries: u32,
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
        let primary_config = create_resolver_config(&resolver_configs[0..1].iter().map(|addr| addr.to_string()).collect::<Vec<_>>())?;
        let mut resolver_opts = ResolverOpts::default();
        resolver_opts.timeout = options.timeout;
        resolver_opts.attempts = options.retries as usize;
        resolver_opts.validate = false; // Don't validate, just resolve
        resolver_opts.use_hosts_file = false; // Don't use hosts file
        resolver_opts.ip_strategy = hickory_resolver::config::LookupIpStrategy::Ipv4thenIpv6; // Prefer IPv4

        // Try system resolver first
        debug!("Attempting to use system resolver configuration");
        let system_resolver = TokioAsyncResolver::tokio_from_system_conf();

        let resolver = match system_resolver {
            Ok(resolver) => {
                debug!("Successfully created system resolver");
                resolver
            }
            Err(e) => {
                debug!("System resolver failed ({}), using manual configuration", e);
                debug!("Creating resolver with config: {:?}", primary_config);
                debug!("Resolver options: timeout={:?}, attempts={}, validate={}", resolver_opts.timeout, resolver_opts.attempts, resolver_opts.validate);

                TokioAsyncResolver::tokio(primary_config, resolver_opts.clone())
            }
        };

        // Create backup resolvers if any
        let mut backup_resolvers = Vec::new();
        let mut backup_resolver_addrs = Vec::new();
        if resolver_configs.len() > 1 {
            for config in &resolver_configs[1..] {
                let backup_config = create_resolver_config(&[config.to_string()])?;
                let backup_resolver = TokioAsyncResolver::tokio(
                    backup_config,
                    resolver_opts.clone(),
                );
                backup_resolvers.push(backup_resolver);
                backup_resolver_addrs.push(config.to_string());
            }
        }

        Ok(Self {
            resolver,
            primary_resolver_addr: primary_resolver_addr.to_string(),
            backup_resolvers,
            _backup_resolver_addrs: backup_resolver_addrs,
            semaphore: Arc::new(Semaphore::new(options.concurrency)),
            timeout: options.timeout,
            _retries: options.retries,
        })
    }

    /// Query DNS with a specific record type
    pub async fn query(
        &self,
        domain: &str,
        record_type: RecordType,
    ) -> Result<(hickory_resolver::lookup::Lookup, String)> {
        let _permit = self.semaphore.acquire().await.map_err(|e| {
            DnsxError::Other(format!("Failed to acquire semaphore: {}", e))
        })?;

        let domain_name = hickory_resolver::Name::parse(domain, None)
            .map_err(|e| DnsxError::invalid_input(format!("Invalid domain name: {}", e)))?;

        // Try primary resolver first
        debug!("Querying {} ({}) using resolver at {}", domain, record_type, self.primary_resolver_addr);
        let result = tokio::time::timeout(self.timeout, self.resolver.lookup(domain_name.clone(), record_type))
            .await;

        match result {
            Ok(Ok(lookup)) => {
                debug!("Query successful for {} ({}), lookup contains {} records", domain, record_type, lookup.iter().count());
                for rdata in lookup.iter() {
                    debug!("Found record: {:?}", rdata);
                }
                Ok((lookup, self.primary_resolver_addr.clone()))
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
    ) -> Result<(hickory_resolver::lookup::Lookup, String)> {
        for backup in &self.backup_resolvers {
            let result = tokio::time::timeout(self.timeout, backup.lookup(domain_name.clone(), record_type))
                .await;
            match result {
                Ok(Ok(response)) => {
                    trace!("Backup resolver succeeded");
                    return Ok((response, "backup-resolver".to_string()));
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
        let (lookup, _) = self.query(domain, RecordType::A).await?;
        let mut ips = Vec::new();

        for rdata in lookup.iter() {
            if let RData::A(ipv4) = rdata {
                ips.push(**ipv4);
            }
        }

        Ok(ips)
    }

    /// Lookup AAAA records (IPv6)
    pub async fn lookup_ipv6(&self, domain: &str) -> Result<Vec<std::net::Ipv6Addr>> {
        let (lookup, _) = self.query(domain, RecordType::AAAA).await?;
        let mut ips = Vec::new();

        for rdata in lookup.iter() {
            if let RData::AAAA(ipv6) = rdata {
                ips.push(**ipv6);
            }
        }

        Ok(ips)
    }
}

/// Create resolver config from resolver addresses
fn create_resolver_config(addrs: &[String]) -> Result<ResolverConfig> {
    use hickory_resolver::config::{NameServerConfig, Protocol};
    use std::net::{SocketAddr, ToSocketAddrs};

    let mut config = ResolverConfig::new();

    for addr in addrs {
        let socket_addr: SocketAddr = addr
            .to_socket_addrs()
            .map_err(|e| DnsxError::ResolverConfig(format!("Invalid resolver address {}: {}", addr, e)))?
            .next()
            .ok_or_else(|| DnsxError::ResolverConfig(format!("Failed to resolve {}", addr)))?;

        config.add_name_server(NameServerConfig {
            socket_addr,
            protocol: Protocol::Udp, // Default to UDP
            tls_dns_name: None,
            trust_negative_responses: false,
            bind_addr: None,
            tls_config: None,
        });
    }

    Ok(config)
}
