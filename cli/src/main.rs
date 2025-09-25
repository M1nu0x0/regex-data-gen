use clap::{Args, Parser, Subcommand, ValueEnum};
use regex_data_gen_core::{
    CsvExporter, DataGenerator, Exporter, GenerationMode, JsonExporter, TsvExporter, XmlExporter,
};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "regex-data-gen")]
#[command(about = "Generate random data from regex patterns")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Generate(GenerateArgs),
    Validate(ValidateArgs),
}

#[derive(Args)]
struct GenerateArgs {
    /// Regex pattern to generate data from
    #[arg(short, long)]
    pattern: String,

    /// Number of data items to generate
    #[arg(short = 'n', long, default_value = "10")]
    count: usize,

    /// Output format
    #[arg(short, long, default_value = "csv")]
    format: OutputFormat,

    /// Output file path
    #[arg(short, long)]
    output: PathBuf,

    /// Random seed for reproducible generation
    #[arg(short, long)]
    seed: Option<u64>,

    /// Generation mode
    #[arg(short, long, default_value = "random")]
    mode: GenerationModeArg,
}

#[derive(Args)]
struct ValidateArgs {
    /// Regex pattern to validate
    pattern: String,
}

#[derive(Clone, ValueEnum)]
enum OutputFormat {
    Csv,
    Json,
    Xml,
    Tsv,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Csv => write!(f, "csv"),
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Xml => write!(f, "xml"),
            OutputFormat::Tsv => write!(f, "tsv"),
        }
    }
}

#[derive(Clone, ValueEnum)]
enum GenerationModeArg {
    Random,
    Sequential,
    Reverse,
}

impl From<GenerationModeArg> for GenerationMode {
    fn from(mode: GenerationModeArg) -> Self {
        match mode {
            GenerationModeArg::Random => GenerationMode::Random,
            GenerationModeArg::Sequential => GenerationMode::Sequential,
            GenerationModeArg::Reverse => GenerationMode::ReverseSequential,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate(args) => generate_data(args).await,
        Commands::Validate(args) => validate_pattern(args).await,
    }
}

async fn generate_data(args: GenerateArgs) -> anyhow::Result<()> {
    // Validate output directory exists
    if let Some(parent) = args.output.parent()
        && !parent.exists() {
            eprintln!("Error: Output directory '{}' does not exist", parent.display());
            std::process::exit(1);
        }

    println!(
        "🔄 Generating {} items from pattern: '{}'",
        args.count, args.pattern
    );

    let mut generator = if let Some(seed) = args.seed {
        match DataGenerator::with_seed(&args.pattern, seed) {
            Ok(generator) => {
                println!("ℹ️  Using seed: {} for reproducible generation", seed);
                generator
            }
            Err(e) => {
                eprintln!("❌ Failed to create generator: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        match DataGenerator::new(&args.pattern) {
            Ok(generator) => generator,
            Err(e) => {
                eprintln!("❌ Failed to create generator: {}", e);
                std::process::exit(1);
            }
        }
    };

    let generation_mode: GenerationMode = args.mode.into();

    let data = match generator.generate_with_mode(args.count, generation_mode) {
        Ok(data) => {
            println!("✅ Successfully generated {} items", data.len());
            data
        }
        Err(e) => {
            eprintln!("❌ Data generation failed: {}", e);
            std::process::exit(1);
        }
    };

    let output_path = args.output.to_string_lossy();
    println!("💾 Exporting to {} format...", args.format.to_string().to_uppercase());

    let export_result = match args.format {
        OutputFormat::Csv => {
            let exporter = CsvExporter::new();
            exporter.export(&data, &output_path)
        }
        OutputFormat::Json => {
            let exporter = JsonExporter::new();
            exporter.export(&data, &output_path)
        }
        OutputFormat::Xml => {
            let exporter = XmlExporter::new();
            exporter.export(&data, &output_path)
        }
        OutputFormat::Tsv => {
            let exporter = TsvExporter::new();
            exporter.export(&data, &output_path)
        }
    };

    match export_result {
        Ok(()) => {
            println!(
                "🎉 Successfully exported {} items to '{}'",
                args.count, output_path
            );
        }
        Err(e) => {
            eprintln!("❌ Export failed: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

async fn validate_pattern(args: ValidateArgs) -> anyhow::Result<()> {
    match regex_data_gen_core::RegexEngine::validate_pattern(&args.pattern) {
        Ok(()) => {
            println!("✓ Pattern '{}' is valid", args.pattern);
        }
        Err(e) => {
            println!("✗ Pattern '{}' is invalid: {}", args.pattern, e);
            std::process::exit(1);
        }
    }
    Ok(())
}
