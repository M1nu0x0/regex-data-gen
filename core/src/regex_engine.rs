use regex::Regex;
use crate::{Error, Result};

pub struct RegexEngine {
    pattern: String,
    compiled: Regex,
}

impl RegexEngine {
    pub fn new(pattern: &str) -> Result<Self> {
        let compiled = Regex::new(pattern)
            .map_err(|e| Error::InvalidRegex(e.to_string()))?;

        Ok(Self {
            pattern: pattern.to_string(),
            compiled,
        })
    }

    pub fn pattern(&self) -> &str {
        &self.pattern
    }

    pub fn is_match(&self, text: &str) -> bool {
        self.compiled.is_match(text)
    }

    pub fn validate_pattern(pattern: &str) -> Result<()> {
        Regex::new(pattern)
            .map_err(|e| Error::InvalidRegex(e.to_string()))?;
        Ok(())
    }
}