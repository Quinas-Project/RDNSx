//! Utility functions

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
pub fn parse_resolver(addr: &str) -> Result<String, crate::error::DnsxError> {
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
