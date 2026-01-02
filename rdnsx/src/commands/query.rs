//! Query command implementation

use std::sync::Arc;

use anyhow::Result;
use clap::Args;
use rdnsx_core::{DnsxClient, RecordType, ResponseCode, CassandraExporter, ElasticsearchExporter, MongodbExporter, ResolverPool, WildcardFilter, Exporter, config::DnsxOptions};

use crate::cli::Config;
use crate::output_writer::OutputWriter;

#[derive(Args)]
pub struct QueryArgs {
    /// Domains to query
    #[arg(value_name = "DOMAIN")]
    pub domains: Vec<String>,

    /// Input file (default: stdin)
    #[arg(short, long)]
    pub list: Option<String>,

    /// DNS record types to query (can be repeated)
    #[arg(short = 't', long = "record-type", value_name = "TYPE", action = clap::ArgAction::Append)]
    pub record_type: Vec<String>,

    /// A records (default if no record types specified)
    #[arg(short, long)]
    pub a: bool,

    /// AAAA records
    #[arg(long)]
    pub aaaa: bool,

    /// CNAME records
    #[arg(long)]
    pub cname: bool,

    /// MX records
    #[arg(long)]
    pub mx: bool,

    /// TXT records
    #[arg(long)]
    pub txt: bool,

    /// NS records
    #[arg(long)]
    pub ns: bool,

    /// SOA records
    #[arg(long)]
    pub soa: bool,

    /// PTR records
    #[arg(long)]
    pub ptr: bool,

    /// SRV records
    #[arg(long)]
    pub srv: bool,

    /// CAA records
    #[arg(long)]
    pub caa: bool,

    /// CERT records
    #[arg(long)]
    pub cert: bool,

    /// DNAME records
    #[arg(long)]
    pub dname: bool,

    /// DNSKEY records
    #[arg(long)]
    pub dnskey: bool,

    /// DS records
    #[arg(long)]
    pub ds: bool,

    /// HINFO records
    #[arg(long)]
    pub hinfo: bool,

    /// HTTPS records
    #[arg(long)]
    pub https: bool,

    /// KEY records
    #[arg(long)]
    pub key: bool,

    /// LOC records
    #[arg(long)]
    pub loc: bool,

    /// NAPTR records
    #[arg(long)]
    pub naptr: bool,

    /// NSEC records
    #[arg(long)]
    pub nsec: bool,

    /// NSEC3 records
    #[arg(long)]
    pub nsec3: bool,

    /// OPT records
    #[arg(long)]
    pub opt: bool,

    /// RRSIG records
    #[arg(long)]
    pub rrsig: bool,

    /// SSHFP records
    #[arg(long)]
    pub sshfp: bool,

    /// SVCB records
    #[arg(long)]
    pub svcb: bool,

    /// TLSA records
    #[arg(long)]
    pub tlsa: bool,

    /// URI records
    #[arg(long)]
    pub uri: bool,

    /// ASN information
    #[arg(long)]
    pub asn: bool,

    /// Filter by response code (comma-separated)
    #[arg(long)]
    pub rcode: Option<String>,

    /// Domain for wildcard detection
    #[arg(short = 'w', long)]
    pub wildcard_domain: Option<String>,

    /// Response values only
    #[arg(long)]
    pub resp_only: bool,
}

