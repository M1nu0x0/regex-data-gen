use crate::{RegexEngine, Result};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng, rng};
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenerationMode {
    Random,
    Sequential,
    ReverseSequential,
}

pub struct DataGenerator {
    regex_engine: RegexEngine,
    regex_generator: rand_regex::Regex,
    rng: StdRng,
    pattern: String,
}

impl DataGenerator {
    pub fn new(pattern: &str) -> Result<Self> {
        let regex_engine = RegexEngine::new(pattern)?;
        let regex_generator = rand_regex::Regex::compile(pattern, 100)
            .map_err(|e| crate::Error::InvalidRegex(e.to_string()))?;
        let mut thread_rng = rng();
        let rng = StdRng::from_rng(&mut thread_rng);

        Ok(Self {
            regex_engine,
            regex_generator,
            rng,
            pattern: pattern.to_string(),
        })
    }

    pub fn with_seed(pattern: &str, seed: u64) -> Result<Self> {
        let regex_engine = RegexEngine::new(pattern)?;
        let regex_generator = rand_regex::Regex::compile(pattern, 100)
            .map_err(|e| crate::Error::InvalidRegex(e.to_string()))?;
        let rng = StdRng::seed_from_u64(seed);

        Ok(Self {
            regex_engine,
            regex_generator,
            rng,
            pattern: pattern.to_string(),
        })
    }

    pub fn generate(&mut self, count: usize) -> Result<Vec<String>> {
        self.generate_with_mode(count, GenerationMode::Random)
    }

    pub fn generate_with_mode(&mut self, count: usize, mode: GenerationMode) -> Result<Vec<String>> {
        match mode {
            GenerationMode::Random => self.generate_random(count),
            GenerationMode::Sequential => self.generate_sequential(count, false),
            GenerationMode::ReverseSequential => self.generate_sequential(count, true),
        }
    }

    fn generate_random(&mut self, count: usize) -> Result<Vec<String>> {
        let mut results = Vec::with_capacity(count);

        for _ in 0..count {
            let data = self.generate_single()?;
            results.push(data);
        }

        Ok(results)
    }

    fn generate_single(&mut self) -> Result<String> {
        // Use rand_regex to generate data that actually matches the pattern
        let generated = self.rng.sample(&self.regex_generator);
        Ok(generated)
    }

    /// Check if the regex generator can only produce ASCII characters
    pub fn is_ascii(&self) -> bool {
        self.regex_generator.is_ascii()
    }

    /// Get the maximum capacity (length) the regex can generate
    pub fn capacity(&self) -> usize {
        self.regex_generator.capacity()
    }

    /// Get the original regex pattern
    pub fn pattern(&self) -> &str {
        self.regex_engine.pattern()
    }

    /// Create a new generator with ASCII-only mode for better performance
    pub fn new_ascii_only(pattern: &str) -> Result<Self> {
        use regex_syntax::ParserBuilder;

        let regex_engine = RegexEngine::new(pattern)?;

        // Parse with Unicode disabled for ASCII-only generation
        let mut parser = ParserBuilder::new().unicode(false).build();
        let hir = parser.parse(pattern)
            .map_err(|e| crate::Error::InvalidRegex(e.to_string()))?;

        let regex_generator = rand_regex::Regex::with_hir(hir, 100)
            .map_err(|e| crate::Error::InvalidRegex(e.to_string()))?;

        let mut thread_rng = rng();
        let rng = StdRng::from_rng(&mut thread_rng);

        Ok(Self {
            regex_engine,
            regex_generator,
            rng,
            pattern: pattern.to_string(),
        })
    }

    /// Create a new generator with ASCII-only mode and custom seed
    pub fn with_seed_ascii_only(pattern: &str, seed: u64) -> Result<Self> {
        use regex_syntax::ParserBuilder;

        let regex_engine = RegexEngine::new(pattern)?;

        // Parse with Unicode disabled for ASCII-only generation
        let mut parser = ParserBuilder::new().unicode(false).build();
        let hir = parser.parse(pattern)
            .map_err(|e| crate::Error::InvalidRegex(e.to_string()))?;

        let regex_generator = rand_regex::Regex::with_hir(hir, 100)
            .map_err(|e| crate::Error::InvalidRegex(e.to_string()))?;

        let rng = StdRng::seed_from_u64(seed);

        Ok(Self {
            regex_engine,
            regex_generator,
            rng,
            pattern: pattern.to_string(),
        })
    }

    fn generate_sequential(&self, count: usize, reverse: bool) -> Result<Vec<String>> {
        // Simple sequential generation for basic patterns
        // This is a simplified implementation focusing on common patterns
        if let Some(results) = self.generate_simple_sequential(&self.pattern, count, reverse) {
            Ok(results)
        } else {
            // Fallback to random generation for complex patterns
            Err(crate::Error::GenerationFailed(
                "Sequential generation not supported for complex patterns".to_string()
            ))
        }
    }

    fn generate_simple_sequential(&self, pattern: &str, count: usize, reverse: bool) -> Option<Vec<String>> {
        // Handle simple character class patterns like [a-z]{3}, [0-9]{4}, etc.
        let single_char_class = Regex::new(r"^\[([a-z])-([a-z])\]\{(\d+)\}$").ok()?;
        let single_digit_class = Regex::new(r"^\[0-9\]\{(\d+)\}$").ok()?;

        if let Some(caps) = single_char_class.captures(pattern) {
            let start_char = caps[1].chars().next()?;
            let end_char = caps[2].chars().next()?;
            let length: usize = caps[3].parse().ok()?;

            return Some(self.generate_char_sequence(start_char, end_char, length, count, reverse));
        }

        if let Some(caps) = single_digit_class.captures(pattern) {
            let length: usize = caps[1].parse().ok()?;
            return Some(self.generate_digit_sequence(length, count, reverse));
        }

        None
    }

    fn generate_char_sequence(&self, start: char, end: char, length: usize, count: usize, reverse: bool) -> Vec<String> {
        let mut results = Vec::new();
        let char_range = (start as u8..=end as u8).collect::<Vec<_>>();
        let base = char_range.len();

        for i in 0..count {
            let index = if reverse {
                // Start from maximum value and go down
                let max_val = base.pow(length as u32);
                if i >= max_val { break; }
                max_val - 1 - i
            } else {
                i
            };

            let mut chars = Vec::new();
            let mut temp_index = index;

            for _ in 0..length {
                let char_idx = temp_index % base;
                chars.push(char_range[char_idx] as char);
                temp_index /= base;
            }

            if !reverse {
                chars.reverse(); // For normal order, reverse to get correct sequence
            }

            results.push(chars.into_iter().collect());

            // Break if we've exhausted all possibilities
            if temp_index > 0 && index >= base.pow(length as u32) { break; }
        }

        results
    }

    fn generate_digit_sequence(&self, length: usize, count: usize, reverse: bool) -> Vec<String> {
        let mut results = Vec::new();
        let max_val = 10_usize.pow(length as u32);

        for i in 0..count {
            let number = if reverse {
                if i >= max_val { break; }
                max_val - 1 - i
            } else {
                if i >= max_val { break; }
                i
            };

            results.push(format!("{:0width$}", number, width = length));
        }

        results
    }
}
