# RDNSx

Fast and multi-purpose DNS toolkit written in Rust.

RDNSx is a Rust rewrite of the popular [DNSx](https://github.com/projectdiscovery/dnsx) tool, optimized for performance and accuracy using Rust's async capabilities and type safety.

## Features

- **Fast DNS Resolution**: High-performance DNS queries with async/await concurrency
- **Comprehensive Record Types**: Support for 27 DNS record types including A, AAAA, CNAME, MX, TXT, NS, SOA, PTR, SRV, CAA, CERT, DNSKEY, DS, HINFO, HTTPS, KEY, LOC, NAPTR, NSEC, NSEC3, OPT, RRSIG, SSHFP, SVCB, TLSA, URI
- **Custom Resolvers**: Support for multiple DNS resolvers with load balancing and failover
- **Subdomain Enumeration**: Bruteforce subdomains using wordlists
- **Wildcard Filtering**: Advanced wildcard detection and filtering
- **Reverse DNS**: PTR queries for IP ranges and ASN lookups
- **Database Export**: Export results to Elasticsearch, MongoDB, and Cassandra
- **CLI Interface**: User-friendly command-line interface with comprehensive options
- **Library API**: Embeddable library for use in other Rust projects

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
rdnsx --config rdnsx.toml query example.com
```

## Configuration

RDNSx uses a simple configuration file to manage settings. Create a config file using:

```bash
rdnsx --create-config rdnsx.toml
```

Then use it with any command:

```bash
rdnsx --config rdnsx.toml query example.com
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
rdnsx --config rdnsx.toml query example.com
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
