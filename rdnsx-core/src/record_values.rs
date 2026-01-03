//! DNS record value types and implementations

use std::net::IpAddr;
use serde::{Deserialize, Serialize};

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