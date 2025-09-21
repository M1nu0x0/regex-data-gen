use crate::{RegexEngine, Result};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub struct DataGenerator {
    regex_engine: RegexEngine,
    rng: StdRng,
}

impl DataGenerator {
    pub fn new(pattern: &str) -> Result<Self> {
        let regex_engine = RegexEngine::new(pattern)?;
        let rng = StdRng::from_entropy();

        Ok(Self { regex_engine, rng })
    }

    pub fn with_seed(pattern: &str, seed: u64) -> Result<Self> {
        let regex_engine = RegexEngine::new(pattern)?;
        let rng = StdRng::seed_from_u64(seed);

        Ok(Self { regex_engine, rng })
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
        // Placeholder implementation - will be enhanced later
        // For now, generate simple random strings that might match basic patterns
        let length = self.rng.gen_range(5..20);
        let chars: String = (0..length)
            .map(|_| {
                let chars = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
                chars[self.rng.gen_range(0..chars.len())] as char
            })
            .collect();

        Ok(chars)
    }
}
