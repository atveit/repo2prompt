#[cfg(test)]
mod tests {
    use crate::{
        Config, OutputFormat, 
        format_as_json, format_as_text, format_as_xml, 
        process_repository
    };
    use std::collections::HashMap;
    use std::fs;
    use std::path::{Path, PathBuf};
    use tempfile::tempdir;
    use anyhow::Result;

    fn create_test_files(dir: &Path) -> Result<()> {
        // Create a python file
        fs::write(
            dir.join("test.py"),
            "def hello():\n    print('Hello, world!')\n",
        )?;
        
        // Create a markdown file
        fs::write(
            dir.join("README.md"),
            "# Test Project\n\nThis is a test project.\n",
        )?;
        
        // Create a directory to be excluded
        fs::create_dir(dir.join("node_modules"))?;
        fs::write(
            dir.join("node_modules/package.json"),
            r#"{"name": "test", "version": "1.0.0"}"#,
        )?;
        
        // Create a file to be excluded
        fs::write(dir.join("excluded.log"), "Some log data\n")?;
        
        Ok(())
    }

    #[test]
    fn test_process_repository() -> Result<()> {
        let temp_dir = tempdir()?;
        create_test_files(temp_dir.path())?;
        
        let config = Config {
            directory: PathBuf::from(temp_dir.path()),
            output_file: None,
            format: OutputFormat::Json,
            include_patterns: vec!["*.py".to_string(), "*.md".to_string()],
            exclude_patterns: Vec::new(),
            exclude_dir_patterns: vec!["node_modules".to_string()],
            max_file_size: 1_000_000,
            pretty_print: false,
        };
        
        let files_map = process_repository(&config)?;
        
        assert_eq!(files_map.len(), 2);
        assert!(files_map.contains_key("test.py"));
        assert!(files_map.contains_key("README.md"));
        assert!(!files_map.contains_key("excluded.log"));
        assert!(!files_map.contains_key("node_modules/package.json"));
        
        Ok(())
    }

    #[test]
    fn test_format_as_json() -> Result<()> {
        let mut files_map = HashMap::new();
        files_map.insert("test.py".to_string(), "def hello():\n    print('Hello, world!')\n".to_string());
        
        let json = format_as_json(&files_map, false)?;
        assert!(json.contains("test.py"));
        assert!(json.contains("def hello():"));
        
        let json_pretty = format_as_json(&files_map, true)?;
        assert!(json_pretty.contains("test.py"));
        assert!(json_pretty.contains("def hello():"));
        assert!(json_pretty.len() > json.len()); // Pretty should be longer due to whitespace
        
        Ok(())
    }

    #[test]
    fn test_format_as_xml() -> Result<()> {
        let mut files_map = HashMap::new();
        files_map.insert("test.py".to_string(), "def hello():\n    print('Hello, world!')\n".to_string());
        
        let xml = format_as_xml(&files_map, false)?;
        assert!(xml.contains("test.py"));
        assert!(xml.contains("def hello():"));
        
        // Since we removed pretty formatting for XML, the test will be simpler
        let xml_pretty = format_as_xml(&files_map, true)?;
        assert!(xml_pretty.contains("test.py"));
        assert!(xml_pretty.contains("def hello():"));
        
        Ok(())
    }

    #[test]
    fn test_format_as_text() -> Result<()> {
        let mut files_map = HashMap::new();
        files_map.insert("test.py".to_string(), "def hello():\n    print('Hello, world!')\n".to_string());
        
        let text = format_as_text(&files_map)?;
        assert!(text.contains("# test.py"));
        assert!(text.contains("def hello():"));
        
        Ok(())
    }
}