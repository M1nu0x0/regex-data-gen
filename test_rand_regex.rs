// Test file for understanding rand_regex API
extern crate rand;
extern crate rand_regex;
extern crate regex_syntax;

use rand::{SeedableRng, Rng};
use rand::rngs::StdRng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing rand_regex library...");

    // Test 1: Basic usage with seed
    let mut rng = StdRng::seed_from_u64(12345);
    let gen = rand_regex::Regex::compile(r"\d{4}-\d{2}-\d{2}", 100)?;

    println!("\nTest 1: Date pattern \\d{{4}}-\\d{{2}}-\\d{{2}}");
    for i in 0..3 {
        let sample = rng.sample(&gen);
        println!("  Sample {}: {}", i + 1, sample);
    }

    // Test 2: Email pattern
    let email_gen = rand_regex::Regex::compile(r"[a-z]{3,8}@[a-z]{3,6}\.(com|org|net)", 100)?;
    println!("\nTest 2: Email pattern [a-z]{{3,8}}@[a-z]{{3,6}}\\.(com|org|net)");
    for i in 0..3 {
        let sample = rng.sample(&email_gen);
        println!("  Sample {}: {}", i + 1, sample);
    }

    // Test 3: Phone number pattern
    let phone_gen = rand_regex::Regex::compile(r"\+1-\d{3}-\d{3}-\d{4}", 100)?;
    println!("\nTest 3: Phone pattern \\+1-\\d{{3}}-\\d{{3}}-\\d{{4}}");
    for i in 0..3 {
        let sample = rng.sample(&phone_gen);
        println!("  Sample {}: {}", i + 1, sample);
    }

    // Test 4: Check properties
    println!("\nTest 4: Generator properties");
    println!("  Date gen is_ascii: {}", gen.is_ascii());
    println!("  Date gen capacity: {}", gen.capacity());

    // Test 5: ASCII-only generation using regex_syntax
    use regex_syntax::ParserBuilder;
    let mut parser = ParserBuilder::new().unicode(false).build();
    let hir = parser.parse(r"\d{4}-\d{2}-\d{2}")?;
    let ascii_gen = rand_regex::Regex::with_hir(hir, 100)?;

    println!("\nTest 5: ASCII-only date generation");
    for i in 0..3 {
        let sample = rng.sample(&ascii_gen);
        println!("  Sample {}: {}", i + 1, sample);
    }
    println!("  ASCII gen is_ascii: {}", ascii_gen.is_ascii());

    Ok(())
}