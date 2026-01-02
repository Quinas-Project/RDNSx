---
layout: home
title: RDNSx Documentation
description: "Complete documentation for RDNSx - Fast and multi-purpose DNS toolkit written in Rust"
---

# Welcome to RDNSx Documentation

RDNSx is a fast and multi-purpose DNS toolkit written in Rust, optimized for performance and accuracy using Rust's async capabilities and type safety.

## 🚀 Quick Start

Get up and running with RDNSx in minutes:

- **[Installation Guide](guide/installation)** - Install RDNSx on your system
- **[Quick Start](guide/quick-start)** - Your first DNS queries
- **[CLI Reference](api/cli-reference)** - Complete command reference

## 📚 Guides

### Getting Started
- [Installation](guide/installation) - How to install RDNSx
- [Quick Start](guide/quick-start) - Basic usage examples
- [Configuration](guide/configuration) - Configuration options

### DNS Operations
- [DNS Record Types](guide/dns-records) - Supported record types and usage
- [Querying Domains](guide/querying) - Domain resolution and enumeration
- [Subdomain Enumeration](guide/bruteforce) - Finding subdomains with wordlists
- [Reverse DNS](guide/reverse-dns) - IP to hostname lookups

### Advanced Features
- [Wildcard Filtering](guide/wildcard-filtering) - Handling wildcard DNS responses
- [Database Exports](guide/exports) - Exporting results to databases
- [Custom Resolvers](guide/resolvers) - Using custom DNS servers

## 🔧 API Reference

- [CLI Reference](api/cli-reference) - Complete command-line interface documentation
- [Library API](api/library) - Using RDNSx as a Rust library

## 📖 Examples

### Basic DNS Query
```bash
# Query A records for a domain
rdnsx query example.com

# Query multiple record types
rdnsx query example.com --record-type A --record-type MX
```

### Subdomain Enumeration
```bash
# Bruteforce subdomains
rdnsx bruteforce -d example.com -w wordlist.txt
```

### Reverse DNS Lookup
```bash
# PTR lookup for IP ranges
rdnsx ptr 192.168.1.0/24
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](https://github.com/Quinas-Project/RDNSx/blob/main/CONTRIBUTING.md) for details.

## 📄 License

RDNSx is released under the MIT License. See the [LICENSE](https://github.com/Quinas-Project/RDNSx/blob/main/LICENSE) file for details.

---

**Built by [Quinas Project](https://github.com/Quinas-Project)**