---
layout: home
title: "RDNSx Documentation - High-Performance DNS Toolkit in Rust"
description: "Complete guide to RDNSx, the fastest DNS enumeration toolkit written in Rust. Master DNS reconnaissance, ASN analysis, reverse DNS lookups, subdomain enumeration, and security research tools. Perfect for cybersecurity professionals and network administrators."
keywords: "RDNSx, DNS toolkit, Rust DNS scanner, network enumeration, ASN lookup, reverse DNS, subdomain enumeration, DNSSEC analysis, security research tools, network reconnaissance, command line DNS, DNS record types, IPv6 enumeration, email security analysis, CDN detection, zone transfer, passive DNS, server fingerprinting, DNS bruteforce, Rust CLI tools, network security, penetration testing, red team tools, cybersecurity, network analysis, domain intelligence, DNS enumeration tools, network mapping, security assessment"
og_image: /assets/images/logo.svg
twitter_card: summary_large_image
author: "Quinas Project"
lang: en-US
permalink: /
priority: 1.0
changefreq: weekly
reading_time: 8
---

# 🚀 RDNSx Documentation

**Master DNS reconnaissance with the fastest toolkit written in Rust**

RDNSx is a **high-performance DNS enumeration toolkit** written in Rust, designed for cybersecurity professionals, network administrators, and developers. Built with async programming and memory safety, RDNSx delivers enterprise-grade DNS analysis tools with exceptional speed and reliability.

## 🎯 What is RDNSx?

RDNSx represents the next generation of DNS reconnaissance tools, combining **Rust's performance** with **comprehensive security research capabilities**. Whether you're conducting penetration testing, network assessments, or building DNS-aware applications, RDNSx provides the industry-leading tools you need for thorough DNS intelligence gathering.

### 🔬 Built for Security Research
- **Zero-cost abstractions** with Rust's ownership system
- **Memory-safe** concurrent processing
- **Type-safe** DNS protocol handling
- **Performance-optimized** for large-scale scanning

## ✨ Core Features & Capabilities

### ⚡ Performance & Speed
- **Lightning-Fast DNS Resolution**: Async/await concurrency with up to 50+ simultaneous queries
- **Memory-Efficient Processing**: Rust's ownership system ensures optimal resource utilization
- **Smart Rate Limiting**: Intelligent throttling prevents blocking and ensures reliable scanning
- **Optimized Algorithms**: Custom DNS protocol implementations for maximum throughput

### 🔍 Comprehensive DNS Analysis
- **Complete Record Type Support**: All 27 DNS record types including A, AAAA, CNAME, MX, TXT, NS, SOA, PTR, SRV, CAA, DNSKEY, DS, RRSIG, NSEC, HTTPS, SVCB, and more
- **ASN Intelligence Gathering**: Discover IP ranges and network infrastructure for major organizations (Google, Amazon, Cloudflare, Microsoft)
- **IPv4/IPv6 Dual-Stack**: Full support for modern network infrastructure analysis
- **Reverse DNS Mastery**: Advanced PTR lookups with ASN integration and smart IP range handling

### 🛡️ Advanced Security Research Tools
- **Subdomain Enumeration**: High-performance bruteforce discovery with customizable wordlists
- **DNSSEC Security Analysis**: Comprehensive DNSSEC configuration assessment and zone walking
- **Wildcard Detection & Bypass**: Advanced filtering techniques for accurate reconnaissance
- **Email Security Validation**: SPF, DMARC, DKIM record analysis for email infrastructure assessment
- **CDN Detection**: Identify content delivery networks and analyze configurations
- **Server Fingerprinting**: Advanced DNS server capability analysis

### 📊 Enterprise-Ready Features
- **Multi-Database Export**: Native support for Elasticsearch, MongoDB, and Cassandra
- **Docker Containerization**: Production-ready container images with multi-architecture support
- **Flexible Configuration**: TOML-based configuration system with environment variable support
- **Rust Library API**: Embeddable library for custom applications and integrations
- **Cross-Platform Support**: Native binaries for Linux, macOS, and Windows

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
- [DNS Record Types](guide/dns-records) - Complete reference for all 27 supported record types
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