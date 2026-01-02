# CLI Reference

Complete command-line interface documentation for RDNSx.

## Global Options

### Output Control
```bash
-o, --output <FILE>          Write output to file instead of stdout
-j, --json                   Output in JSON format
-s, --silent                 Silent mode (suppress progress indicators)
--resp-only                  Show only response data (no headers)
--no-color                   Disable colored output
```

### Performance
```bash
-c, --concurrency <NUM>      Number of concurrent workers (default: 100)
-t, --timeout <SECONDS>      Query timeout in seconds (default: 5)
-r, --retries <NUM>          Number of retries per query (default: 3)
```

### Resolver Configuration
```bash
--resolver <IP>              Custom DNS resolver (can be used multiple times)
--system-resolver            Use system DNS configuration
--backup-resolver <IP>       Backup DNS resolver for failover
```

### Wildcard Detection
```bash
--wildcard-filter            Enable wildcard DNS filtering
--wildcard-domain <DOMAIN>   Domain for wildcard detection
--wildcard-threshold <NUM>   Threshold for wildcard detection (default: 5)
```

## Commands

## `query` - DNS Query

Query DNS records for domains.

### Usage
```bash
rdnsx query [OPTIONS] <DOMAIN>...
rdnsx query [OPTIONS] -l <FILE>
rdnsx query [OPTIONS] -
```

### Record Type Flags

#### Address Records
```bash
--a                          Query A records (IPv4 addresses)
--aaaa                       Query AAAA records (IPv6 addresses)
```

#### Name Resolution
```bash
--cname                      Query CNAME records (canonical names)
--ptr                        Query PTR records (reverse DNS)
--ns                         Query NS records (name servers)
```

#### Mail Records
```bash
--mx                         Query MX records (mail exchangers)
```

#### Text Records
```bash
--txt                        Query TXT records (text data)
```

#### Service Records
```bash
--srv                        Query SRV records (service location)
--svcb                       Query SVCB records (service binding)
--https                      Query HTTPS records (HTTP service)
```

#### DNSSEC Records
```bash
--dnskey                     Query DNSKEY records (DNSSEC keys)
--ds                         Query DS records (delegation signer)
--rrsig                      Query RRSIG records (DNSSEC signatures)
--nsec                       Query NSEC records (next secure)
--nsec3                      Query NSEC3 records (hashed next secure)
```

#### Security Records
```bash
--caa                        Query CAA records (certificate authorities)
--tlsa                       Query TLSA records (TLS authentication)
--sshfp                      Query SSHFP records (SSH fingerprints)
--cert                       Query CERT records (certificates)
```

#### Informational Records
```bash
--soa                        Query SOA records (start of authority)
--hinfo                      Query HINFO records (host information)
--loc                        Query LOC records (location)
```

#### Specialized Records
```bash
--naptr                      Query NAPTR records (name authority pointer)
--dname                      Query DNAME records (delegation name)
--uri                        Query URI records (uniform resource identifier)
--key                        Query KEY records (public keys)
--afsdb                      Query AFSDB records (AFS database)
--opt                        Query OPT records (EDNS options)
```

### Database Export Options

#### Elasticsearch
```bash
--elasticsearch <URL>        Elasticsearch URL (e.g., http://localhost:9200)
--elasticsearch-index <NAME> Index name (default: dnsx)
--elasticsearch-username <USER> Elasticsearch username
--elasticsearch-password <PASS> Elasticsearch password
```

#### MongoDB
```bash
--mongodb <URL>              MongoDB connection URL (e.g., mongodb://localhost:27017)
--mongodb-database <NAME>    Database name (default: dnsx)
--mongodb-collection <NAME>  Collection name (default: records)
--mongodb-username <USER>    MongoDB username
--mongodb-password <PASS>    MongoDB password
```

#### Cassandra
```bash
--cassandra <HOST:PORT>      Cassandra contact points (comma-separated)
--cassandra-keyspace <NAME>  Keyspace name (default: dnsx)
--cassandra-table <NAME>     Table name (default: records)
--cassandra-username <USER>  Cassandra username
--cassandra-password <PASS>  Cassandra password
```

### Examples

```bash
# Basic A record lookup
rdnsx query example.com

# Multiple record types
rdnsx query example.com --a --aaaa --mx --txt

# From file input
rdnsx query -l domains.txt --a --aaaa

# JSON output with Elasticsearch export
rdnsx query example.com --json --elasticsearch http://localhost:9200

# Security audit
rdnsx query example.com --dnskey --ds --caa --tlsa

# High-performance scanning
rdnsx query -l domains.txt --concurrency 1000 --timeout 2
```

## `bruteforce` - Subdomain Enumeration

Perform subdomain bruteforcing using wordlists.

### Usage
```bash
rdnsx bruteforce [OPTIONS] <DOMAIN> --wordlist <WORDLIST>
```

### Options
```bash
-w, --wordlist <FILE>        Wordlist file path or '-' for stdin
--wordlist-words <WORDS>     Comma-separated wordlist
```

