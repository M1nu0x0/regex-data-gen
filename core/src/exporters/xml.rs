use super::Exporter;
use crate::{Error, Result};
use quick_xml::events::{Event, BytesEnd, BytesStart, BytesText};
use quick_xml::Writer;
use std::fs::File;
use std::io::Cursor;

pub struct XmlExporter {
    root_element: String,
    item_element: String,
}

impl XmlExporter {
    pub fn new() -> Self {
        Self {
            root_element: "data".to_string(),
            item_element: "item".to_string(),
        }
    }

    pub fn with_elements(root_element: String, item_element: String) -> Self {
        Self {
            root_element,
            item_element,
        }
    }
}

impl Default for XmlExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Exporter for XmlExporter {
    fn export(&self, data: &[String], output_path: &str) -> Result<()> {
        let file = File::create(output_path)
            .map_err(|e| Error::ExportFailed(format!("Failed to create XML file: {}", e)))?;

        let mut writer = Writer::new(file);

        // Write XML declaration
        writer
            .write_event(Event::Decl(quick_xml::events::BytesDecl::new("1.0", Some("UTF-8"), None)))
            .map_err(|e| Error::ExportFailed(format!("Failed to write XML declaration: {}", e)))?;

        // Write root element start
        writer
            .write_event(Event::Start(BytesStart::new(&self.root_element)))
            .map_err(|e| Error::ExportFailed(format!("Failed to write root element: {}", e)))?;

        // Write data items
        for item in data {
            // Start item element
            writer
                .write_event(Event::Start(BytesStart::new(&self.item_element)))
                .map_err(|e| Error::ExportFailed(format!("Failed to write item start: {}", e)))?;

            // Write item content
            let escaped_content = quick_xml::escape::escape(item);
            writer
                .write_event(Event::Text(BytesText::new(&escaped_content)))
                .map_err(|e| Error::ExportFailed(format!("Failed to write item content: {}", e)))?;

            // End item element
            writer
                .write_event(Event::End(BytesEnd::new(&self.item_element)))
                .map_err(|e| Error::ExportFailed(format!("Failed to write item end: {}", e)))?;
        }

        // Write root element end
        writer
            .write_event(Event::End(BytesEnd::new(&self.root_element)))
            .map_err(|e| Error::ExportFailed(format!("Failed to write root end: {}", e)))?;

        Ok(())
    }

    fn format_name(&self) -> &'static str {
        "XML"
    }
}