use clap::{Args, Parser, Subcommand, ValueEnum};
use regex_data_gen_core::{
    CsvExporter, DataGenerator, Exporter, JsonExporter, TsvExporter, XmlExporter,
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate(args) => generate_data(args).await,
        Commands::Validate(args) => validate_pattern(args).await,
    }
}

async fn generate_data(args: GenerateArgs) -> anyhow::Result<()> {
    println!(
        "Generating {} items from pattern: {}",
        args.count, args.pattern
    );

    let mut generator = if let Some(seed) = args.seed {
        DataGenerator::with_seed(&args.pattern, seed)?
    } else {
        DataGenerator::new(&args.pattern)?
    };

    let data = generator.generate(args.count)?;

    let output_path = args.output.to_string_lossy();

    match args.format {
        OutputFormat::Csv => {
            let exporter = CsvExporter::new();
            exporter.export(&data, &output_path)?;
        }
        OutputFormat::Json => {
            let exporter = JsonExporter::new();
            exporter.export(&data, &output_path)?;
        }
        OutputFormat::Xml => {
            let exporter = XmlExporter::new();
            exporter.export(&data, &output_path)?;
        }
        OutputFormat::Tsv => {
            let exporter = TsvExporter::new();
            exporter.export(&data, &output_path)?;
        }
    }

    println!(
        "Successfully generated {} items to {}",
        args.count, output_path
    );
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
