# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial Rust rewrite of DNSx with all original features
- Async/await DNS resolution with tokio
- Support for all DNS record types (A, AAAA, CNAME, MX, TXT, NS, SOA, PTR, SRV)
- Custom DNS resolvers with failover and load balancing
- Subdomain bruteforcing with wordlist support
- Wildcard DNS detection and filtering
- Reverse DNS (PTR) queries for IP ranges
- Export to Elasticsearch with bulk indexing
- Export to MongoDB with bulk inserts
- CLI interface with comprehensive options
- Library API for embedding in other Rust projects
- GitHub Actions CI/CD pipeline
- Comprehensive documentation and examples

### Technical Improvements
- Zero unsafe code blocks (pure safe Rust)
- Type-safe DNS record handling
- Efficient memory usage with streaming I/O
- Configurable concurrency limits
- Connection pooling and reuse
- Batch processing for database exports
- Structured error handling with thiserror
- Comprehensive test coverage