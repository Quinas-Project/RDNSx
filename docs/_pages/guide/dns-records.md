---
layout: page
title: DNS Record Types
description: "Complete guide to DNS record types supported by RDNSx"
permalink: /guide/dns-records/
---

# DNS Record Types

RDNSx supports 27 DNS record types, providing comprehensive DNS enumeration capabilities.

## Basic Record Types

### A (Address)
Maps a hostname to an IPv4 address.
```bash
rdnsx query example.com --a
```
Output: `example.com 300 IN A 93.184.216.34`

### AAAA (IPv6 Address)
Maps a hostname to an IPv6 address.
```bash
rdnsx query example.com --aaaa
```
Output: `example.com 300 IN AAAA 2606:2800:220:1:248:1893:25c8:1946`

### CNAME (Canonical Name)
Aliases one hostname to another.
```bash
rdnsx query www.example.com --cname
```

## Mail Records

### MX (Mail Exchange)
Specifies mail servers for the domain.
```bash
rdnsx query example.com --mx
```
Output includes priority and mail server hostname.

### TXT (Text)
Contains arbitrary text data, commonly used for SPF, DKIM, DMARC.
```bash
rdnsx query example.com --txt
```

## Name Server Records

### NS (Name Server)
Lists authoritative name servers for the domain.
```bash
rdnsx query example.com --ns
```

### SOA (Start of Authority)
Contains administrative information about the zone.
```bash
rdnsx query example.com --soa
```
Includes serial number, refresh interval, etc.

## Service Records

### SRV (Service)
Specifies location of services (SIP, XMPP, etc.).
```bash
rdnsx query _sip._tcp.example.com --srv
```

### PTR (Pointer)
Maps IP addresses to hostnames (reverse DNS).
```bash
rdnsx ptr 1.2.3.4
```

## Security Records

### CAA (Certification Authority Authorization)
Specifies which CAs can issue certificates for the domain.
```bash
rdnsx query example.com --caa
```

### CERT (Certificate)
Contains certificates or certificate revocation lists.
```bash
rdnsx query example.com --cert
```

### DNSKEY (DNS Key)
Public key material for DNSSEC.
```bash
rdnsx query example.com --dnskey
```

### DS (Delegation Signer)
Contains hash of DNSKEY record from child zone.
```bash
rdnsx query example.com --ds
```

### RRSIG (DNSSEC Signature)
Contains DNSSEC signature for a record set.
```bash
rdnsx query example.com --rrsig
```

## Modern Records

### HTTPS (HTTPS Service)
Provides HTTPS service endpoint information.
```bash
rdnsx query example.com --https
```

### SVCB (Service Binding)
Generalized service binding (successor to SRV).
```bash
rdnsx query example.com --svcb
```

### TLSA (TLS Certificate Association)
Associates certificates with TLS services.
```bash
rdnsx query _443._tcp.example.com --tlsa
```

## Legacy Records

### HINFO (Host Information)
Host hardware and software information.
```bash
rdnsx query example.com --hinfo
```

### KEY (Key)
Public key for the domain.
```bash
rdnsx query example.com --key
```

### LOC (Location)
Geographic location of the domain.
```bash
rdnsx query example.com --loc
```

## Experimental Records

### NAPTR (Naming Authority Pointer)
Regular expression based rewriting.
```bash
rdnsx query example.com --naptr
```

### NSEC (Next Secure)
Proof of non-existence in DNSSEC.
```bash
rdnsx query example.com --nsec
```

### NSEC3 (Next Secure v3)
Hashed proof of non-existence.
```bash
rdnsx query example.com --nsec3
```

### OPT (Option)
EDNS options (usually not queried directly).
```bash
rdnsx query example.com --opt
```

### SSHFP (SSH FingerPrint)
SSH host key fingerprints.
```bash
rdnsx query example.com --sshfp
```

### URI (Uniform Resource Identifier)
URI resource records.
```bash
rdnsx query example.com --uri
```

## Usage Examples

### Comprehensive Domain Analysis
```bash
rdnsx query example.com \
  --a --aaaa --cname --mx --txt --ns --soa \
  --srv --caa --dnskey --https --svcb
```

### Security Audit
```bash
rdnsx query example.com \
  --dnskey --ds --rrsig --tlsa --caa --txt
```

### Mail Server Discovery
```bash
rdnsx query example.com --mx --spf --dkim --dmarc
```

### Service Discovery
```bash
# SIP servers
rdnsx query _sip._tcp.example.com --srv --tlsa

# XMPP servers
rdnsx query _xmpp-client._tcp.example.com --srv
```

## Record Type Flags

All record types can be queried using either:
- Individual flags: `--a --aaaa --mx`
- Generic flag: `--record-type A --record-type AAAA --record-type MX`

The generic flag allows for programmatic usage and supports all record types.