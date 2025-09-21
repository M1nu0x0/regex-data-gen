use regex_data_gen_core::{DataGenerator, ExportFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Regex Data Generator MVP Test ===\n");

    // Test patterns
    let test_patterns = vec![
        (r"[a-z]{3,8}", "Lowercase letters (3-8 chars)"),
        (r"\d{4}", "4-digit numbers"),
        (r"[A-Z][a-z]+", "Capitalized words"),
        (r"[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}", "IP addresses"),
        (r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}", "Email addresses"),
    ];

    for (pattern, description) in test_patterns {
        println!("Testing: {} ({})", pattern, description);

        // Generate 5 samples
        let generator = DataGenerator::new(pattern, Some(42))?; // Use seed for reproducibility
        let samples = generator.generate(5)?;

        println!("Generated samples:");
        for (i, sample) in samples.iter().enumerate() {
            println!("  {}. {}", i + 1, sample);
        }

        // Export to CSV
        let csv_content = generator.export(&samples, ExportFormat::Csv)?;
        println!("CSV export preview (first 200 chars):");
        let preview = if csv_content.len() > 200 {
            format!("{}...", &csv_content[..200])
        } else {
            csv_content.clone()
        };
        println!("  {}", preview.replace('\n', "\\n"));

        println!("---\n");
    }

    println!("MVP test completed successfully!");
    Ok(())
}