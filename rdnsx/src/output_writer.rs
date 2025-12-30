//! Output writing utilities

use anyhow::Result;
use rdnsx_core::types::DnsRecord;
use std::io::{self, Write};

pub struct OutputWriter {
    writer: Box<dyn Write>,
    json_output: bool,
    silent: bool,
}

impl OutputWriter {
    pub fn new(output_file: Option<String>, json_output: bool, silent: bool) -> Result<Self> {
        let writer: Box<dyn Write> = if let Some(file) = output_file {
            Box::new(std::fs::File::create(file)?)
        } else {
            Box::new(io::stdout())
        };

        Ok(Self {
            writer,
            json_output,
            silent,
        })
    }

    pub fn write_record(&mut self, record: &DnsRecord, resp_only: bool) -> Result<()> {
        if self.silent {
            return Ok(());
        }

        let output = if resp_only {
            record.value.to_string()
        } else if self.json_output {
            serde_json::to_string(record)?
        } else {
            format!("{}\n", record)
        };

        write!(self.writer, "{}", output)?;
        self.writer.flush()?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<()> {
        self.writer.flush()?;
        Ok(())
    }
}
