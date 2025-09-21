# Basic Usage Examples

## Email Generation

Generate realistic email addresses:

```bash
regex-data-gen generate \
  --pattern "[a-z]{3,8}\.[a-z]{3,8}@(gmail|yahoo|outlook)\.(com|org)" \
  --count 50 \
  --format json \
  --output emails.json
```

## Phone Numbers

Generate US phone numbers:

```bash
regex-data-gen generate \
  --pattern "\([0-9]{3}\) [0-9]{3}-[0-9]{4}" \
  --count 100 \
  --format csv \
  --output phones.csv
```

## Product Codes

Generate product SKUs:

```bash
regex-data-gen generate \
  --pattern "[A-Z]{2}[0-9]{4}-[A-Z]{1}[0-9]{2}" \
  --count 200 \
  --format xml \
  --output products.xml
```

## Credit Card Numbers (Test Data)

Generate test credit card patterns:

```bash
regex-data-gen generate \
  --pattern "4[0-9]{3} [0-9]{4} [0-9]{4} [0-9]{4}" \
  --count 25 \
  --format tsv \
  --output test_cards.tsv
```

## Reproducible Generation

Use seeds for consistent results:

```bash
regex-data-gen generate \
  --pattern "[A-Z]{3}[0-9]{3}" \
  --count 10 \
  --seed 12345 \
  --format json \
  --output reproducible.json
```

## Library Usage

### Simple Generation

```rust
use regex_data_gen_core::{DataGenerator, JsonExporter, Exporter};

fn main() -> anyhow::Result<()> {
    // Generate usernames
    let mut generator = DataGenerator::new(r"[a-z]{3,8}[0-9]{2,4}")?;
    let usernames = generator.generate(50)?;

    // Export to JSON
    let exporter = JsonExporter::new();
    exporter.export(&usernames, "usernames.json")?;

    Ok(())
}
```

### Multiple Formats

```rust
use regex_data_gen_core::*;

fn export_multiple_formats(data: &[String]) -> anyhow::Result<()> {
    // CSV export
    CsvExporter::new().export(data, "data.csv")?;

    // JSON export
    JsonExporter::new().export(data, "data.json")?;

    // XML export
    XmlExporter::new().export(data, "data.xml")?;

    // TSV export
    TsvExporter::new().export(data, "data.tsv")?;

    Ok(())
}
```