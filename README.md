# RDNSx

Fast and multi-purpose DNS toolkit written in Rust.

RDNSx is a Rust rewrite of the popular [DNSx](https://github.com/projectdiscovery/dnsx) tool, optimized for performance and accuracy using Rust's async capabilities and type safety.

## Features

- **Fast DNS Resolution**: High-performance DNS queries with async/await concurrency
- **Comprehensive Record Types**: Support for 27 DNS record types (see table below)
- **Custom Resolvers**: Support for multiple DNS resolvers with load balancing and failover
- **Subdomain Enumeration**: Bruteforce subdomains using wordlists
- **Wildcard Filtering**: Advanced wildcard detection and filtering
- **ASN Enumeration**: Discover IP ranges and network information for Autonomous Systems (Google, Amazon, Cloudflare, etc.)
- **Enhanced Reverse DNS**: Concurrent PTR queries for IP ranges and ASN-based lookups with smart rate limiting
- **Advanced Enumeration**: 11 specialized enumeration techniques (see table below)
- **Database Export**: Export results to Elasticsearch, MongoDB, and Cassandra
- **CLI Interface**: User-friendly command-line interface with comprehensive options
- **Library API**: Embeddable library for use in other Rust projects

## DNS Record Types

RDNSx supports comprehensive DNS record type querying for thorough network analysis:

| Record Type | Description | Use Case |
|-------------|-------------|----------|
| **A** | IPv4 address record | Maps domain to IP address |
| **AAAA** | IPv6 address record | Maps domain to IPv6 address |
| **CNAME** | Canonical name record | Domain aliases and redirects |
| **MX** | Mail exchange record | Email server configuration |
| **TXT** | Text record | SPF, DKIM, DMARC, and custom data |
| **NS** | Name server record | Authoritative DNS servers |
| **SOA** | Start of authority | Zone administrative information |
| **PTR** | Pointer record | Reverse DNS (IP to hostname) |
| **SRV** | Service locator record | Service discovery (SIP, XMPP, etc.) |
| **CAA** | Certification authority authorization | SSL certificate restrictions |
| **DNSKEY** | DNSSEC public key | DNSSEC key storage |
| **DS** | Delegation signer | DNSSEC key verification |
| **RRSIG** | DNSSEC signature | Resource record signatures |
| **NSEC** | Next secure record | DNSSEC proof of non-existence |
| **NSEC3** | Hashed next secure record | DNSSEC privacy protection |
| **HINFO** | Host information | Hardware/software descriptions |
| **HTTPS** | HTTPS service binding | HTTP/2 and HTTPS configuration |
| **NAPTR** | Name authority pointer | Dynamic delegation discovery |
| **SSHFP** | SSH fingerprint | SSH host key verification |
| **SVCB** | Service binding | Service parameters (HTTPS upgrade) |
| **TLSA** | TLS certificate association | DANE (DNS-based Authentication) |
| **URI** | Uniform resource identifier | URI redirects |
| **AFSDB** | AFS database location | Andrew File System |
| **CERT** | Certificate record | Public key certificates |
| **DNAME** | Delegation name | Domain redirection |
| **KEY** | Key record | Public keys for DNSSEC |
| **LOC** | Location record | Geographic coordinates |
| **OPT** | Option record | EDNS extensions |

## Documentation

📚 **[Complete Documentation](https://docs.rdnsx.dev)** - Installation guides, API reference, and examples

### Quick Links
- [Installation Guide](https://docs.rdnsx.dev/guide/installation)
- [Quick Start](https://docs.rdnsx.dev/guide/quick-start)
- [CLI Reference](https://docs.rdnsx.dev/api/cli-reference)
- [DNS Record Types](https://docs.rdnsx.dev/guide/dns-records)
- [Configuration Guide](https://docs.rdnsx.dev/guide/configuration)

## Prerequisites

**Rust 1.70+** is required. Install Rust from [rustup.rs](https://rustup.rs/):

```bash
# On Windows (PowerShell)
Invoke-WebRequest -Uri https://win.rustup.rs/x86_64 -OutFile rustup-init.exe
.\rustup-init.exe

# On Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, restart your terminal or run:
```bash
source $HOME/.cargo/env  # Linux/macOS
# Or restart terminal on Windows
```

## Installation

### Build from source:

```bash
git clone https://github.com/Quinas-Project/RDNSx.git
cd RDNSx
cargo build --release
```

The binary will be located at `target/release/rdnsx` (or `target/release/rdnsx.exe` on Windows).

### Install globally:

```bash
cargo install --path .
```

This will install `rdnsx` to `~/.cargo/bin` (or `%USERPROFILE%\.cargo\bin` on Windows).

### Docker Installation

#### Using Pre-built Images
```bash
# Pull from Docker Hub
docker pull quinas/rdnsx:latest
docker run --rm quinas/rdnsx --help
```

#### Building from Source
```bash
# Build using provided Dockerfile
docker build -f docker/Dockerfile -t rdnsx .
docker run --rm rdnsx --help
```

#### Docker Compose Setup
For complex deployments with database exports:
```bash
# Start RDNSx with databases
docker-compose -f docker/docker-compose.yml up

# Mount config directory for persistent settings
docker run --rm -v $(pwd)/config:/app/config rdnsx --config /app/config/rdnsx.toml query example.com
```

## Usage

### Query DNS Records

Query A records for domains from stdin:
```bash
echo "example.com" | rdnsx query
```

Query from a file:
```bash
rdnsx query --list domains.txt
```

Query specific record types:
```bash
rdnsx query --list domains.txt --record-type MX --record-type TXT
```

Use custom configuration:
```bash
rdnsx --config config/rdnsx.toml query example.com
```

### Advanced Enumeration

RDNSx supports comprehensive DNS enumeration techniques for security research and network analysis:

#### ASN Enumeration
Discover IP ranges and network information for Autonomous Systems:
```bash
# Enumerate Google ASN
rdnsx enumerate --technique asn-enumeration --target AS15169

# Works with or without "AS" prefix
rdnsx enumerate --technique asn-enumeration --target 16509
```

#### Comprehensive DNS Analysis
Perform complete DNS enumeration on a target:
```bash
rdnsx enumerate --technique comprehensive --target example.com
```

### Enumeration Techniques

RDNSx provides 11 specialized enumeration techniques for comprehensive DNS reconnaissance:

| Technique | Command | Description |
|-----------|---------|-------------|
| **Zone Transfer** | `zone-transfer` | Attempt DNS zone transfer (AXFR) to retrieve all records |
| **Email Security** | `email-security` | Enumerate SPF, DMARC, DKIM records for email authentication |
| **CDN Detection** | `cdn-detection` | Detect CDN usage and analyze configuration |
| **IPv6 Enumeration** | `ipv6-enumeration` | Enumerate IPv6 deployment and addresses |
| **DNSSEC Analysis** | `dnssec-enumeration` | Analyze DNSSEC configuration and security |
| **DNSSEC Zone Walking** | `dnssec-zone-walking` | Perform DNSSEC zone walking (NSEC enumeration) |
| **Wildcard Analysis** | `wildcard-analysis` | Analyze wildcard DNS configurations and bypass techniques |
| **Passive DNS** | `passive-dns` | Perform passive DNS enumeration using historical data |
| **Server Fingerprint** | `server-fingerprint` | Fingerprint DNS server capabilities and versions |
| **ASN Enumeration** | `asn-enumeration` | Enumerate ASN information and associated IP ranges |
| **Comprehensive** | `comprehensive` | Run all enumeration techniques combined |

### Enhanced Reverse DNS Lookups

RDNSx provides powerful PTR enumeration with ASN integration:

#### ASN-Based PTR Lookups
Use ASN enumeration results for targeted reverse DNS:
```bash
# Lookup PTR records for Google ASN
rdnsx ptr AS15169

# Results include IP ranges automatically discovered from ASN
```

#### Smart IP Range Handling
Large IP ranges are automatically limited to prevent excessive lookups:
```bash
# Large ranges are capped at 10,000 IPs
rdnsx ptr 192.168.0.0/16  # Limited to 10,000 IPs with warning

# Per-prefix limits for ASN ranges
rdnsx ptr AS16509         # Up to 1,000 IPs per prefix
```

#### Concurrent Processing
PTR lookups use concurrent processing for better performance:
```bash
# Up to 50 simultaneous PTR queries
rdnsx ptr 8.8.8.0/24
```

## Configuration

RDNSx uses a TOML configuration file for advanced settings. Configuration files are stored in the `config/` directory:

### Creating Configuration
```bash
# Create example configuration file
rdnsx --create-config config/rdnsx.toml
```

### Using Configuration
```bash
# Use custom configuration with any command
rdnsx --config config/rdnsx.toml query example.com

# Configuration includes resolver settings, performance tuning, and database exports
```

### Configuration Options

The config file supports:
- **DNS Resolvers**: Custom DNS servers with timeout/retry settings
- **Performance**: Concurrency limits and rate limiting
- **Database Exports**: Elasticsearch, MongoDB, and Cassandra configuration

**Example configuration:**
```toml
[resolvers]
servers = ["8.8.8.8", "1.1.1.1"]
timeout = 5
retries = 3

[performance]
threads = 100
rate_limit = 0

[export.elasticsearch]
enabled = true
url = "http://localhost:9200"
index = "dns-records"
```

### Export to Databases

Configure exports in your `rdnsx.toml` config file:

```toml
[export.elasticsearch]
enabled = true
url = "http://localhost:9200"
index = "dns-records"

[export.mongodb]
enabled = true
url = "mongodb://localhost:27017"
database = "dnsx"

[export.cassandra]
enabled = true
contact_points = ["127.0.0.1:9042"]
username = "cassandra"
password = "password"
```

Then run queries with export enabled:
```bash
rdnsx --config config/rdnsx.toml query example.com
```

### Bruteforce Subdomains

```bash
rdnsx bruteforce --domain example.com --wordlist wordlist.txt
```

### Reverse DNS Lookups

```bash
rdnsx ptr 192.168.1.0/24
```

## Library Usage

```rust
use rdnsx_core::{DnsxClient, RecordType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DnsxClient::new()?;

    // Query A records
    let records = client.query("example.com", RecordType::A).await?;
    for record in records {
        println!("{}", record);
    }

    // Lookup IP addresses
    let ips = client.lookup_ipv4("example.com").await?;
    for ip in ips {
        println!("{}", ip);
    }

    Ok(())
}
```

## Command Line Options

Global options:
- `-c, --config`: Configuration file path
- `-o, --output`: Output file
- `--json`: JSON output format
- `--silent`: Minimal output
- `--create-config`: Create example configuration file

Command-specific options are documented in `--help` for each command.

## Performance

RDNSx is optimized for high-performance DNS resolution:
- Async/await throughout for non-blocking I/O
- Connection pooling and reuse
- Configurable concurrency limits
- Efficient batching for database exports

## License

MIT License - see LICENSE file for details

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.
