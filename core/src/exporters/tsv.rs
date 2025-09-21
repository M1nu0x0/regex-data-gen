use super::Exporter;
use crate::{Error, Result};
use std::fs::File;
use std::io::Write;

pub struct TsvExporter {
    headers: Vec<String>,
}

impl TsvExporter {
    pub fn new() -> Self {
        Self {
            headers: vec!["generated_data".to_string()],
        }
    }

    pub fn with_headers(headers: Vec<String>) -> Self {
        Self { headers }
    }
}

impl Default for TsvExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Exporter for TsvExporter {
    fn export(&self, data: &[String], output_path: &str) -> Result<()> {
        let mut file = File::create(output_path)
            .map_err(|e| Error::ExportFailed(format!("Failed to create TSV file: {}", e)))?;

        // Write headers
        let header_line = self.headers.join("\t");
        writeln!(file, "{}", header_line)
            .map_err(|e| Error::ExportFailed(format!("Failed to write TSV headers: {}", e)))?;

        // Write data rows
        for item in data {
            let escaped_item = item.replace('\t', "\\t").replace('\n', "\\n");
            writeln!(file, "{}", escaped_item)
                .map_err(|e| Error::ExportFailed(format!("Failed to write TSV record: {}", e)))?;
        }

        Ok(())
    }

    fn format_name(&self) -> &'static str {
        "TSV"
    }
}