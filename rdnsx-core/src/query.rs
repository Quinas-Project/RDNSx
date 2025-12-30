//! DNS query engine

use std::time::Instant;

use hickory_resolver::proto::rr::{RData, Record, RecordType as HRecordType};
use tracing::debug;

use crate::error::{DnsxError, Result};
use crate::resolver::ResolverPool;
use crate::types::{DnsRecord, RecordType, RecordValue, ResponseCode};

/// DNS query engine
pub struct QueryEngine {
    resolver_pool: ResolverPool,
}

impl QueryEngine {
    /// Create a new query engine
    pub fn new(resolver_pool: ResolverPool) -> Self {
        Self { resolver_pool }
    }

    /// Query a domain for a specific record type
    pub async fn query(&self, domain: &str, record_type: RecordType) -> Result<Vec<DnsRecord>> {
        let start = Instant::now();
        let hickory_type = record_type.to_hickory();

        let (response, resolver_addr) = self.resolver_pool.query(domain, hickory_type).await?;

        let query_time_ms = start.elapsed().as_secs_f64() * 1000.0;
        let response_code = ResponseCode::from_hickory(response.response_code());

        let mut records = Vec::new();

        // Extract records from response
        for record in response.records() {
            if let Some(rdata) = record.data() {
                let value = parse_rdata(rdata, record_type)?;
                let ttl = record.ttl();

                records.push(DnsRecord::new(
                    domain.to_string(),
                    record_type,
                    value,
                    ttl,
                    response_code,
                    resolver_addr.clone(),
                    query_time_ms,
                ));
            }
        }

        // If no records but response was successful, still create a record entry
        if records.is_empty() && response_code == ResponseCode::NoError {
            records.push(DnsRecord::new(
                domain.to_string(),
                record_type,
                RecordValue::Other("No records found".to_string()),
                0,
                response_code,
                resolver_addr,
                query_time_ms,
            ));
        }

        Ok(records)
    }

    /// Lookup A records and return IP addresses
    pub async fn lookup_ipv4(&self, domain: &str) -> Result<Vec<std::net::Ipv4Addr>> {
        self.resolver_pool.lookup_ipv4(domain).await
    }

    /// Lookup AAAA records and return IP addresses
    pub async fn lookup_ipv6(&self, domain: &str) -> Result<Vec<std::net::Ipv6Addr>> {
        self.resolver_pool.lookup_ipv6(domain).await
    }
}

/// Parse RData into RecordValue
fn parse_rdata(rdata: &RData, record_type: RecordType) -> Result<RecordValue> {
    match rdata {
        RData::A(ipv4) => Ok(RecordValue::Ip((*ipv4).into())),
        RData::AAAA(ipv6) => Ok(RecordValue::Ip((*ipv6).into())),
        RData::CNAME(cname) => Ok(RecordValue::Domain(cname.to_string())),
        RData::PTR(ptr) => Ok(RecordValue::Domain(ptr.to_string())),
        RData::NS(ns) => Ok(RecordValue::Domain(ns.to_string())),
        RData::MX(mx) => Ok(RecordValue::Mx {
            priority: mx.preference(),
            exchange: mx.exchange().to_string(),
        }),
        RData::TXT(txt) => {
            // TXT records can have multiple strings, join them
            let text = txt.iter()
                .map(|bytes| String::from_utf8_lossy(bytes))
                .collect::<Vec<_>>()
                .join("");
            Ok(RecordValue::Text(text))
        }
        RData::SRV(srv) => Ok(RecordValue::Srv {
            priority: srv.priority(),
            weight: srv.weight(),
            port: srv.port(),
            target: srv.target().to_string(),
        }),
        RData::SOA(soa) => Ok(RecordValue::Soa {
            mname: soa.mname().to_string(),
            rname: soa.rname().to_string(),
            serial: soa.serial(),
            refresh: soa.refresh(),
            retry: soa.retry(),
            expire: soa.expire(),
            minimum: soa.minimum(),
        }),
        _ => Ok(RecordValue::Other(format!("{:?}", rdata))),
    }
}
