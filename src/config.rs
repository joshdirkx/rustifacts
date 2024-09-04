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
    #[arg(short, long, default_value = "")]
    pub additional_ignored_dirs: String,
}

impl Config {
    pub fn get_ignored_dirs(&self) -> Vec<String> {
        let mut ignored_dirs = vec![
            ".git".to_string(),
            ".idea".to_string(),
            ".vscode".to_string(),
            "node_modules".to_string(),
            "target".to_string(),
            "build".to_string(),
            "dist".to_string(),
            "__pycache__".to_string(),
        ];

        ignored_dirs.extend(self.additional_ignored_dirs
            .split(',')
            .filter(|s| !s.is_empty())
            .map(String::from));

        ignored_dirs
    }
}