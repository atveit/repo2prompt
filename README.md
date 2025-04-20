# repo2prompt

A Rust tool for extracting repository content into structured formats (XML, JSON, or text) for use with LLMs and other tools.

## Features

- **Multiple Output Formats**: XML, JSON, or plain text with file markers
- **Flexible File Selection**: Include/exclude files using glob patterns
- **Directory Filtering**: Skip irrelevant directories
- **Size Limits**: Skip files larger than a specified size
- **Pretty Printing**: Formatted output for XML and JSON

## Installation

### From crates.io

```bash
cargo install repo2prompt
```

### From source

```bash
git clone https://github.com/user/repo2prompt.git
cd repo2prompt
cargo install --path .
```

## Usage

```
repo2prompt [OPTIONS]
```

### Options

```
  -d, --directory <DIRECTORY>        Root directory to process [default: .]
  -o, --output <o>              Output file (default: stdout)
  -f, --format <FORMAT>              Output format [default: xml] [possible values: xml, json, text]
  -i, --include <INCLUDE>            File patterns to include (can specify multiple)
  -e, --exclude <EXCLUDE>            File patterns to exclude (can specify multiple)
      --exclude-dirs <EXCLUDE_DIRS>  Directory patterns to exclude (can specify multiple)
      --max-size <MAX_SIZE>          Maximum file size in bytes [default: 1000000]
      --pretty                       Pretty-print output (for XML and JSON)
  -v, --verbose                      Enable verbose logging
  -h, --help                         Print help
  -V, --version                      Print version
```

## Examples

### Command Line Examples

#### Basic usage (XML output to stdout)

```bash
repo2prompt
```

#### Generate JSON output for Python and Markdown files

```bash
repo2prompt -f json -i "*.py" -i "*.md" --pretty
```

#### Save text output to a file, excluding tests

```bash
repo2prompt -f text -o repo-content.txt -e "*test*" -e "*spec*"
```

#### Process a specific directory, excluding multiple directories

```bash
repo2prompt -d /path/to/project --exclude-dirs "node_modules" --exclude-dirs "target"
```

### Library Usage Examples

The repo2prompt crate can also be used as a library in your Rust projects. Check the `examples/` directory for runnable examples:

```bash
# Run the basic example
cargo run --example basic_usage

# Run the JSON output example
cargo run --example json_output

# Run the programmatic processing example
cargo run --example programmatic_processing

# Run the custom filtering example
cargo run --example custom_filtering
```

Basic library usage:

```rust
use std::path::PathBuf;
use repo2prompt::{Config, OutputFormat, run};

// Create a configuration
let config = Config {
    directory: PathBuf::from("path/to/repo"),
    output_file: Some(PathBuf::from("output.json")),
    format: OutputFormat::Json,
    include_patterns: vec!["*.rs".to_string()],
    exclude_patterns: vec![],
    exclude_dir_patterns: vec!["target".to_string()],
    max_file_size: 1_000_000,
    pretty_print: true,
};

// Run the extraction
run(config)?;
```

## Output Formats

### XML

```xml
<repository>
  <files>
    <file path="src/main.rs">
      <content><![CDATA[// File content here]]></content>
    </file>
    <!-- More files -->
  </files>
</repository>
```

### JSON

```json
{
  "files": [
    {
      "path": "src/main.rs",
      "content": "// File content here"
    }
  ]
}
```

### Text

```
# src/main.rs
// File content here

# src/lib.rs
// File content here
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Copyright

Copyright © 2025 Amund Tveit