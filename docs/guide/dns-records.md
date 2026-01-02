# DNS Record Types

RDNSx supports comprehensive DNS record type coverage with 27 different record types.

## Address Records

### A (Address)
Maps a hostname to IPv4 address.
```bash
rdnsx query example.com --a
```
**Example Output:**
```
example.com.  3600  IN  A  93.184.216.34
```

### AAAA (IPv6 Address)
Maps a hostname to IPv6 address.
```bash
rdnsx query example.com --aaaa
```
**Example Output:**
```
example.com.  3600  IN  AAAA  2606:2800:220:1:248:1893:25c8:1946
```

## Name Resolution Records

### CNAME (Canonical Name)
Aliases one name to another.
```bash
rdnsx query www.example.com --cname
```
**Example Output:**
```
www.example.com.  3600  IN  CNAME  example.com.
```

### PTR (Pointer)
Reverse DNS lookup (IP to hostname).
```bash
rdnsx ptr 8.8.8.8
```
**Example Output:**
```
8.8.8.8.in-addr.arpa.  86400  IN  PTR  dns.google.
```

## Mail Records

### MX (Mail Exchange)
Specifies mail servers for the domain.
```bash
rdnsx query example.com --mx
```
**Example Output:**
```
example.com.  3600  IN  MX  10 mail.example.com.
```

## Text Records

### TXT (Text)
Arbitrary text data associated with a name.
```bash
rdnsx query example.com --txt
```
**Example Output:**
```
example.com.  3600  IN  TXT  "v=spf1 -all"
```

### SPF (Sender Policy Framework)
Legacy TXT-based SPF records (use TXT instead).
```bash
rdnsx query example.com --txt | grep spf
```

## Service Records

### SRV (Service Location)
Specifies location of services.
```bash
rdnsx query _sip._tcp.example.com --srv
```
**Example Output:**
```
_sip._tcp.example.com.  3600  IN  SRV  10 5 5060 sip.example.com.
```

### SVCB/HTTPS (Service Binding)
Modern service binding records.
```bash
rdnsx query example.com --svcb --https
```
**Example Output:**
```
example.com.  3600  IN  HTTPS  1 . alpn="h2,h3"
```

## DNSSEC Records

### DNSKEY (DNS Key)
Public key for DNSSEC signing.
```bash
rdnsx query example.com --dnskey
```
**Example Output:**
```
example.com.  86400  IN  DNSKEY  257 3 13 (...)
```

### DS (Delegation Signer)
Delegation signer record.
```bash
rdnsx query example.com --ds
```
**Example Output:**
```
example.com.  86400  IN  DS  12345 13 2 (...)
```

### RRSIG (DNSSEC Signature)
Digital signature for DNSSEC.
```bash
rdnsx query example.com --rrsig
```

### NSEC (Next Secure)
Proof of non-existence for DNSSEC.
```bash
rdnsx query example.com --nsec
```

### NSEC3 (Next Secure v3)
Hashed next secure record.
```bash
rdnsx query example.com --nsec3
```

## Security Records

### CAA (Certificate Authority Authorization)
Restricts which CAs can issue certificates.
```bash
rdnsx query example.com --caa
```
**Example Output:**
```
example.com.  3600  IN  CAA  0 issue "letsencrypt.org"
```

### TLSA (TLS Authentication)
Associates certificates with domain names.
```bash
rdnsx query _443._tcp.example.com --tlsa
```
**Example Output:**
```
_443._tcp.example.com.  3600  IN  TLSA  3 0 1 (...)
```

### SSHFP (SSH Fingerprint)
SSH host key fingerprints.
```bash
rdnsx query example.com --sshfp
```
**Example Output:**
```
example.com.  3600  IN  SSHFP  1 1 (...)
```

## Informational Records

### SOA (Start of Authority)
Administrative information about the zone.
```bash
rdnsx query example.com --soa
```
**Example Output:**
```
example.com.  3600  IN  SOA  ns1.example.com. admin.example.com. (...) 3600 1800 604800 86400
```

