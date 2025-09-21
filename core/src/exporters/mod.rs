pub mod csv;
pub mod json;
pub mod xml;
pub mod tsv;

use crate::Result;

pub trait Exporter {
    fn export(&self, data: &[String], output_path: &str) -> Result<()>;
    fn format_name(&self) -> &'static str;
}

pub use csv::CsvExporter;
pub use json::JsonExporter;
pub use xml::XmlExporter;
pub use tsv::TsvExporter;