use regex_data_gen_core::{
    DataGenerator, CsvExporter, JsonExporter, XmlExporter, TsvExporter,
    Exporter, RegexEngine
};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_end_to_end_data_generation_and_export() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Test CSV export
    let csv_path = temp_dir.path().join("test.csv");
    let mut generator = DataGenerator::with_seed(r"\d{3}-\d{2}-\d{4}", 42).unwrap();
    let data = generator.generate(5).unwrap();

    let csv_exporter = CsvExporter::new();
    csv_exporter.export(&data, csv_path.to_str().unwrap()).unwrap();

    let csv_content = fs::read_to_string(&csv_path).unwrap();
    assert!(csv_content.contains("generated_data"));
    assert_eq!(csv_content.lines().count(), 6); // header + 5 data rows

    // Test JSON export
    let json_path = temp_dir.path().join("test.json");
    let json_exporter = JsonExporter::new();
    json_exporter.export(&data, json_path.to_str().unwrap()).unwrap();

    let json_content = fs::read_to_string(&json_path).unwrap();
    assert!(json_content.starts_with('['));
    assert!(json_content.ends_with(']'));

    // Test XML export
    let xml_path = temp_dir.path().join("test.xml");
    let xml_exporter = XmlExporter::new();
    xml_exporter.export(&data, xml_path.to_str().unwrap()).unwrap();

    let xml_content = fs::read_to_string(&xml_path).unwrap();
    assert!(xml_content.contains("<?xml"));
    assert!(xml_content.contains("<data>"));
    assert!(xml_content.contains("<item>"));

    // Test TSV export
    let tsv_path = temp_dir.path().join("test.tsv");
    let tsv_exporter = TsvExporter::new();
    tsv_exporter.export(&data, tsv_path.to_str().unwrap()).unwrap();

    let tsv_content = fs::read_to_string(&tsv_path).unwrap();
    assert!(tsv_content.contains("generated_data"));
    assert_eq!(tsv_content.lines().count(), 6); // header + 5 data rows
}

#[test]
fn test_regex_validation() {
    // Valid patterns
    assert!(RegexEngine::validate_pattern(r"\d{3}-\d{2}-\d{4}").is_ok());
    assert!(RegexEngine::validate_pattern(r"[a-z]+@[a-z]+\.com").is_ok());
    assert!(RegexEngine::validate_pattern(r"\+1-\d{3}-\d{3}-\d{4}").is_ok());

    // Invalid patterns
    assert!(RegexEngine::validate_pattern(r"[invalid").is_err());
    assert!(RegexEngine::validate_pattern(r"*invalid").is_err());
    assert!(RegexEngine::validate_pattern(r"(?invalid)").is_err());
}

#[test]
fn test_data_generator_with_seed() {
    let mut gen1 = DataGenerator::with_seed(r"\d{2}", 42).unwrap();
    let mut gen2 = DataGenerator::with_seed(r"\d{2}", 42).unwrap();

    let data1 = gen1.generate(5).unwrap();
    let data2 = gen2.generate(5).unwrap();

    // With same seed, should generate same data
    assert_eq!(data1, data2);
}

#[test]
fn test_ascii_only_generation() {
    let mut generator = DataGenerator::with_seed_ascii_only(r"\d{4}", 42).unwrap();
    assert!(generator.is_ascii());

    let data = generator.generate(10).unwrap();
    for item in data {
        assert!(item.is_ascii());
        assert_eq!(item.len(), 4);
        assert!(item.chars().all(|c| c.is_ascii_digit()));
    }
}

#[test]
fn test_multiple_format_exports() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let mut generator = DataGenerator::with_seed(r"[A-Z]{3}[0-9]{3}", 123).unwrap();
    let data = generator.generate(3).unwrap();

    // Test all exporters produce valid output
    let formats = vec![
        ("csv", Box::new(CsvExporter::new()) as Box<dyn Exporter>),
        ("json", Box::new(JsonExporter::new()) as Box<dyn Exporter>),
        ("xml", Box::new(XmlExporter::new()) as Box<dyn Exporter>),
        ("tsv", Box::new(TsvExporter::new()) as Box<dyn Exporter>),
    ];

    for (format, exporter) in formats {
        let path = temp_dir.path().join(format!("test.{}", format));
        exporter.export(&data, path.to_str().unwrap()).unwrap();

        let content = fs::read_to_string(&path).unwrap();
        assert!(!content.is_empty());
        assert_eq!(exporter.format_name(), format.to_uppercase());
    }
}