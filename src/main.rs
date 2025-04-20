use anyhow::Result;
use clap::{Parser, ValueEnum};
use repo2prompt::{run, Config, OutputFormat};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Format {
    Xml,
    Json,
    Text,
}

impl From<Format> for OutputFormat {
    fn from(format: Format) -> Self {
        match format {
            Format::Xml => OutputFormat::Xml,
            Format::Json => OutputFormat::Json,
            Format::Text => OutputFormat::Text,
        }
    }
}

#[derive(Parser, Debug)]
#[command(
    name = "repo2prompt",
    about = "Extract repository content into XML, JSON, or text format",
    version
)]
struct Cli {
    /// Root directory to process (default: current directory)
    #[arg(short, long, default_value = ".")]
    directory: PathBuf,
    
    /// Output file (default: stdout)
    #[arg(short, long)]
    output: Option<PathBuf>,
    
    /// Output format (default: xml)
    #[arg(short, long, value_enum, default_value_t = Format::Xml)]
    format: Format,
    
    /// File patterns to include (can specify multiple, e.g., '*.py', '*.js')
    #[arg(short, long)]
    include: Vec<String>,
    
    /// File patterns to exclude (can specify multiple)
    #[arg(short, long)]
    exclude: Vec<String>,
    
    /// Directory patterns to exclude (can specify multiple)
    #[arg(long)]
    exclude_dirs: Vec<String>,
    
    /// Maximum file size in bytes (default: 1MB)
    #[arg(long, default_value_t = 1_000_000)]
    max_size: u64,
    
    /// Pretty-print output (for XML and JSON)
    #[arg(long)]
    pretty: bool,
    
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    // Parse command line arguments
    let args = Cli::parse();
    
    // Setup logging
    if args.verbose {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    } else {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    }
    
    // Create configuration
    let config = Config {
        directory: args.directory,
        output_file: args.output,
        format: args.format.into(),
        include_patterns: if args.include.is_empty() {
            vec![
                "*.py".to_string(),
                "*.js".to_string(),
                "*.rs".to_string(), 
                "*.md".to_string(),
                "*.txt".to_string(),
                "*.ini".to_string(),
            ]
        } else {
            args.include
        },
        exclude_patterns: args.exclude,
        exclude_dir_patterns: args.exclude_dirs,
        max_file_size: args.max_size,
        pretty_print: args.pretty,
    };
    
    // Run program
    run(config)
}