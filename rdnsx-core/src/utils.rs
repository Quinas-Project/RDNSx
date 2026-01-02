//! Utility functions

use std::net::SocketAddr;

/// Parse a resolver string into a SocketAddr
/// Supports formats like: "8.8.8.8", "8.8.8.8:53", "[::1]:53"
pub fn parse_resolver(resolver_str: &str) -> crate::error::Result<SocketAddr> {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    // Handle IPv4 with optional port
    if let Ok(ipv4) = resolver_str.parse::<Ipv4Addr>() {
        return Ok(SocketAddr::new(IpAddr::V4(ipv4), 53));
    }

    // Handle IPv6 with optional port
    if let Ok(ipv6) = resolver_str.parse::<Ipv6Addr>() {
        return Ok(SocketAddr::new(IpAddr::V6(ipv6), 53));
    }

    // Handle full socket address
    if let Ok(sockaddr) = resolver_str.parse::<SocketAddr>() {
        return Ok(sockaddr);
    }

    // Handle hostname:port format (though we typically use IPs)
    if resolver_str.contains(':') {
        let parts: Vec<&str> = resolver_str.split(':').collect();
        if parts.len() == 2 {
            if let (Ok(ip), Ok(port)) = (parts[0].parse::<IpAddr>(), parts[1].parse::<u16>()) {
                return Ok(SocketAddr::new(ip, port));
            }
        }
    }

    Err(crate::error::DnsxError::invalid_input(format!(
        "Invalid resolver format: {}. Expected IP address or IP:port",
        resolver_str
    )))
}

/// Validate domain name format
pub fn is_valid_domain(domain: &str) -> bool {
    // Basic domain validation regex
    let re = regex::Regex::new(r"^([a-zA-Z0-9]([a-zA-Z0-9\-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,}$")
        .expect("Invalid regex pattern");
    re.is_match(domain)
}

/// Validate IP address
pub fn is_valid_ip(ip: &str) -> bool {
    ip.parse::<std::net::IpAddr>().is_ok()
}

/// Parse resolver string (can be IP or IP:port)
pub fn parse_resolver(addr: &str) -> crate::error::Result<String> {
    let parts: Vec<&str> = addr.split(':').collect();
    match parts.len() {
        1 => {
            // Just IP, add default port 53
            if is_valid_ip(parts[0]) {
                Ok(format!("{}:53", parts[0]))
            } else {
                Err(crate::error::DnsxError::invalid_input(format!(
                    "Invalid resolver IP: {}",
                    addr
                )))
            }
        }
        2 => {
            // IP:port
            if is_valid_ip(parts[0]) && parts[1].parse::<u16>().is_ok() {
                Ok(addr.to_string())
            } else {
                Err(crate::error::DnsxError::invalid_input(format!(
                    "Invalid resolver format: {}",
                    addr
                )))
            }
        }
        _ => Err(crate::error::DnsxError::invalid_input(format!(
            "Invalid resolver format: {}",
            addr
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_domain() {
        assert!(is_valid_domain("example.com"));
        assert!(is_valid_domain("sub.example.com"));
        assert!(is_valid_domain("a.b.c.example.com"));
        assert!(!is_valid_domain("invalid"));
        assert!(!is_valid_domain("example"));
        assert!(!is_valid_domain(".example.com"));
        assert!(!is_valid_domain("example..com"));
    }

    #[test]
    fn test_is_valid_ip() {
        assert!(is_valid_ip("8.8.8.8"));
        assert!(is_valid_ip("2001:4860:4860::8888"));
        assert!(!is_valid_ip("invalid"));
        assert!(!is_valid_ip("999.999.999.999"));
    }

    #[test]
    fn test_parse_resolver() {
        assert_eq!(parse_resolver("8.8.8.8").unwrap(), "8.8.8.8:53");
        assert_eq!(parse_resolver("8.8.8.8:53").unwrap(), "8.8.8.8:53");
        assert_eq!(parse_resolver("8.8.8.8:5353").unwrap(), "8.8.8.8:5353");
        assert!(parse_resolver("invalid").is_err());
        assert!(parse_resolver("8.8.8.8:invalid").is_err());
    }
}
