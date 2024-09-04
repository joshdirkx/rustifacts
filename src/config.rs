use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Source directory to process files from
    #[arg(short, long, default_value = ".")]
    pub source_dir: PathBuf,

    /// Destination directory to copy processed files to
    #[arg(short, long, default_value = "./claude_files")]
    pub dest_dir: PathBuf,

    /// Comma-separated list of directories to ignore
    #[arg(short, long, default_value = "target,frontend/target,frontend/pkg")]
    pub ignored_dirs: String,
}