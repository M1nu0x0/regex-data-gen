# API Documentation

## Core Library (`regex-data-gen-core`)

### RegexEngine

Handles regex pattern validation and compilation.

```rust
use regex_data_gen_core::RegexEngine;

// Create engine with pattern
let engine = RegexEngine::new(r"\d{3}-\d{2}-\d{4}")?;

// Validate pattern without creating engine
RegexEngine::validate_pattern(r"[a-z]+")?;

// Check if text matches pattern
let matches = engine.is_match("123-45-6789");
```

### DataGenerator

Generates random data matching regex patterns.

```rust
use regex_data_gen_core::DataGenerator;

// Create generator with random seed
let mut generator = DataGenerator::new(r"[A-Z]{2}\d{4}")?;

// Create generator with specific seed for reproducible results
let mut generator = DataGenerator::with_seed(r"[A-Z]{2}\d{4}", 12345)?;

// Generate multiple items
let data = generator.generate(100)?;
```

### Exporters

All exporters implement the `Exporter` trait:

```rust
pub trait Exporter {
    fn export(&self, data: &[String], output_path: &str) -> Result<()>;
    fn format_name(&self) -> &'static str;
}
```

#### CsvExporter

```rust
use regex_data_gen_core::{CsvExporter, Exporter};

// Default exporter with "generated_data" header
let exporter = CsvExporter::new();

// Custom headers
let exporter = CsvExporter::with_headers(vec!["email".to_string()]);

exporter.export(&data, "output.csv")?;
```

#### JsonExporter

```rust
use regex_data_gen_core::{JsonExporter, Exporter};

// Default: pretty printed array format
let exporter = JsonExporter::new();

// Custom options
let exporter = JsonExporter::with_options(false, true); // compact, array format

exporter.export(&data, "output.json")?;
```

#### XmlExporter

```rust
use regex_data_gen_core::{XmlExporter, Exporter};

// Default: <data><item>...</item></data>
let exporter = XmlExporter::new();

// Custom element names
let exporter = XmlExporter::with_elements("records".to_string(), "record".to_string());

exporter.export(&data, "output.xml")?;
```

#### TsvExporter

```rust
use regex_data_gen_core::{TsvExporter, Exporter};

// Default with "generated_data" header
let exporter = TsvExporter::new();

// Custom headers
let exporter = TsvExporter::with_headers(vec!["data".to_string()]);

exporter.export(&data, "output.tsv")?;
```

## Error Handling

The library uses a custom `Error` type:

```rust
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid regex pattern: {0}")]
    InvalidRegex(String),
    #[error("Data generation failed: {0}")]
    GenerationFailed(String),
    #[error("Export failed: {0}")]
    ExportFailed(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

All operations return `Result<T, Error>` for proper error handling.