# Regex Data Generator

A comprehensive tool for generating random dummy data based on regex patterns and exporting it to multiple data formats.

## Quick Start

### CLI Usage

```bash
# Generate 100 CSV records from email pattern
regex-data-gen generate --pattern "[a-z]{5,10}@[a-z]{3,8}\.(com|org|net)" --count 100 --format csv --output emails.csv

# Validate a regex pattern
regex-data-gen validate "[0-9]{3}-[0-9]{3}-[0-9]{4}"
```

### Library Usage

```rust
use regex_data_gen_core::{DataGenerator, CsvExporter, Exporter};

// Create generator
let mut generator = DataGenerator::new(r"\d{3}-\d{3}-\d{4}")?;

// Generate data
let data = generator.generate(50)?;

// Export to CSV
let exporter = CsvExporter::new();
exporter.export(&data, "phone_numbers.csv")?;
```

## Supported Formats

- **CSV** - Comma-separated values
- **TSV** - Tab-separated values
- **JSON** - JavaScript Object Notation
- **XML** - eXtensible Markup Language

## Installation

### From Source

```bash
git clone https://github.com/your-username/regex-data-gen.git
cd regex-data-gen
cargo build --release
```

### Binary Installation

```bash
cargo install regex-data-gen-cli
```

## Documentation

- [API Documentation](API.md)
- [Examples](examples/)
- [Contributing Guide](../CONTRIBUTING.md)

## License

MIT License - see [LICENSE](../LICENSE) for details.