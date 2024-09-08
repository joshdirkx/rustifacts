# Rustifacts

Rustifacts is a command-line tool designed to prepare files for upload as artifacts to Claude AI projects. It processes files from a source directory, flattens the directory structure, and generates a collection of artifacts suitable for sharing with Claude.

## Features

- Recursively processes files from a source directory
- Flattens directory structure for easier navigation
- Renames files to avoid conflicts
- Ignores specified directories to exclude unnecessary files
- Supports file extension filtering (include/exclude)
- Provides preset configurations for common project types
- Supports custom configuration files
- Offers a flexible command-line interface

## Installation

To install Rustifacts, you need to have Rust and Cargo installed on your system. If you don't have them installed, you can get them from [rustup.rs](https://rustup.rs/).

Once you have Rust and Cargo set up, you can install Rustifacts by cloning this repository and building the project:

```bash
git clone https://github.com/yourusername/rustifacts.git
cd rustifacts
cargo build --release
```

The compiled binary will be available in the `target/release` directory.

## Usage

To use Rustifacts, run the following command:

```bash
rustifacts [OPTIONS]
```

### Options

- `-s, --source-dir <SOURCE_DIR>`: Specifies the source directory to process files from (default: current directory)
- `-d, --dest-dir <DEST_DIR>`: Specifies the destination directory for processed files (default: "./claude_files")
- `-a, --additional-ignored-dirs <DIRS>`: Comma-separated list of additional directories to ignore
- `-t, --target-dirs <DIRS>`: Comma-separated list of target directories to include (relative to source_dir)
- `-x, --excluded-extensions <EXTENSIONS>`: Comma-separated list of file extensions to exclude (e.g., "jpg,png,pdf")
- `-i, --included-extensions <EXTENSIONS>`: Comma-separated list of file extensions to include (e.g., "rs,toml,md")
- `--preset <PRESET>`: Preset configuration to use (e.g., "nextjs", "rust")
- `-c, --config-file <FILE>`: Path to a custom configuration file

### Examples

1. Process files from a `my_project` directory and output them to a `claude_ready` directory:

```bash
rustifacts -s ./my_project -d ./claude_ready
```

2. Use a preset configuration for a Next.js project:

```bash
rustifacts --preset nextjs -s ./my_nextjs_project
```

3. Use a custom configuration file:

```bash
rustifacts -c ./my_config.toml
```

## Configuration

### Default Ignored Directories

Rustifacts automatically ignores the following directories:

- `.git`
- `.idea`
- `.vscode`
- `node_modules`
- `target`
- `build`
- `dist`
- `__pycache__`

You can specify additional directories to ignore using the `-a` option.

### Presets

Rustifacts includes preset configurations for common project types. Currently supported presets are:

- `nextjs`: Optimized for Next.js projects
- `rust`: Optimized for Rust projects

To use a preset, specify it with the `--preset` option.

### Custom Configuration File

You can create a custom configuration file in TOML format. Here's an example:

```toml
source_dir = "./my_project"
dest_dir = "./claude_files"
additional_ignored_dirs = ["temp", "logs"]
target_dirs = ["src", "tests"]
excluded_extensions = ["exe", "dll"]
included_extensions = ["rs", "toml", "md"]
```

Use the `-c` option to specify the path to your configuration file.

## Output

Rustifacts will create the following in your destination directory:

1. Processed files with flattened names
2. A summary of the processed artifacts (coming soon)