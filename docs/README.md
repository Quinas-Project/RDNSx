# RDNSx Documentation

<div align="center">
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/DNS-007ACC?style=for-the-badge&logo=dns&logoColor=white" alt="DNS">
  <img src="https://img.shields.io/badge/Security-FF6B35?style=for-the-badge&logo=security&logoColor=white" alt="Security">
</div>

<div align="center">
  <h1>🚀 RDNSx - Fast and Multi-Purpose DNS Toolkit</h1>
  <p><em>Advanced DNS reconnaissance and resolution toolkit written in Rust</em></p>
</div>

## ✨ Features

- **🔍 Comprehensive DNS Resolution**: Support for 27+ DNS record types (A, AAAA, CNAME, MX, TXT, SOA, PTR, SRV, CAA, CERT, DNAME, DNSKEY, DS, HINFO, HTTPS, KEY, LOC, NAPTR, NSEC, NSEC3, OPT, RRSIG, SSHFP, SVCB, TLSA, URI)
- **🛡️ Security-First**: Zero unsafe code, memory-safe throughout, regular security audits
- **📊 Multi-Database Export**: Elasticsearch, MongoDB, Cassandra support with batched operations
- **🔄 Advanced Features**: Wildcard detection, subdomain bruteforcing, PTR lookups, ASN support
- **⚡ High Performance**: Async Rust implementation with concurrent queries and connection pooling
- **🎯 Production Ready**: Comprehensive testing, CI/CD pipelines, enterprise-grade reliability

## 🚀 Quick Start

```bash
# Install RDNSx
cargo install rdnsx

# Basic DNS query
rdnsx query example.com

# Query specific record types
rdnsx query example.com --a --aaaa --mx --txt

# Export to Elasticsearch
rdnsx query example.com --elasticsearch http://localhost:9200 --elasticsearch-index dns-records

# Subdomain enumeration
rdnsx bruteforce example.com --wordlist subdomains.txt
```

## 📖 Documentation

- [📚 Installation Guide](./guide/installation.md)
- [🚀 Quick Start](./guide/quick-start.md)
- [🔍 DNS Records](./guide/dns-records.md)
- [📊 Database Exports](./guide/exports.md)
- [⚙️ Advanced Usage](./guide/advanced-usage.md)
- [🛠️ CLI Reference](./api/cli-reference.md)
- [📖 Library API](./api/library-api.md)

## 🏗️ Architecture

RDNSx is built with a modular architecture:

```
rdnsx/           # CLI application
├── commands/    # CLI subcommands
├── config.rs    # Configuration management
└── main.rs      # Application entry point

rdnsx-core/      # Core DNS library
├── client.rs    # Main DNS client
├── resolver.rs  # DNS resolver with failover
├── query.rs     # Query engine
├── export/      # Database exporters
├── types.rs     # DNS record types
└── wildcard.rs  # Wildcard detection
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](https://github.com/Quinas-Project/RDNSx/blob/main/CONTRIBUTING.md) for details.

## 📄 License

Licensed under the MIT License - see the [LICENSE](https://github.com/Quinas-Project/RDNSx/blob/main/LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Hickory DNS](https://github.com/hickory-dns/hickory-dns) - Modern DNS library for Rust
- Inspired by [dnsx](https://github.com/projectdiscovery/dnsx) - Original Go implementation
- Community contributions and feedback

---

<div align="center">
  <p>Made with ❤️ by the RDNSx community</p>
  <p>
    <a href="https://github.com/Quinas-Project/RDNSx">GitHub</a> •
    <a href="https://github.com/Quinas-Project/RDNSx/issues">Issues</a> •
    <a href="https://github.com/Quinas-Project/RDNSx/discussions">Discussions</a>
  </p>
</div>