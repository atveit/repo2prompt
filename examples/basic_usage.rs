use std::path::PathBuf;
use repo2prompt::{Config, OutputFormat, run};

fn main() -> anyhow::Result<()> {
    // Create a basic configuration
    let config = Config {
        directory: PathBuf::from("."),
        output_file: Some(PathBuf::from("output.xml")),
        format: OutputFormat::Xml,
        include_patterns: vec!["*.rs".to_string()],
        exclude_patterns: vec!["*test*".to_string()], 
        exclude_dir_patterns: vec![],
        max_file_size: 1_000_000,
        pretty_print: true,
    };

    // Run the extraction
    run(config)?;
    println!("Repository content extracted to output.xml");
    
    Ok(())
}