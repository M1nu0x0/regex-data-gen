use crate::{RegexEngine, Result};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng, rng};

pub struct DataGenerator {
    regex_engine: RegexEngine,
    regex_generator: rand_regex::Regex,
    rng: StdRng,
}

impl DataGenerator {
    pub fn new(pattern: &str) -> Result<Self> {
        let regex_engine = RegexEngine::new(pattern)?;
        let regex_generator = rand_regex::Regex::compile(pattern, 100)
            .map_err(|e| crate::Error::InvalidRegex(e.to_string()))?;
        let mut thread_rng = rng();
        let rng = StdRng::from_rng(&mut thread_rng);

        Ok(Self { regex_engine, regex_generator, rng })
    }

    pub fn with_seed(pattern: &str, seed: u64) -> Result<Self> {
        let regex_engine = RegexEngine::new(pattern)?;
        let regex_generator = rand_regex::Regex::compile(pattern, 100)
            .map_err(|e| crate::Error::InvalidRegex(e.to_string()))?;
        let rng = StdRng::seed_from_u64(seed);

        Ok(Self { regex_engine, regex_generator, rng })
    }

    pub fn generate(&mut self, count: usize) -> Result<Vec<String>> {
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

        Ok(Self { regex_engine, regex_generator, rng })
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

        Ok(Self { regex_engine, regex_generator, rng })
    }
}