### Examples

```bash
# Basic subdomain enumeration
rdnsx bruteforce example.com --wordlist subdomains.txt

# From stdin
cat subdomains.txt | rdnsx bruteforce example.com --wordlist -

# With specific record types
rdnsx bruteforce example.com --wordlist subdomains.txt --a --aaaa --cname

# Export results
rdnsx bruteforce example.com --wordlist subdomains.txt --mongodb mongodb://localhost:27017
```

## `ptr` - Reverse DNS Lookup

Perform reverse DNS lookups for IP addresses.

### Usage
```bash
rdnsx ptr [OPTIONS] <IP>...
rdnsx ptr [OPTIONS] -l <FILE>
rdnsx ptr [OPTIONS] -
```

### Options
```bash
--ip-range <RANGE>           IP range in CIDR notation (e.g., 192.168.1.0/24)
--asn <ASN>                  Autonomous System Number lookup
```

### Examples

```bash
# Single IP lookup
rdnsx ptr 8.8.8.8

# IP range
rdnsx ptr --ip-range 192.168.1.0/24

# From file
rdnsx ptr -l ips.txt

# ASN lookup
rdnsx ptr --asn AS15169
```

## Configuration Files

### Global Configuration

Create `~/.rdnsx/config.toml`:

```toml
[default]
resolvers = ["8.8.8.8", "1.1.1.1", "9.9.9.9"]
concurrency = 100
timeout = 5
retries = 3

[export]
batch_size = 1000
elasticsearch_url = "http://localhost:9200"
elasticsearch_index = "dnsx"

[mongodb]
url = "mongodb://localhost:27017"
database = "dnsx"
collection = "records"

[cassandra]
contact_points = ["127.0.0.1:9042"]
keyspace = "dnsx"
table = "records"
username = "cassandra"
password = "cassandra"
```

### Environment Variables

```bash
# Default resolvers
export RDNSX_RESOLVERS="8.8.8.8,1.1.1.1"

# Performance settings
export RDNSX_CONCURRENCY=100
export RDNSX_TIMEOUT=5

# Database connections
export RDNSX_ELASTICSEARCH_URL="http://localhost:9200"
export RDNSX_MONGODB_URL="mongodb://localhost:27017"
```

## Exit Codes

- `0`: Success
- `1`: General error
- `2`: Configuration error
- `3`: Network error
- `4`: Authentication error

## Error Handling

RDNSx provides detailed error messages for troubleshooting:

### Common Errors

#### Network Errors
```
Error: DNS query timeout for domain.com
Cause: Network connectivity issues or unresponsive resolver
Solution: Check network connection or use different resolvers
```

#### Authentication Errors
```
Error: Elasticsearch authentication failed
Cause: Invalid credentials or missing permissions
Solution: Verify username/password and permissions
```

#### Configuration Errors
```
Error: Invalid resolver format: invalid.ip.format
Cause: Malformed IP address or hostname
Solution: Use valid IP addresses (e.g., 8.8.8.8) or hostnames
```

## Performance Tuning

### Memory Usage
- Batch size affects memory consumption
- Higher concurrency increases memory usage
- Use `--silent` mode to reduce memory for progress tracking

### Network Optimization
- Use multiple resolvers for redundancy
- Adjust timeouts based on network latency
- Enable wildcard filtering for large datasets

### Database Performance
- Batch size of 1000 is optimal for most databases
- Use connection pooling for high-throughput scenarios
- Monitor database performance during large exports

## Examples

### Complete Reconnaissance Pipeline

```bash
# 1. Discover subdomains
rdnsx bruteforce example.com --wordlist subdomains.txt --silent > subdomains.txt

# 2. Resolve all discovered domains
rdnsx query -l subdomains.txt --a --aaaa --cname --mx --txt --silent > results.txt

# 3. Security audit
rdnsx query -l subdomains.txt --dnskey --ds --caa --tlsa --json > security.json

# 4. Export to multiple databases
rdnsx query -l subdomains.txt \
  --elasticsearch http://localhost:9200 \
  --mongodb mongodb://localhost:27017 \
  --cassandra 127.0.0.1:9042
```

### Monitoring and Alerting

```bash
# Continuous monitoring with JSON output
rdnsx query -l domains.txt --json --silent | jq '.[] | select(.response_code != "NOERROR")'

# Export to time-series database
rdnsx query example.com --json --elasticsearch http://localhost:9200 --elasticsearch-index dns-monitoring-$(date +%Y-%m-%d)
```

### CI/CD Integration

```yaml
# .github/workflows/dns-audit.yml
name: DNS Security Audit
on:
  schedule:
    - cron: '0 2 * * *'  # Daily at 2 AM

jobs:
  audit:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    - name: Install RDNSx
      run: cargo install rdnsx
    - name: DNS Security Audit
      run: |
        rdnsx query example.com --dnskey --ds --caa --tlsa --json > audit.json
    - name: Upload Results
      uses: actions/upload-artifact@v4
      with:
        name: dns-audit
        path: audit.json
```