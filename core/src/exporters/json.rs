use super::Exporter;
use crate::{Error, Result};
use serde_json::Value;
use std::fs::File;
use std::io::Write;

pub struct JsonExporter {
    pretty: bool,
    array_format: bool,
}

impl JsonExporter {
    pub fn new() -> Self {
        Self {
            pretty: true,
            array_format: true,
        }
    }

    pub fn with_options(pretty: bool, array_format: bool) -> Self {
        Self {
            pretty,
            array_format,
        }
    }
}

impl Default for JsonExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Exporter for JsonExporter {
    fn export(&self, data: &[String], output_path: &str) -> Result<()> {
        let mut file = File::create(output_path)
            .map_err(|e| Error::ExportFailed(format!("Failed to create JSON file: {}", e)))?;

        let json_data: Value = if self.array_format {
            // Export as array of strings
            Value::Array(data.iter().map(|s| Value::String(s.clone())).collect())
        } else {
            // Export as array of objects
            Value::Array(
                data.iter()
                    .enumerate()
                    .map(|(i, s)| {
                        serde_json::json!({
                            "id": i,
                            "value": s
                        })
                    })
                    .collect(),
            )
        };

        let json_string = if self.pretty {
            serde_json::to_string_pretty(&json_data)
        } else {
            serde_json::to_string(&json_data)
        }
        .map_err(|e| Error::ExportFailed(format!("Failed to serialize JSON: {}", e)))?;

        file.write_all(json_string.as_bytes())
            .map_err(|e| Error::ExportFailed(format!("Failed to write JSON file: {}", e)))?;

        Ok(())
    }

    fn format_name(&self) -> &'static str {
        "JSON"
    }
}