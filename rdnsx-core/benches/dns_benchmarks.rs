//! Performance benchmarks for DNS operations

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rdnsx_core::{client::DnsxClient, config::DnsxOptions, types::RecordType};
use std::time::Duration;

fn bench_dns_query(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let client = rt.block_on(async {
        DnsxClient::with_options(DnsxOptions::default()).unwrap()
    });

    c.bench_function("dns_query_a_record", |b| {
        b.to_async(&rt).iter(|| async {
            black_box(client.query("google.com", RecordType::A).await.unwrap());
        });
    });

    c.bench_function("dns_lookup_ipv4", |b| {
        b.to_async(&rt).iter(|| async {
            black_box(client.lookup_ipv4("google.com").await.unwrap());
        });
    });
}

fn bench_resolver_pool_creation(c: &mut Criterion) {
    c.bench_function("resolver_pool_creation", |b| {
        b.iter(|| {
            black_box(DnsxOptions::default());
        });
    });
}

fn bench_record_parsing(c: &mut Criterion) {
    use hickory_resolver::proto::rr::rdata::{A, AAAA, CNAME, MX, TXT, SRV, SOA};
    use std::net::Ipv4Addr;

    // Create some test RData
    let a_record = A::new(Ipv4Addr::new(192, 168, 1, 1));
    let txt_record = TXT::new(vec![b"test value".to_vec()]);

    c.bench_function("parse_a_record", |b| {
        b.iter(|| {
            black_box(crate::query::parse_rdata(
                &hickory_resolver::proto::rr::RData::A(a_record),
                RecordType::A,
            ).unwrap());
        });
    });

    c.bench_function("parse_txt_record", |b| {
        b.iter(|| {
            black_box(crate::query::parse_rdata(
                &hickory_resolver::proto::rr::RData::TXT(txt_record.clone()),
                RecordType::Txt,
            ).unwrap());
        });
    });
}

fn bench_concurrent_queries(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let client = rt.block_on(async {
        DnsxClient::with_options(DnsxOptions::default()).unwrap()
    });

    let domains = vec![
        "google.com",
        "cloudflare.com",
        "github.com",
        "stackoverflow.com",
        "rust-lang.org",
    ];

    c.bench_function("concurrent_queries_5_domains", |b| {
        b.to_async(&rt).iter(|| async {
            let mut handles = Vec::new();
            for domain in &domains {
                let client = &client;
                let domain = *domain;
                let handle = tokio::spawn(async move {
                    black_box(client.query(domain, RecordType::A).await.unwrap());
                });
                handles.push(handle);
            }

            for handle in handles {
                black_box(handle.await.unwrap());
            }
        });
    });
}

fn configure_criterion() -> Criterion {
    Criterion::default()
        .measurement_time(Duration::from_secs(10))
        .sample_size(100)
        .warm_up_time(Duration::from_secs(1))
}

criterion_group! {
    name = benches;
    config = configure_criterion();
    targets = bench_dns_query, bench_resolver_pool_creation, bench_record_parsing, bench_concurrent_queries
}

criterion_main!(benches);