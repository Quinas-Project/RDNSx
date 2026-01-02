# Quick Start Guide

This guide will get you up and running with RDNSx in minutes.

## Basic DNS Queries

### Simple Domain Resolution

```bash
# Resolve A records (IPv4 addresses)
rdnsx query example.com

# Resolve multiple record types
rdnsx query example.com --a --aaaa --mx --txt
```

### Output Formats

```bash
# JSON output for scripting
rdnsx query example.com --json

# Silent mode (no progress indicators)
rdnsx query example.com --silent

# Only show response data (no headers)
rdnsx query example.com --resp-only
```

## Advanced Queries

### Multiple Domains

```bash
# Query multiple domains from file
rdnsx query -l domains.txt

# Query from stdin
cat domains.txt | rdnsx query -

# Query with wildcards
rdnsx query "*.example.com"
```

### Custom Resolvers

```bash
# Use custom DNS resolvers
rdnsx query example.com --resolver 8.8.8.8 --resolver 1.1.1.1

# Use system resolvers
rdnsx query example.com --system-resolver
```

### Record Type Specific Queries

```bash
# DNSKEY records (DNSSEC)
rdnsx query example.com --dnskey

# CAA records (Certificate Authority Authorization)
rdnsx query example.com --caa

# HTTPS records (HTTP Service)
rdnsx query example.com --https

# SVCB records (Service Binding)
rdnsx query example.com --svcb
```

## Database Export

### Elasticsearch Export

```bash
# Export to Elasticsearch
rdnsx query example.com \
  --elasticsearch http://localhost:9200 \
  --elasticsearch-index dns-records

# With authentication
rdnsx query example.com \
  --elasticsearch https://user:pass@localhost:9200 \
  --elasticsearch-index dns-records
```

### MongoDB Export

```bash
# Export to MongoDB
rdnsx query example.com \
  --mongodb mongodb://localhost:27017 \
  --mongodb-database dns \
  --mongodb-collection records
```

### Cassandra Export

```bash
# Export to Cassandra
rdnsx query example.com \
  --cassandra 127.0.0.1:9042 \
  --cassandra-keyspace dnsx \
  --cassandra-table records
```

## Subdomain Enumeration

### Wordlist-Based Bruteforcing

```bash
# Basic subdomain enumeration
rdnsx bruteforce example.com --wordlist subdomains.txt

# With custom wordlist from stdin
cat wordlist.txt | rdnsx bruteforce example.com --wordlist -

# With multiple record types
rdnsx bruteforce example.com --wordlist subdomains.txt --a --aaaa --cname
```

### Common Wordlists

```bash
# Download common wordlists
curl -s https://raw.githubusercontent.com/danielmiessler/SecLists/master/Discovery/DNS/subdomains-top1million-5000.txt -o subdomains.txt

# Use with RDNSx
rdnsx bruteforce example.com --wordlist subdomains.txt
```

## Reverse DNS (PTR)

### IP Range Lookup

```bash
# Single IP address
rdnsx ptr 8.8.8.8

# IP range (CIDR notation)
rdnsx ptr 192.168.1.0/24

# Multiple IPs from file
rdnsx ptr -l ips.txt
```

## Advanced Configuration

### Concurrency Control

```bash
# High concurrency for fast networks
rdnsx query -l domains.txt --concurrency 1000

# Lower concurrency for rate-limited environments
rdnsx query -l domains.txt --concurrency 10
```

### Timeout Configuration

```bash
# Short timeout for fast reconnaissance
rdnsx query -l domains.txt --timeout 2

# Longer timeout for unreliable networks
rdnsx query -l domains.txt --timeout 10
```

### Wildcard Detection

```bash
# Enable wildcard filtering
rdnsx query -l domains.txt --wildcard-filter

# Specify wildcard domain
rdnsx query -l domains.txt --wildcard-filter --wildcard-domain example.com
```

## Example Workflows

### Comprehensive Domain Analysis

```bash
# Comprehensive analysis pipeline
rdnsx query example.com \
  --a --aaaa --cname --mx --txt --srv --caa --dnskey \
  --json \
  --elasticsearch http://localhost:9200 \
  --elasticsearch-index comprehensive-scan
```

### Subdomain Discovery Pipeline

```bash
# Discover subdomains and analyze them
rdnsx bruteforce example.com --wordlist subdomains.txt --silent | \
  rdnsx query - --a --aaaa --cname --json | \
  jq '.[] | select(.value != null) | .domain' | \
  sort | uniq
```

### Security Audit Pipeline

```bash
# Security-focused DNS audit
rdnsx query example.com \
  --dnskey --ds --rrsig --nsec --nsec3 \
  --caa --tlsa --sshfp \
  --json \
  --output security-audit.json
```

## Next Steps

- Learn about all supported [DNS record types](./dns-records.md)
- Explore [database export options](./exports.md)
- Check [advanced usage patterns](./advanced-usage.md)
- See the complete [CLI reference](../api/cli-reference.md)