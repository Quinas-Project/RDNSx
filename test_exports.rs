//! Test script for database exports
//! Run with: cargo run --bin test_exports

use std::time::{Duration, SystemTime};

use rdnsx_core::{
    export::{CassandraExporter, ElasticsearchExporter, MongodbExporter},
    types::{DnsRecord, RecordType, RecordValue, ResponseCode},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Testing database exports...");

    // Create a test DNS record
    let test_record = DnsRecord::new(
        "example.com".to_string(),
        RecordType::A,
        RecordValue::Ip("93.184.216.34".parse().unwrap()),
        300,
        ResponseCode::NoError,
        "8.8.8.8:53".to_string(),
        45.2,
    );

    // Test MongoDB export
    println!("📊 Testing MongoDB export...");
    match MongodbExporter::new("mongodb://localhost:27017", "dnsx_test", "records", 10).await {
        Ok(mut exporter) => {
            println!("✅ Connected to MongoDB");
            if let Err(e) = exporter.export(test_record.clone()).await {
                println!("❌ MongoDB export failed: {}", e);
            } else {
                println!("✅ MongoDB export successful");
            }
            if let Err(e) = exporter.flush().await {
                println!("❌ MongoDB flush failed: {}", e);
            } else {
                println!("✅ MongoDB flush successful");
            }
        }
        Err(e) => {
            println!("❌ MongoDB connection failed: {}", e);
        }
    }

    // Test Elasticsearch export
    println!("\n🔍 Testing Elasticsearch export...");
    match ElasticsearchExporter::new("http://localhost:9200", "dnsx-test-records", 10).await {
        Ok(mut exporter) => {
            println!("✅ Connected to Elasticsearch");
            if let Err(e) = exporter.export(test_record.clone()).await {
                println!("❌ Elasticsearch export failed: {}", e);
            } else {
                println!("✅ Elasticsearch export successful");
            }
            if let Err(e) = exporter.flush().await {
                println!("❌ Elasticsearch flush failed: {}", e);
            } else {
                println!("✅ Elasticsearch flush successful");
            }
        }
        Err(e) => {
            println!("❌ Elasticsearch connection failed: {}", e);
        }
    }

    // Test Cassandra export
    println!("\n🗄️  Testing Cassandra export...");
    match CassandraExporter::new(
        &["127.0.0.1:9042".to_string()],
        None,
        None,
        "dnsx_test",
        "records",
        10,
    ).await {
        Ok(mut exporter) => {
            println!("✅ Connected to Cassandra");
            if let Err(e) = exporter.export(test_record.clone()).await {
                println!("❌ Cassandra export failed: {}", e);
            } else {
                println!("✅ Cassandra export successful");
            }
            if let Err(e) = exporter.flush().await {
                println!("❌ Cassandra flush failed: {}", e);
            } else {
                println!("✅ Cassandra flush successful");
            }
        }
        Err(e) => {
            println!("❌ Cassandra connection failed: {}", e);
        }
    }

    println!("\n🏁 Export tests completed!");
    Ok(())
}