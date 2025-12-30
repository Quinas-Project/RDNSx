//! DNS record types and data structures

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

/// DNS record types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum RecordType {
    /// A record (IPv4 address)
    A,
    /// AAAA record (IPv6 address)
    Aaaa,
    /// CNAME record (canonical name)
    Cname,
    /// MX record (mail exchange)
    Mx,
    /// TXT record (text)
    Txt,
    /// NS record (name server)
    Ns,
    /// SOA record (start of authority)
    Soa,
    /// PTR record (pointer/reverse DNS)
    Ptr,
    /// SRV record (service)
    Srv,
    /// AFSDB record
    Afsdb,
}

impl RecordType {
    /// Get all supported record types
    pub fn all() -> Vec<RecordType> {
        vec![
            RecordType::A,
            RecordType::Aaaa,
            RecordType::Cname,
            RecordType::Mx,
            RecordType::Txt,
            RecordType::Ns,
            RecordType::Soa,
            RecordType::Ptr,
            RecordType::Srv,
            RecordType::Afsdb,
        ]
    }

    /// Convert to hickory-dns RecordType
    pub fn to_hickory(&self) -> hickory_resolver::proto::rr::RecordType {
        use hickory_resolver::proto::rr::RecordType as HRecordType;
        match self {
            RecordType::A => HRecordType::A,
            RecordType::Aaaa => HRecordType::AAAA,
            RecordType::Cname => HRecordType::CNAME,
            RecordType::Mx => HRecordType::MX,
            RecordType::Txt => HRecordType::TXT,
            RecordType::Ns => HRecordType::NS,
            RecordType::Soa => HRecordType::SOA,
            RecordType::Ptr => HRecordType::PTR,
            RecordType::Srv => HRecordType::SRV,
            RecordType::Afsdb => HRecordType::AFSDB,
        }
    }
}

impl std::fmt::Display for RecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecordType::A => write!(f, "A"),
            RecordType::Aaaa => write!(f, "AAAA"),
            RecordType::Cname => write!(f, "CNAME"),
            RecordType::Mx => write!(f, "MX"),
            RecordType::Txt => write!(f, "TXT"),
            RecordType::Ns => write!(f, "NS"),
            RecordType::Soa => write!(f, "SOA"),
            RecordType::Ptr => write!(f, "PTR"),
            RecordType::Srv => write!(f, "SRV"),
            RecordType::Afsdb => write!(f, "AFSDB"),
        }
    }
}

/// DNS response code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ResponseCode {
    /// No error condition
    NoError,
    /// Name server failure
    ServFail,
    /// Name does not exist
    NxDomain,
    /// Query refused
    Refused,
    /// Format error
    FormErr,
    /// Not implemented
    NotImp,
    /// Server failure
    ServFailOther,
}

impl ResponseCode {
    /// Convert from hickory-dns ResponseCode
    pub fn from_hickory(code: hickory_resolver::proto::op::ResponseCode) -> Self {
        use hickory_resolver::proto::op::ResponseCode as HResponseCode;
        match code {
            HResponseCode::NoError => Self::NoError,
            HResponseCode::ServFail => Self::ServFail,
            HResponseCode::NXDomain => Self::NxDomain,
            HResponseCode::Refused => Self::Refused,
            HResponseCode::FormErr => Self::FormErr,
            HResponseCode::NotImp => Self::NotImp,
            _ => Self::ServFailOther,
        }
    }
}

impl std::fmt::Display for ResponseCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseCode::NoError => write!(f, "NOERROR"),
            ResponseCode::ServFail => write!(f, "SERVFAIL"),
            ResponseCode::NxDomain => write!(f, "NXDOMAIN"),
            ResponseCode::Refused => write!(f, "REFUSED"),
            ResponseCode::FormErr => write!(f, "FORMERR"),
            ResponseCode::NotImp => write!(f, "NOTIMP"),
            ResponseCode::ServFailOther => write!(f, "SERVFAIL"),
        }
    }
}

/// DNS record value
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RecordValue {
    /// IP address (A or AAAA)
    Ip(IpAddr),
    /// Domain name (CNAME, NS, PTR)
    Domain(String),
    /// Text value (TXT)
    Text(String),
    /// MX record with priority
    Mx { priority: u16, exchange: String },
    /// SRV record
    Srv {
        priority: u16,
        weight: u16,
        port: u16,
        target: String,
    },
    /// SOA record
    Soa {
        mname: String,
        rname: String,
        serial: u32,
        refresh: i32,
        retry: i32,
        expire: i32,
        minimum: u32,
    },
    /// Generic record value
    Other(String),
}

impl RecordValue {
    /// Convert to string representation
    pub fn to_string(&self) -> String {
        match self {
            RecordValue::Ip(ip) => ip.to_string(),
            RecordValue::Domain(d) => d.clone(),
            RecordValue::Text(t) => t.clone(),
            RecordValue::Mx { priority, exchange } => format!("{} {}", priority, exchange),
            RecordValue::Srv {
                priority,
                weight,
                port,
                target,
            } => format!("{} {} {} {}", priority, weight, port, target),
            RecordValue::Soa { .. } => "SOA".to_string(), // SOA is complex, simplified here
            RecordValue::Other(o) => o.clone(),
        }
    }
}

/// DNS record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsRecord {
    /// Domain name queried
    pub domain: String,
    /// Record type
    pub record_type: RecordType,
    /// Record value(s)
    pub value: RecordValue,
    /// Time to live
    pub ttl: u32,
    /// Response code
    pub response_code: ResponseCode,
    /// Resolver used
    pub resolver: String,
    /// Query timestamp
    pub timestamp: SystemTime,
    /// Query time in milliseconds
    pub query_time_ms: f64,
}

impl DnsRecord {
    /// Create a new DNS record
    pub fn new(
        domain: String,
        record_type: RecordType,
        value: RecordValue,
        ttl: u32,
        response_code: ResponseCode,
        resolver: String,
        query_time_ms: f64,
    ) -> Self {
        Self {
            domain,
            record_type,
            value,
            ttl,
            response_code,
            resolver,
            timestamp: SystemTime::now(),
            query_time_ms,
        }
    }
}

impl std::fmt::Display for DnsRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} [{}]",
            self.domain,
            self.value.to_string()
        )
    }
}
