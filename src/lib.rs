use anyhow::{Context, Result};
use globset::{Glob, GlobSet, GlobSetBuilder};
use quick_xml::se::to_string as xml_to_string;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;
use walkdir::{DirEntry, WalkDir};

#[cfg(test)]
mod tests;

/// Supported output formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Xml,
    Json,
    Text,
}

/// Configuration for repository processing
#[derive(Debug, Clone)]
pub struct Config {
    pub directory: PathBuf,
    pub output_file: Option<PathBuf>,
    pub format: OutputFormat,
    pub include_patterns: Vec<String>,
    pub exclude_patterns: Vec<String>,
    pub exclude_dir_patterns: Vec<String>,
    pub max_file_size: u64,
    pub pretty_print: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            directory: PathBuf::from("."),
            output_file: None,
            format: OutputFormat::Xml,
            include_patterns: vec![
                "*.py".to_string(),
                "*.js".to_string(),
                "*.rs".to_string(),
                "*.md".to_string(),
                "*.txt".to_string(),
                "*.ini".to_string(),
            ],
            exclude_patterns: Vec::new(),
            exclude_dir_patterns: Vec::new(),
            max_file_size: 1_000_000, // 1MB
            pretty_print: false,
        }
    }
}

/// Repository file with path and content
#[derive(Debug, Clone, Serialize)]
pub struct RepoFile {
    pub path: String,
    pub content: String,
}

/// Repository representation for serialization
#[derive(Debug, Serialize)]
struct Repository {
    files: Vec<RepoFile>,
}

/// Errors that can occur during repository processing
#[derive(Error, Debug)]
pub enum RepoError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Failed to process directory: {0}")]
    DirectoryProcessing(String),
    
    #[error("Failed to serialize output: {0}")]
    Serialization(String),
}

/// Process a repository and extract file contents based on configuration
pub fn process_repository(config: &Config) -> Result<HashMap<String, String>> {
    let mut files_map = HashMap::new();
    let root_path = config.directory.canonicalize()?;
    
    // Build include glob set
    let include_glob_set = build_glob_set(&config.include_patterns)?;
    
    // Build exclude glob set
    let exclude_glob_set = build_glob_set(&config.exclude_patterns)?;
    
    // Build exclude dir glob set
    let exclude_dir_glob_set = build_glob_set(&config.exclude_dir_patterns)?;
    
    // Default directory excludes
    let default_dir_excludes = [
        ".git", "target", "node_modules", "__pycache__", 
        "venv", ".venv", "env", ".env",
    ];
    
    // Walk directory
    for entry in WalkDir::new(&root_path)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| is_directory_included(e, &exclude_dir_glob_set, &default_dir_excludes))
        .filter_map(|e| e.ok())
    {
        // Skip directories
        if entry.file_type().is_dir() {
            continue;
        }
        
        let path = entry.path();
        
        // Check file size
        if let Ok(metadata) = path.metadata() {
            if metadata.len() > config.max_file_size {
                log::debug!("Skipping large file: {:?}", path);
                continue;
            }
        }
        
        // Check if file matches include/exclude patterns
        if !is_file_included(path, &include_glob_set, &exclude_glob_set) {
            continue;
        }
        
        // Read file content
        match fs::read_to_string(path) {
            Ok(content) => {
                // Get relative path from root
                if let Ok(rel_path) = path.strip_prefix(&root_path) {
                    let rel_path_str = rel_path.to_string_lossy().to_string();
                    files_map.insert(rel_path_str, content);
                }
            }
            Err(e) => {
                log::warn!("Failed to read file {:?}: {}", path, e);
            }
        }
    }
    
    Ok(files_map)
}

/// Check if a directory should be included in processing
fn is_directory_included(
    entry: &DirEntry,
    exclude_dir_glob_set: &GlobSet,
    default_excludes: &[&str],
) -> bool {
    if !entry.file_type().is_dir() {
        return true;
    }
    
    let path = entry.path();
    let dir_name = match path.file_name() {
        Some(name) => name.to_string_lossy(),
        None => return true,
    };
    
    // Check default excludes
    if default_excludes.iter().any(|&ex| dir_name == ex) {
        return false;
    }
    
    // Check glob patterns
    !exclude_dir_glob_set.is_match(path)
}

/// Check if a file should be included based on glob patterns
fn is_file_included(
    path: &Path,
    include_glob_set: &GlobSet,
    exclude_glob_set: &GlobSet,
) -> bool {
    // Must match include pattern
    if !include_glob_set.is_match(path) {
        return false;
    }
    
    // Must not match exclude pattern
    !exclude_glob_set.is_match(path)
}

/// Build a GlobSet from a list of patterns
fn build_glob_set(patterns: &[String]) -> Result<GlobSet> {
    let mut builder = GlobSetBuilder::new();
    
    for pattern in patterns {
        let glob = Glob::new(pattern)
            .context(format!("Invalid glob pattern: {}", pattern))?;
        builder.add(glob);
    }
    
    builder.build().context("Failed to build glob set")
}

/// Format files as XML
pub fn format_as_xml(
    files_map: &HashMap<String, String>,
    _pretty: bool, // Unused but kept for API consistency
) -> Result<String> {
    let files = files_map_to_vec(files_map);
    let repo = Repository { files };
    
    // Quick-xml doesn't support pretty printing in its simple API
    // For now, we'll just use the basic XML output
    xml_to_string(&repo)
        .context("Failed to serialize to XML")
}

/// Format files as JSON
pub fn format_as_json(
    files_map: &HashMap<String, String>,
    pretty: bool,
) -> Result<String> {
    let files = files_map_to_vec(files_map);
    let repo = Repository { files };
    
    if pretty {
        serde_json::to_string_pretty(&repo)
    } else {
        serde_json::to_string(&repo)
    }
    .context("Failed to serialize to JSON")
}

/// Format files as plain text
pub fn format_as_text(files_map: &HashMap<String, String>) -> Result<String> {
    let mut output = String::new();
    
    for (path, content) in files_map {
        output.push_str(&format!("# {}\n", path));
        output.push_str(content);
        output.push_str("\n\n");
    }
    
    Ok(output)
}

/// Convert files map to vector of RepoFile
fn files_map_to_vec(files_map: &HashMap<String, String>) -> Vec<RepoFile> {
    files_map
        .iter()
        .map(|(path, content)| RepoFile {
            path: path.clone(),
            content: content.clone(),
        })
        .collect()
}

/// Generate output in the specified format
pub fn generate_output(files_map: &HashMap<String, String>, config: &Config) -> Result<String> {
    match config.format {
        OutputFormat::Xml => format_as_xml(files_map, config.pretty_print),
        OutputFormat::Json => format_as_json(files_map, config.pretty_print),
        OutputFormat::Text => format_as_text(files_map),
    }
}

/// Write output to file or stdout
pub fn write_output(output: &str, config: &Config) -> Result<()> {
    match &config.output_file {
        Some(path) => {
            fs::write(path, output).context("Failed to write output to file")?;
            log::info!("Output written to {:?}", path);
        }
        None => {
            std::io::stdout()
                .write_all(output.as_bytes())
                .context("Failed to write output to stdout")?;
        }
    }
    
    Ok(())
}

/// Run the repository processing and generate output
pub fn run(config: Config) -> Result<()> {
    // Process repository
    let files_map = process_repository(&config)?;
    log::info!("Processed {} files", files_map.len());
    
    // Generate output
    let output = generate_output(&files_map, &config)?;
    
    // Write output
    write_output(&output, &config)?;
    
    Ok(())
}