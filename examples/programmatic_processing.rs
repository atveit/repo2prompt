use std::collections::HashMap;
use std::path::PathBuf;
use repo2prompt::{Config, OutputFormat, process_repository, generate_output};

fn main() -> anyhow::Result<()> {
    // Create configuration
    let config = Config {
        directory: PathBuf::from("."),
        output_file: None,
        format: OutputFormat::Text,
        include_patterns: vec!["*.rs".to_string(), "*.md".to_string()],
        exclude_patterns: vec![],
        exclude_dir_patterns: vec!["target".to_string()],
        max_file_size: 1_000_000,
        pretty_print: false,
    };

    // Process repository
    println!("Processing repository...");
    let files_map = process_repository(&config)?;
    
    // Print statistics
    println!("Found {} files", files_map.len());
    let total_size: usize = files_map.iter().map(|(_, content)| content.len()).sum();
    println!("Total content size: {} bytes", total_size);
    
    // Count lines of code in Rust files
    let rust_files: HashMap<String, String> = files_map
        .iter()
        .filter(|(path, _)| path.ends_with(".rs"))
        .map(|(path, content)| (path.clone(), content.clone()))
        .collect();
    
    let rust_loc: usize = rust_files
        .iter()
        .map(|(_, content)| content.lines().count())
        .sum();
    
    println!("Rust files: {}, Lines of code: {}", rust_files.len(), rust_loc);
    
    // Generate output in text format
    let output = generate_output(&files_map, &config)?;
    
    // Just print the first 200 characters to avoid flooding the console
    println!("\nPreview of text output:");
    println!("{}", &output[..200.min(output.len())]);
    println!("...");
    
    Ok(())
}