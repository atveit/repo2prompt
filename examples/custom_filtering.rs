use std::path::PathBuf;
use repo2prompt::{Config, OutputFormat, process_repository, generate_output, write_output};

fn main() -> anyhow::Result<()> {
    // Create configuration with specific file filters
    let config = Config {
        directory: PathBuf::from("."),
        output_file: Some(PathBuf::from("filtered_repo.json")),
        format: OutputFormat::Json,
        include_patterns: vec![
            // Include only source code files
            "*.rs".to_string(),
            "*.toml".to_string(),
            "*.md".to_string(),
        ],
        exclude_patterns: vec![
            // Exclude test files
            "*test*".to_string(),
            "*spec*".to_string(),
        ],
        exclude_dir_patterns: vec![
            // Exclude build/dependency directories
            "target".to_string(),
            ".git".to_string(),
        ],
        max_file_size: 100_000, // 100KB max
        pretty_print: true,
    };

    // Process repository
    let mut files_map = process_repository(&config)?;
    println!("Initial processing found {} files", files_map.len());
    
    // Additional programmatic filtering
    // Remove any file containing "TODO" or "FIXME" comments
    files_map.retain(|_, content| {
        !content.contains("TODO") && !content.contains("FIXME")
    });
    
    println!("After additional filtering: {} files", files_map.len());
    
    // Generate and write output
    let output = generate_output(&files_map, &config)?;
    write_output(&output, &config)?;
    
    println!("Filtered repository content written to filtered_repo.json");
    
    Ok(())
}