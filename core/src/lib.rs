pub mod regex_engine;
pub mod data_generator;
pub mod exporters;

pub use regex_engine::RegexEngine;
pub use data_generator::DataGenerator;
pub use exporters::*;

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