//! DNS record types and data structures

use std::net::IpAddr;
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
    /// CAA record (Certification Authority Authorization)
    Caa,
    /// CERT record (certificate)
    Cert,
    /// DNAME record (delegation name)
    Dname,
    /// DNSKEY record (DNSSEC key)
    Dnskey,
    /// DS record (delegation signer)
    Ds,
    /// HINFO record (host information)
    Hinfo,
    /// HTTPS record
    Https,
    /// KEY record
    Key,
    /// LOC record (location)
    Loc,
    /// NAPTR record (naming authority pointer)
    Naptr,
    /// NSEC record (DNSSEC)
    Nsec,
    /// NSEC3 record (DNSSEC)
    Nsec3,
    /// OPT record (EDNS options)
    Opt,
    /// RRSIG record (DNSSEC signature)
    Rrsig,
    /// SSHFP record (SSH fingerprint)
    Sshfp,
    /// SVCB record (service binding)
    Svcb,
    /// TLSA record (TLSA certificate association)
    Tlsa,
    /// URI record (uniform resource identifier)
    Uri,
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
            RecordType::Caa,
            RecordType::Cert,
            RecordType::Dname,
            RecordType::Dnskey,
            RecordType::Ds,
            RecordType::Hinfo,
            RecordType::Https,
            RecordType::Key,
            RecordType::Loc,
            RecordType::Naptr,
            RecordType::Nsec,
            RecordType::Nsec3,
            RecordType::Opt,
            RecordType::Rrsig,
            RecordType::Sshfp,
            RecordType::Svcb,
            RecordType::Tlsa,
            RecordType::Uri,
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
            // RecordType::Afsdb => HRecordType::AFSDB,
            RecordType::Caa => HRecordType::CAA,
            // RecordType::Cert => HRecordType::CERT,
            RecordType::Dname => HRecordType::ANAME,
            RecordType::Dnskey => HRecordType::DNSKEY,
            RecordType::Ds => HRecordType::DS,
            RecordType::Hinfo => HRecordType::HINFO,
            RecordType::Https => HRecordType::HTTPS,
            RecordType::Key => HRecordType::KEY,
            // RecordType::Loc => HRecordType::LOC,
            RecordType::Naptr => HRecordType::NAPTR,
            RecordType::Nsec => HRecordType::NSEC,
            RecordType::Nsec3 => HRecordType::NSEC3,
            RecordType::Opt => HRecordType::OPT,
            RecordType::Rrsig => HRecordType::RRSIG,
            RecordType::Sshfp => HRecordType::SSHFP,
            RecordType::Svcb => HRecordType::SVCB,
            RecordType::Tlsa => HRecordType::TLSA,
            // RecordType::Uri => HRecordType::URI,
            // Unsupported record types - return A as fallback
            RecordType::Afsdb | RecordType::Cert | RecordType::Loc | RecordType::Uri => HRecordType::A,
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
            RecordType::Caa => write!(f, "CAA"),
            RecordType::Cert => write!(f, "CERT"),
            RecordType::Dname => write!(f, "DNAME"),
            RecordType::Dnskey => write!(f, "DNSKEY"),
            RecordType::Ds => write!(f, "DS"),
            RecordType::Hinfo => write!(f, "HINFO"),
            RecordType::Https => write!(f, "HTTPS"),
            RecordType::Key => write!(f, "KEY"),
            RecordType::Loc => write!(f, "LOC"),
            RecordType::Naptr => write!(f, "NAPTR"),
            RecordType::Nsec => write!(f, "NSEC"),
            RecordType::Nsec3 => write!(f, "NSEC3"),
            RecordType::Opt => write!(f, "OPT"),
            RecordType::Rrsig => write!(f, "RRSIG"),
            RecordType::Sshfp => write!(f, "SSHFP"),
            RecordType::Svcb => write!(f, "SVCB"),
            RecordType::Tlsa => write!(f, "TLSA"),
            RecordType::Uri => write!(f, "URI"),
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
    /// Domain name (CNAME, NS, PTR, DNAME)
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
    /// CAA record (Certification Authority Authorization)
    Caa { flags: u8, tag: String, value: String },
    /// CERT record
    Cert {
        cert_type: u16,
        key_tag: u16,
        algorithm: u8,
        certificate: Vec<u8>,
    },
    /// DNSKEY record (DNSSEC)
    Dnskey {
        flags: u16,
        protocol: u8,
        algorithm: u8,
        public_key: Vec<u8>,
    },
    /// DS record (DNSSEC)
    Ds {
        key_tag: u16,
        algorithm: u8,
        digest_type: u8,
        digest: Vec<u8>,
    },
    /// HINFO record
    Hinfo { cpu: String, os: String },
    /// HTTPS record (similar to SVCB)
    Https {
        priority: u16,
        target: String,
        params: Vec<String>,
    },
    /// KEY record
    Key {
        flags: u16,
        protocol: u8,
        algorithm: u8,
        public_key: Vec<u8>,
    },
    /// LOC record (location)
    Loc {
        version: u8,
        size: u8,
        horiz_pre: u8,
        vert_pre: u8,
        latitude: u32,
        longitude: u32,
        altitude: u32,
    },
    /// NAPTR record
    Naptr {
        order: u16,
        preference: u16,
        flags: String,
        services: String,
        regexp: String,
        replacement: String,
    },
    /// SSHFP record
    Sshfp {
        algorithm: u8,
        fingerprint_type: u8,
        fingerprint: Vec<u8>,
    },
    /// SVCB record (service binding)
    Svcb {
        priority: u16,
        target: String,
        params: Vec<String>,
    },
    /// TLSA record
    Tlsa {
        cert_usage: u8,
        selector: u8,
        matching_type: u8,
        cert_data: Vec<u8>,
    },
    /// URI record
    Uri { priority: u16, weight: u16, target: String },
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
            RecordValue::Caa { flags, tag, value } => format!("{} {} {}", flags, tag, value),
            RecordValue::Cert { cert_type, key_tag, algorithm, .. } => {
                format!("{} {} {}", cert_type, key_tag, algorithm)
            }
            RecordValue::Dnskey { flags, protocol, algorithm, .. } => {
                format!("{} {} {}", flags, protocol, algorithm)
            }
            RecordValue::Ds { key_tag, algorithm, digest_type, .. } => {
                format!("{} {} {}", key_tag, algorithm, digest_type)
            }
            RecordValue::Hinfo { cpu, os } => format!("{} {}", cpu, os),
            RecordValue::Https { priority, target, params } => {
                format!("{} {} {}", priority, target, params.join(" "))
            }
            RecordValue::Key { flags, protocol, algorithm, .. } => {
                format!("{} {} {}", flags, protocol, algorithm)
            }
            RecordValue::Loc { latitude, longitude, altitude, .. } => {
                format!("{} {} {}", latitude, longitude, altitude)
            }
            RecordValue::Naptr { order, preference, flags, services, regexp, replacement } => {
                format!("{} {} {} {} {} {}", order, preference, flags, services, regexp, replacement)
            }
            RecordValue::Sshfp { algorithm, fingerprint_type, .. } => {
                format!("{} {}", algorithm, fingerprint_type)
            }
            RecordValue::Svcb { priority, target, params } => {
                format!("{} {} {}", priority, target, params.join(" "))
            }
            RecordValue::Tlsa { cert_usage, selector, matching_type, .. } => {
                format!("{} {} {}", cert_usage, selector, matching_type)
            }
            RecordValue::Uri { priority, weight, target } => {
                format!("{} {} {}", priority, weight, target)
            }
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
