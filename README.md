# RDNSx

Fast and multi-purpose DNS toolkit written in Rust.

RDNSx is a Rust rewrite of the popular [DNSx](https://github.com/projectdiscovery/dnsx) tool, optimized for performance and accuracy using Rust's async capabilities and type safety.

## Features

- **Fast DNS Resolution**: High-performance DNS queries with async/await concurrency
- **Multiple Record Types**: Support for A, AAAA, CNAME, MX, TXT, NS, SOA, PTR, SRV records
- **Custom Resolvers**: Support for multiple DNS resolvers with load balancing and failover
- **Subdomain Enumeration**: Bruteforce subdomains using wordlists
- **Wildcard Filtering**: Advanced wildcard detection and filtering
- **Reverse DNS**: PTR queries for IP ranges and ASN lookups
- **Database Export**: Export results to Elasticsearch and MongoDB
- **CLI Interface**: User-friendly command-line interface
- **Library API**: Embeddable library for use in other Rust projects

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
git clone <repository-url>
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

Query A records for domains from a file:
```bash
rdnsx query -l domains.txt
```

Query specific record types:
```bash
rdnsx query -l domains.txt --mx --txt
```

Query from stdin:
```bash
echo "example.com" | rdnsx query
```

### Export to Elasticsearch

```bash
rdnsx query -l domains.txt --elasticsearch http://localhost:9200 --elasticsearch-index dns-records
```

### Export to MongoDB

```bash
rdnsx query -l domains.txt --mongodb mongodb://localhost:27017 --mongodb-database dnsx
```

### Bruteforce Subdomains

```bash
rdnsx bruteforce -d example.com -w wordlist.txt
```

### Reverse DNS Lookups

```bash
rdnsx ptr 173.0.84.0/24
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
    let ips = client.lookup("example.com").await?;
    for ip in ips {
        println!("{}", ip);
    }
    
    Ok(())
}
```

## Configuration

Global options:
- `-r, --resolvers`: Custom resolver list (file or comma-separated)
- `-t, --threads`: Concurrency level (default: 100)
- `-o, --output`: Output file
- `--json`: JSON output format
- `--silent`: Minimal output
- `--timeout`: Query timeout in seconds (default: 5)
- `--retries`: Retry attempts (default: 3)
- `--rate-limit`: Rate limit (queries per second, 0 = unlimited)

Export options:
- `--elasticsearch`: Elasticsearch connection string
- `--elasticsearch-index`: Elasticsearch index name (default: dnsx-records)
- `--mongodb`: MongoDB connection string
- `--mongodb-database`: MongoDB database name (default: dnsx)
- `--mongodb-collection`: MongoDB collection name (default: records)
- `--export-batch-size`: Batch size for database exports (default: 1000)

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