pub async fn run(args: QueryArgs, config: Config) -> Result<()> {
    // Determine record types to query
    let record_types = determine_record_types(&args);

    // Create DNS client
    let dns_options = DnsxOptions {
        resolvers: config.core_config.resolvers.servers.clone(),
        timeout: std::time::Duration::from_secs(config.core_config.resolvers.timeout),
        retries: config.core_config.resolvers.retries,
        concurrency: config.core_config.performance.threads,
        rate_limit: config.core_config.performance.rate_limit,
    };
    let client = DnsxClient::with_options(dns_options.clone())?;

    // Create wildcard filter if domain specified
    let wildcard_filter: Option<WildcardFilter> = if let Some(ref base_domain) = args.wildcard_domain {
        let resolver_pool = Arc::new(ResolverPool::new(&dns_options)?);
        Some(WildcardFilter::new(
            Some(base_domain.clone()),
            resolver_pool,
            10, // Default threshold: 10 domains pointing to same IP
        ))
    } else {
        None
    };

    // Parse response code filter
    let allowed_rcodes = parse_rcodes(&args.rcode)?;

    // Create output writer
    let mut output = OutputWriter::new(config.output_file.clone(), config.json_output, config.silent)?;

    // Create exporters if configured
    let mut es_exporter: Option<ElasticsearchExporter> = None;
    let mut mongo_exporter: Option<MongodbExporter> = None;
    let mut cassandra_exporter: Option<CassandraExporter> = None;

    if config.core_config.export.elasticsearch.enabled {
        es_exporter = Some(
            ElasticsearchExporter::new(
                &config.core_config.export.elasticsearch.url,
                &config.core_config.export.elasticsearch.index,
                config.core_config.export.batch_size,
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create Elasticsearch exporter: {}", e))?,
        );
    }

    if config.core_config.export.mongodb.enabled {
        mongo_exporter = Some(
            MongodbExporter::new(
                &config.core_config.export.mongodb.url,
                &config.core_config.export.mongodb.database,
                &config.core_config.export.mongodb.collection,
                config.core_config.export.batch_size,
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create MongoDB exporter: {}", e))?,
        );
    }

    if config.core_config.export.cassandra.enabled {
        cassandra_exporter = Some(
            CassandraExporter::new(
                &config.core_config.export.cassandra.contact_points,
                Some(&config.core_config.export.cassandra.username),
                Some(&config.core_config.export.cassandra.password),
                &config.core_config.export.cassandra.keyspace,
                &config.core_config.export.cassandra.table,
                config.core_config.export.batch_size,
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create Cassandra exporter: {}", e))?,
        );
    }

    // Read domains from input
    let mut domains = read_domains(&args.list)?;
    // Add positional domains
    domains.extend(args.domains);

    // Query each domain
    for domain in domains {
        for record_type in &record_types {
            match client.query(&domain, *record_type).await {
                Ok(mut records) => {
                    // Filter by response code if specified
                    if !allowed_rcodes.is_empty() {
                        records.retain(|r| allowed_rcodes.contains(&r.response_code));
                    }

                    // Apply wildcard filtering if enabled
                    let filtered_records = if let Some(ref filter) = wildcard_filter {
                        filter.filter(records.clone()).await
                            .unwrap_or_else(|e| {
                                if !config.silent {
                                    eprintln!("Warning: Wildcard filtering failed: {}", e);
                                }
                                records
                            })
                    } else {
                        records
                    };

                    // Output records
                    for record in filtered_records {
                        output.write_record(&record, args.resp_only)?;

                        // Export to Elasticsearch if configured
                        if let Some(ref exporter) = es_exporter {
                            if let Err(e) = exporter.export(record.clone()).await {
                                if !config.silent {
                                    eprintln!("Warning: Failed to export to Elasticsearch: {}", e);
                                }
                            }
                        }

                        // Export to MongoDB if configured
                        if let Some(ref exporter) = mongo_exporter {
                            if let Err(e) = exporter.export(record.clone()).await {
                                if !config.silent {
                                    eprintln!("Warning: Failed to export to MongoDB: {}", e);
                                }
                            }
                        }

                        // Export to Cassandra if configured
                        if let Some(ref exporter) = cassandra_exporter {
                            if let Err(e) = exporter.export(record.clone()).await {
                                if !config.silent {
                                    eprintln!("Warning: Failed to export to Cassandra: {}", e);
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    if !config.silent {
                        eprintln!("Error querying {} ({:?}): {}", domain, record_type, e);
                    }
                }
            }
        }
    }

    // Flush exporters
    if let Some(ref exporter) = es_exporter {
        exporter.flush().await.map_err(|e| anyhow::anyhow!("Failed to flush Elasticsearch: {}", e))?;
    }
    if let Some(ref exporter) = mongo_exporter {
        exporter.flush().await.map_err(|e| anyhow::anyhow!("Failed to flush MongoDB: {}", e))?;
    }
    if let Some(ref exporter) = cassandra_exporter {
        exporter.flush().await.map_err(|e| anyhow::anyhow!("Failed to flush Cassandra: {}", e))?;
    }

    output.flush()?;
    Ok(())
}

fn determine_record_types(args: &QueryArgs) -> Vec<RecordType> {
    let mut types = Vec::new();

    // If --record-type is specified, use those
    if !args.record_type.is_empty() {
        for rt in &args.record_type {
            match rt.to_uppercase().as_str() {
                "A" => types.push(RecordType::A),
                "AAAA" => types.push(RecordType::Aaaa),
                "CNAME" => types.push(RecordType::Cname),
                "MX" => types.push(RecordType::Mx),
                "TXT" => types.push(RecordType::Txt),
                "NS" => types.push(RecordType::Ns),
                "SOA" => types.push(RecordType::Soa),
                "PTR" => types.push(RecordType::Ptr),
                "SRV" => types.push(RecordType::Srv),
                "CAA" => types.push(RecordType::Caa),
                "CERT" => types.push(RecordType::Cert),
                "DNAME" => types.push(RecordType::Dname),
                "DNSKEY" => types.push(RecordType::Dnskey),
                "DS" => types.push(RecordType::Ds),
                "HINFO" => types.push(RecordType::Hinfo),
                "HTTPS" => types.push(RecordType::Https),
                "KEY" => types.push(RecordType::Key),
                "LOC" => types.push(RecordType::Loc),
                "NAPTR" => types.push(RecordType::Naptr),
                "NSEC" => types.push(RecordType::Nsec),
                "NSEC3" => types.push(RecordType::Nsec3),
                "OPT" => types.push(RecordType::Opt),
                "RRSIG" => types.push(RecordType::Rrsig),
                "SSHFP" => types.push(RecordType::Sshfp),
                "SVCB" => types.push(RecordType::Svcb),
                "TLSA" => types.push(RecordType::Tlsa),
                "URI" => types.push(RecordType::Uri),
                _ => eprintln!("Warning: Unknown record type '{}', ignoring", rt),
            }
        }
        return types;
    }

    // Fall back to individual flags
    if args.aaaa {
        types.push(RecordType::Aaaa);
    }
    if args.cname {
        types.push(RecordType::Cname);
    }
    if args.mx {
        types.push(RecordType::Mx);
    }
    if args.txt {
        types.push(RecordType::Txt);
    }
    if args.ns {
        types.push(RecordType::Ns);
    }
    if args.soa {
        types.push(RecordType::Soa);
    }
    if args.ptr {
        types.push(RecordType::Ptr);
    }
    if args.srv {
        types.push(RecordType::Srv);
    }
    if args.caa {
        types.push(RecordType::Caa);
    }
    if args.cert {
        types.push(RecordType::Cert);
    }
    if args.dname {
        types.push(RecordType::Dname);
    }
    if args.dnskey {
        types.push(RecordType::Dnskey);
    }
    if args.ds {
        types.push(RecordType::Ds);
    }
    if args.hinfo {
        types.push(RecordType::Hinfo);
    }
    if args.https {
        types.push(RecordType::Https);
    }
    if args.key {
        types.push(RecordType::Key);
    }
    if args.loc {
        types.push(RecordType::Loc);
    }
    if args.naptr {
        types.push(RecordType::Naptr);
    }
    if args.nsec {
        types.push(RecordType::Nsec);
    }
    if args.nsec3 {
        types.push(RecordType::Nsec3);
    }
    if args.opt {
        types.push(RecordType::Opt);
    }
    if args.rrsig {
        types.push(RecordType::Rrsig);
    }
    if args.sshfp {
        types.push(RecordType::Sshfp);
    }
    if args.svcb {
        types.push(RecordType::Svcb);
    }
    if args.tlsa {
        types.push(RecordType::Tlsa);
    }
    if args.uri {
        types.push(RecordType::Uri);
    }

    // Default to A if no specific types requested
    if types.is_empty() || args.a {
        types.push(RecordType::A);
    }

    types
}

fn read_domains(input_file: &Option<String>) -> Result<Vec<String>> {
    let lines = if let Some(file) = input_file {
        std::fs::read_to_string(file)?.lines().map(|s| s.to_string()).collect()
    } else {
        // Read from stdin
        use std::io::{self, BufRead};
        let stdin = io::stdin();
        stdin.lock().lines().collect::<io::Result<Vec<String>>>()?
    };

    Ok(lines.into_iter().filter(|s| !s.trim().is_empty()).collect())
}

fn parse_rcodes(rcode_str: &Option<String>) -> Result<Vec<ResponseCode>> {
    if let Some(rcodes) = rcode_str {
        let mut result = Vec::new();
        for code_str in rcodes.split(',') {
            let code = match code_str.trim().to_uppercase().as_str() {
                "NOERROR" => ResponseCode::NoError,
                "SERVFAIL" => ResponseCode::ServFail,
                "NXDOMAIN" => ResponseCode::NxDomain,
                "REFUSED" => ResponseCode::Refused,
                "FORMERR" => ResponseCode::FormErr,
                "NOTIMP" => ResponseCode::NotImp,
                _ => continue, // Skip unknown codes
            };
            result.push(code);
        }
        Ok(result)
    } else {
        Ok(Vec::new()) // No filter
    }
}
