use std::path::PathBuf;
use repo2prompt::{Config, OutputFormat, run};

fn main() -> anyhow::Result<()> {
    // Create a configuration for JSON output
    let config = Config {
        directory: PathBuf::from("."), 
        output_file: None, // Output to stdout
        format: OutputFormat::Json,
        include_patterns: vec!["*.md".to_string(), "*.toml".to_string()],
        exclude_patterns: vec![],
        exclude_dir_patterns: vec!["target".to_string()],
        max_file_size: 500_000, // 500KB max
        pretty_print: true,
    };

    // Run the extraction
    println!("Extracting repository content to JSON...");
    run(config)?;
    
    Ok(())
}