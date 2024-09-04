# Rustifacts

Rustifacts is a command-line tool designed to prepare files for easy upload to Claude AI projects. It simplifies the process of organizing and formatting your project files, making it easier to share your code with Claude.

## Features

- Recursively processes files from a source directory
- Flattens directory structure for easier navigation
- Renames files to avoid conflicts
- Ignores specified directories to exclude unnecessary files
- Provides a simple command-line interface

## Installation

To install Rustifacts, you need to have Rust and Cargo installed on your system. If you don't have them installed, you can get them from [rustup.rs](https://rustup.rs/).

Once you have Rust and Cargo set up, you can install Rustifacts by cloning this repository and building the project:

```bash
git clone https://github.com/joshdirkx/rustifacts.git
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

### Example

To process files from a `my_project` directory and output them to a `claude_ready` directory, ignoring `tests` and `docs` directories:

```bash
rustifacts -s ./my_project -d ./claude_ready -a tests,docs
```

## Configuration

Rustifacts automatically ignores common directories like `.git`, `node_modules`, and `target`. You can specify additional directories to ignore using the `-a` option.

## Output

Rustifacts will create the following in your destination directory:

1. Processed files with flattened names

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.