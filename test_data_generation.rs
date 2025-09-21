use regex_data_gen_core::DataGenerator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing updated DataGenerator with rand_regex...");

    // Test 1: Date pattern
    println!("\n=== Test 1: Date Pattern ===");
    let mut date_gen = DataGenerator::with_seed(r"\d{4}-\d{2}-\d{2}", 12345)?;
    println!("Pattern: \\d{{4}}-\\d{{2}}-\\d{{2}}");
    println!("Is ASCII: {}", date_gen.is_ascii());
    println!("Capacity: {}", date_gen.capacity());

    let dates = date_gen.generate(5)?;
    for (i, date) in dates.iter().enumerate() {
        println!("  Sample {}: {}", i + 1, date);
    }

    // Test 2: Email pattern
    println!("\n=== Test 2: Email Pattern ===");
    let mut email_gen = DataGenerator::with_seed(r"[a-z]{3,8}@[a-z]{3,6}\.(com|org|net)", 12345)?;
    println!("Pattern: [a-z]{{3,8}}@[a-z]{{3,6}}\\.(com|org|net)");
    println!("Is ASCII: {}", email_gen.is_ascii());
    println!("Capacity: {}", email_gen.capacity());

    let emails = email_gen.generate(5)?;
    for (i, email) in emails.iter().enumerate() {
        println!("  Sample {}: {}", i + 1, email);
    }

    // Test 3: Phone number pattern
    println!("\n=== Test 3: Phone Pattern ===");
    let mut phone_gen = DataGenerator::with_seed(r"\+1-\d{3}-\d{3}-\d{4}", 12345)?;
    println!("Pattern: \\+1-\\d{{3}}-\\d{{3}}-\\d{{4}}");
    println!("Is ASCII: {}", phone_gen.is_ascii());
    println!("Capacity: {}", phone_gen.capacity());

    let phones = phone_gen.generate(5)?;
    for (i, phone) in phones.iter().enumerate() {
        println!("  Sample {}: {}", i + 1, phone);
    }

    // Test 4: ASCII-only generation
    println!("\n=== Test 4: ASCII-only Date Generation ===");
    let mut ascii_gen = DataGenerator::with_seed_ascii_only(r"\d{4}-\d{2}-\d{2}", 12345)?;
    println!("Pattern: \\d{{4}}-\\d{{2}}-\\d{{2}} (ASCII-only)");
    println!("Is ASCII: {}", ascii_gen.is_ascii());
    println!("Capacity: {}", ascii_gen.capacity());

    let ascii_dates = ascii_gen.generate(5)?;
    for (i, date) in ascii_dates.iter().enumerate() {
        println!("  Sample {}: {}", i + 1, date);
    }

    // Test 5: Simple word pattern
    println!("\n=== Test 5: Word Pattern ===");
    let mut word_gen = DataGenerator::with_seed(r"[A-Z][a-z]{4,8}", 12345)?;
    println!("Pattern: [A-Z][a-z]{{4,8}}");
    println!("Is ASCII: {}", word_gen.is_ascii());
    println!("Capacity: {}", word_gen.capacity());

    let words = word_gen.generate(5)?;
    for (i, word) in words.iter().enumerate() {
        println!("  Sample {}: {}", i + 1, word);
    }

    println!("\n✅ All tests completed successfully!");
    Ok(())
}