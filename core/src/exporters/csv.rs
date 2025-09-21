use super::Exporter;
use crate::{Error, Result};
use csv::Writer;
use std::fs::File;

pub struct CsvExporter {
    headers: Vec<String>,
}

impl CsvExporter {
    pub fn new() -> Self {
        Self {
            headers: vec!["generated_data".to_string()],
        }
    }

    pub fn with_headers(headers: Vec<String>) -> Self {
        Self { headers }
    }
}

impl Default for CsvExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Exporter for CsvExporter {
    fn export(&self, data: &[String], output_path: &str) -> Result<()> {
        let file = File::create(output_path)
            .map_err(|e| Error::ExportFailed(format!("Failed to create CSV file: {}", e)))?;

        let mut writer = Writer::from_writer(file);

        // Write headers
        writer
            .write_record(&self.headers)
            .map_err(|e| Error::ExportFailed(format!("Failed to write CSV headers: {}", e)))?;

        // Write data rows
        for item in data {
            writer
                .write_record([item])
                .map_err(|e| Error::ExportFailed(format!("Failed to write CSV record: {}", e)))?;
        }

        writer
            .flush()
            .map_err(|e| Error::ExportFailed(format!("Failed to flush CSV writer: {}", e)))?;

        Ok(())
    }

    fn format_name(&self) -> &'static str {
        "CSV"
    }
}
