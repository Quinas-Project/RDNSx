//! Output formatting for DNS records

use crate::types::DnsRecord;

/// Output format
pub enum OutputFormat {
    /// Plain text format
    Plain,
    /// JSON format
    Json,
    /// Response values only
    ResponseOnly,
}

/// Format DNS record for output
pub fn format_record(record: &DnsRecord, format: OutputFormat) -> String {
    match format {
        OutputFormat::Plain => format!("{}", record),
        OutputFormat::Json => {
            serde_json::to_string(record).unwrap_or_else(|_| format!("{}", record))
        }
        OutputFormat::ResponseOnly => record.value.to_string(),
    }
}
