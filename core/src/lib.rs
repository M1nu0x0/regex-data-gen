pub mod data_generator;
pub mod exporters;
pub mod regex_engine;

pub use data_generator::{DataGenerator, GenerationMode};
pub use exporters::*;
pub use regex_engine::RegexEngine;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid regex pattern: {0}")]
    InvalidRegex(String),
    #[error("Data generation failed: {0}")]
    GenerationFailed(String),
    #[error("Export failed: {0}")]
    ExportFailed(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequential_letter_generation() {
        let mut generator = DataGenerator::new("[a-z]{2}").unwrap();
        let data = generator.generate_with_mode(5, GenerationMode::Sequential).unwrap();

        assert_eq!(data.len(), 5);
        assert_eq!(data[0], "aa");
        assert_eq!(data[1], "ab");
        assert_eq!(data[2], "ac");
        assert_eq!(data[3], "ad");
        assert_eq!(data[4], "ae");
    }

    #[test]
    fn test_reverse_letter_generation() {
        let mut generator = DataGenerator::new("[a-z]{2}").unwrap();
        let data = generator.generate_with_mode(3, GenerationMode::ReverseSequential).unwrap();

        assert_eq!(data.len(), 3);
        assert_eq!(data[0], "zz");
        assert_eq!(data[1], "yz");
        assert_eq!(data[2], "xz");
    }

    #[test]
    fn test_sequential_digit_generation() {
        let mut generator = DataGenerator::new("[0-9]{3}").unwrap();
        let data = generator.generate_with_mode(5, GenerationMode::Sequential).unwrap();

        assert_eq!(data.len(), 5);
        assert_eq!(data[0], "000");
        assert_eq!(data[1], "001");
        assert_eq!(data[2], "002");
        assert_eq!(data[3], "003");
        assert_eq!(data[4], "004");
    }

    #[test]
    fn test_reverse_digit_generation() {
        let mut generator = DataGenerator::new("[0-9]{2}").unwrap();
        let data = generator.generate_with_mode(3, GenerationMode::ReverseSequential).unwrap();

        assert_eq!(data.len(), 3);
        assert_eq!(data[0], "99");
        assert_eq!(data[1], "98");
        assert_eq!(data[2], "97");
    }

    #[test]
    fn test_complex_pattern_fallback() {
        let mut generator = DataGenerator::new("[a-z]+@[a-z]+\\.com").unwrap();
        let result = generator.generate_with_mode(5, GenerationMode::Sequential);

        // Complex patterns should fail with sequential mode
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Sequential generation not supported"));
    }
}