### HINFO (Host Information)
Host hardware/software information.
```bash
rdnsx query example.com --hinfo
```
**Example Output:**
```
example.com.  3600  IN  HINFO  "Intel" "Linux"
```

### LOC (Location)
Geographic location information.
```bash
rdnsx query example.com --loc
```

## Specialized Records

### NAPTR (Naming Authority Pointer)
Regular expression based rewriting.
```bash
rdnsx query example.com --naptr
```

### CERT (Certificate)
Certificate storage in DNS.
```bash
rdnsx query example.com --cert
```

### DNAME (Delegation Name)
Non-terminal redirection.
```bash
rdnsx query example.com --dname
```

### URI (Uniform Resource Identifier)
URI storage in DNS.
```bash
rdnsx query example.com --uri
```

### KEY (Key)
Public key storage (legacy).
```bash
rdnsx query example.com --key
```

### OPT (Option)
EDNS options (usually queried automatically).

## Name Server Records

### NS (Name Server)
Authoritative name servers for the domain.
```bash
rdnsx query example.com --ns
```
**Example Output:**
```
example.com.  3600  IN  NS  ns1.example.com.
example.com.  3600  IN  NS  ns2.example.com.
```

### AFSDB (AFS Database)
AFS cell database location (legacy).
```bash
rdnsx query example.com --afsdb
```

## Querying Multiple Record Types

### All Records
```bash
# Query all supported record types
rdnsx query example.com --a --aaaa --cname --mx --txt --ns --soa --srv --caa --dnskey --ds --hinfo --https --loc --naptr --nsec --nsec3 --opt --rrsig --sshfp --svcb --tlsa --uri
```

### Security Audit
```bash
# Security-focused record types
rdnsx query example.com --dnskey --ds --rrsig --nsec --nsec3 --caa --tlsa --sshfp
```

### Service Discovery
```bash
# Service location records
rdnsx query example.com --srv --svcb --https --naptr
```

## Record Type Reference

| Record Type | Purpose | Example Use Case |
|-------------|---------|------------------|
| A | IPv4 address resolution | Basic domain resolution |
| AAAA | IPv6 address resolution | Modern network support |
| CNAME | Name aliasing | www → main domain |
| PTR | Reverse DNS | IP → hostname lookup |
| MX | Mail server location | Email delivery |
| TXT | Text data | SPF, DKIM, verification |
| SRV | Service location | SIP, XMPP services |
| SVCB/HTTPS | Modern service binding | HTTP/3, alt-svc |
| DNSKEY | DNSSEC public keys | Zone signing verification |
| DS | Delegation signing | DNSSEC chain of trust |
| RRSIG | DNSSEC signatures | Data integrity |
| NSEC/NSEC3 | DNSSEC proof of non-existence | Zone enumeration prevention |
| CAA | Certificate authority restrictions | Certificate issuance control |
| TLSA | TLS certificate association | DANE (DNS-based Authentication of Named Entities) |
| SSHFP | SSH key fingerprints | SSH host verification |
| SOA | Zone administrative data | Zone transfer information |
| HINFO | Host information | Hardware/software details |
| LOC | Geographic location | Host positioning |
| NAPTR | Name rewriting | E.164 number resolution |
| CERT | Certificate storage | PKI in DNS |
| DNAME | Non-terminal redirection | Zone delegation |
| URI | URI storage | Resource discovery |
| KEY | Public key storage | Legacy key distribution |
| OPT | EDNS options | Extended DNS features |
| NS | Name server delegation | Domain authority |
| AFSDB | AFS database location | Legacy distributed filesystem |

## Troubleshooting

### Record Not Found
Some domains may not have all record types configured. This is normal.

### DNSSEC Validation
For DNSSEC-enabled domains, query DNSKEY, DS, and RRSIG records to verify the chain of trust.

### Service Discovery
Use SRV and SVCB/HTTPS records to discover available services for a domain.

### Security Considerations
Regularly audit CAA records to ensure only authorized CAs can issue certificates for your domain.