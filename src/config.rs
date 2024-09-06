use clap::Parser;
use std::path::PathBuf;

/// Configuration options for the file preparation tool.
///
/// This struct is derived from `clap::Parser` to automatically generate
/// a command-line interface for setting these options.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Source directory to process files from
    #[arg(short, long, default_value = ".")]
    pub source_dir: PathBuf,

    /// Destination directory to copy processed files to
    #[arg(short, long, default_value = "./claude_files")]
    pub dest_dir: PathBuf,

    /// Comma-separated list of additional directories to ignore
    #[arg(short, long, default_value = "")]
    pub additional_ignored_dirs: String,

    /// Comma-separated list of target directories to include (relative to source_dir)
    #[arg(short, long)]
    pub target_dirs: Option<String>,
}

impl Config {
    /// Returns a vector of directories to ignore during file processing.
    ///
    /// This method combines a default list of commonly ignored directories
    /// with any additional directories specified by the user.
    ///
    /// # Returns
    ///
    /// A `Vec<String>` containing all directories to be ignored.
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


    pub fn get_target_dirs(&self) -> Vec<PathBuf> {
        self.target_dirs
            .as_ref()
            .map(|dirs| dirs.split(',').map(PathBuf::from).collect())
            .unwrap_or_else(Vec::new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_config_default_values() {
        let args = vec!["program"];
        let config = Config::parse_from(args);

        assert_eq!(config.source_dir, PathBuf::from("."));
        assert_eq!(config.dest_dir, PathBuf::from("./claude_files"));
        assert_eq!(config.additional_ignored_dirs, "");
    }

    #[test]
    fn test_config_custom_values() {
        let args = vec![
            "program",
            "--source-dir", "/path/to/source",
            "--dest-dir", "/path/to/dest",
            "--additional-ignored-dirs", "node_modules,dist"
        ];
        let config = Config::parse_from(args);

        assert_eq!(config.source_dir, PathBuf::from("/path/to/source"));
        assert_eq!(config.dest_dir, PathBuf::from("/path/to/dest"));
        assert_eq!(config.additional_ignored_dirs, "node_modules,dist");
    }

    #[test]
    fn test_get_ignored_dirs() {
        let config = Config {
            source_dir: PathBuf::from("."),
            dest_dir: PathBuf::from("./claude_files"),
            additional_ignored_dirs: "custom_dir,another_dir".to_string(),
        };

        let ignored_dirs = config.get_ignored_dirs();

        // Check if default ignored dirs are present
        assert!(ignored_dirs.contains(&".git".to_string()));
        assert!(ignored_dirs.contains(&"target".to_string()));

        // Check if additional ignored dirs are present
        assert!(ignored_dirs.contains(&"custom_dir".to_string()));
        assert!(ignored_dirs.contains(&"another_dir".to_string()));

        // Check total count (8 default + 2 additional)
        assert_eq!(ignored_dirs.len(), 10);
    }
}