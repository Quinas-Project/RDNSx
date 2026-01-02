---
layout: page
title: Quick Start
description: "Get started with RDNSx in minutes"
permalink: /guide/quick-start/
---

# Quick Start

This guide will get you up and running with RDNSx in just a few minutes.

## Basic DNS Queries

### Query A Records (Default)
```bash
rdnsx query example.com
```

Output:
```
██████╗ ██████╗ ███╗   ██╗███████╗██╗  ██╗
██╔══██╗██╔══██╗████╗  ██║██╔════╝╚██╗██╔╝
██████╔╝██║  ██║██╔██╗ ██║███████╗ ╚███╔╝
██╔══██╗██║  ██║██║╚██╗██║╚════██║ ██╔██╗
██║  ██║██████╔╝██║ ╚████║███████║██╔╝ ██╗
╚═╝  ╚═╝╚═════╝ ╚═╝  ╚═══╝╚══════╝╚═╝  ╚═╝

          Quinas Project by RFS
    Fast and multi-purpose DNS toolkit

example.com 300 IN A 93.184.216.34
```

### Query Multiple Record Types
```bash
rdnsx query example.com --record-type A --record-type MX
```

### Query All Record Types
```bash
rdnsx query example.com --record-type A --record-type AAAA --record-type MX --record-type TXT --record-type NS
```

## Working with Multiple Domains

### From Command Line
```bash
rdnsx query example.com google.com github.com
```

### From File
Create a file with domains:
```bash
echo -e "example.com\ngoogle.com\ngithub.com" > domains.txt
rdnsx query --list domains.txt
```

### From Standard Input
```bash
cat domains.txt | rdnsx query
```

## Subdomain Enumeration

### Basic Bruteforce
```bash
# Create a wordlist
echo -e "www\napi\nmail\ndev\nstaging" > wordlist.txt

# Run bruteforce
rdnsx bruteforce --domain example.com --wordlist wordlist.txt
```

## Reverse DNS Lookups

### PTR Lookup for IP Range
```bash
rdnsx ptr 192.168.1.0/24
```

### ASN Lookup
```bash
rdnsx ptr --asn AS15169
```

## Output Formats

### JSON Output
```bash
rdnsx query example.com --json
```

### Silent Mode (Minimal Output)
```bash
rdnsx query example.com --silent
```

### Response Values Only
```bash
rdnsx query example.com --resp-only
```

## Configuration

### Create Default Config
```bash
rdnsx --create-config rdnsx.toml
```

### Use Custom Config
```bash
rdnsx --config my-config.toml query example.com
```

## Performance Tuning

### Increase Concurrency
```bash
rdnsx query --threads 100 domains.txt
```

### Adjust Timeout
```bash
rdnsx query --timeout 10 domains.txt
```

### Rate Limiting
```bash
rdnsx query --rate-limit 50 domains.txt
```

## Next Steps

Now that you know the basics, explore:

- [DNS Record Types](dns-records) - Learn about all supported record types
- [CLI Reference](../api/cli-reference) - Complete command documentation
- [Configuration](configuration) - Advanced configuration options
- [Database Exports](exports) - Export results to databases