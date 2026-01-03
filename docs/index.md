---
layout: home
title: "RDNSx Documentation - Fast DNS Toolkit in Rust"
description: "Complete documentation for RDNSx, a high-performance DNS toolkit written in Rust. Features ASN enumeration, reverse DNS lookups, subdomain enumeration, and comprehensive DNS analysis tools for security research."
keywords: "RDNSx, DNS toolkit, Rust, documentation, ASN enumeration, reverse DNS, subdomain enumeration, DNSSEC, security tools, network analysis"
og_image: /assets/images/logo.svg
twitter_card: summary_large_image
author: "Quinas Project"
lang: en-US
permalink: /
priority: 1.0
changefreq: weekly
---

# Welcome to RDNSx Documentation

RDNSx is a **high-performance DNS toolkit written in Rust**, designed for security researchers, network administrators, and developers. Leveraging Rust's memory safety and async capabilities, RDNSx provides comprehensive DNS enumeration, ASN analysis, and reverse DNS lookup functionality.

## What is RDNSx?

RDNSx combines the power of modern async programming with battle-tested DNS resolution techniques. Whether you're conducting security assessments, network reconnaissance, or building DNS-aware applications, RDNSx delivers the tools you need with exceptional performance and reliability.

### Key Features

- ⚡ **Blazing Fast**: Async DNS resolution with configurable concurrency
- 🔍 **Comprehensive Enumeration**: Support for 27+ DNS record types
- 🏢 **ASN Intelligence**: Discover IP ranges and network information for major organizations
- 🔄 **Reverse DNS**: Concurrent PTR lookups with smart rate limiting
- 🛡️ **Security Focused**: Advanced DNSSEC analysis and wildcard detection
- 📊 **Database Integration**: Export results to Elasticsearch, MongoDB, and Cassandra
- 🐳 **Container Ready**: Docker images for easy deployment

## 🚀 Quick Start Guide

New to RDNSx? Get started in minutes with our comprehensive guides:

- **[📦 Installation Guide](guide/installation)** - Step-by-step installation for Windows, Linux, macOS, and Docker
- **[⚡ Quick Start Tutorial](guide/quick-start)** - Learn basic DNS queries, ASN enumeration, and advanced techniques
- **[📖 CLI Reference](api/cli-reference)** - Complete command documentation with examples

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

## 📖 Usage Examples

### 🔍 Basic DNS Intelligence
```bash
# Query A records for a domain
rdnsx query example.com

# Comprehensive record enumeration
rdnsx query example.com --record-type A --record-type AAAA --record-type MX --record-type TXT --record-type NS

# Batch processing from file
rdnsx query --list domains.txt --json --silent
```

### 🏢 ASN Network Analysis
```bash
# Discover Google ASN information and IP ranges
rdnsx enumerate --technique asn-enumeration --target AS15169

# Analyze Amazon Web Services network
rdnsx enumerate --technique asn-enumeration --target 16509

# Cloudflare network intelligence
rdnsx enumerate --technique asn-enumeration --target AS13335
```

### 🔄 Advanced Reverse DNS Operations
```bash
# PTR lookup using ASN data (automatic IP discovery)
rdnsx ptr AS15169

# Large-scale reverse DNS analysis with concurrency
rdnsx ptr 10.0.0.0/8 --threads 50

# Targeted IP range analysis
rdnsx ptr 192.168.1.0/24
```

### 🎯 Subdomain Discovery
```bash
# Comprehensive subdomain enumeration
rdnsx bruteforce --domain example.com --wordlist wordlist.txt

# Advanced enumeration with wildcard filtering
rdnsx bruteforce --domain example.com --wordlist wordlist.txt --wildcard --concurrency 100
```

### 🛡️ Security & Compliance Analysis
```bash
# Complete DNS security assessment
rdnsx enumerate --technique comprehensive --target example.com

# DNSSEC configuration analysis
rdnsx enumerate --technique dnssec-enumeration --target example.com

# Email security validation (SPF, DMARC, DKIM)
rdnsx enumerate --technique email-security --target example.com
```

### 📊 Database Integration & Reporting
```bash
# Export results to Elasticsearch
rdnsx query example.com --export-elasticsearch

# Export to MongoDB with custom batching
rdnsx enumerate --technique comprehensive --target example.com --export-mongodb --batch-size 500

# JSON output for custom processing
rdnsx enumerate --technique asn-enumeration --target AS15169 --json > asn-report.json
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](https://github.com/Quinas-Project/RDNSx/blob/main/CONTRIBUTING.md) for details.

## 📄 License

RDNSx is released under the MIT License. See the [LICENSE](https://github.com/Quinas-Project/RDNSx/blob/main/LICENSE) file for details.

---

**Built by [Quinas Project](https://github.com/Quinas-Project)**