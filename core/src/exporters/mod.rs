pub mod csv;
pub mod json;
pub mod tsv;
pub mod xml;

use crate::Result;

pub trait Exporter {
    fn export(&self, data: &[String], output_path: &str) -> Result<()>;
    fn format_name(&self) -> &'static str;
}

pub use csv::CsvExporter;
pub use json::JsonExporter;
pub use tsv::TsvExporter;
pub use xml::XmlExporter;
