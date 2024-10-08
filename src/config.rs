use clap::Parser;
use std::path::PathBuf;
use crate::config_file::ConfigFile;

/// Configuration options for the Rustifacts file preparation tool.
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

    /// Comma-separated list of file extensions to exclude (e.g., "jpg,png,pdf")
    #[arg(short = 'x', long, default_value = "")]
    pub excluded_extensions: String,

    /// Comma-separated list of file extensions to include (e.g., "rs,toml,md")
    #[arg(short = 'i', long, default_value = "")]
    pub included_extensions: String,

    /// Preset configuration to use (e.g., "nextjs")
    #[arg(long)]
    pub preset: Option<String>,

    /// Path to the configuration file
    #[arg(long, short = 'c')]
    pub config_file: Option<PathBuf>,
}

impl Config {
    /// Returns a vector of directories to ignore during file processing.
    ///
    /// This method combines a default list of commonly ignored directories
    /// with any additional directories specified by the user and the destination directory.
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

        // Add the destination directory to the ignored list
        if let Some(dest_dir_name) = self.dest_dir.file_name() {
            ignored_dirs.push(dest_dir_name.to_string_lossy().into_owned());
        }

        ignored_dirs.extend(self.additional_ignored_dirs
            .split(',')
            .filter(|s| !s.is_empty())
            .map(String::from));

        ignored_dirs
    }

    /// Returns a vector of target directories to process.
    ///
    /// If target directories are specified, only these directories will be processed.
    /// If no target directories are specified, an empty vector is returned,
    /// indicating that the entire source directory should be processed.
    ///
    /// # Returns
    ///
    /// A `Vec<PathBuf>` containing the target directories to process.
    pub fn get_target_dirs(&self) -> Vec<PathBuf> {
        self.target_dirs
            .as_ref()
            .map(|dirs| dirs.split(',').filter(|s| !s.is_empty()).map(PathBuf::from).collect())
            .unwrap_or_else(Vec::new)
    }

    /// Returns a vector of file extensions to exclude during processing.
    ///
    /// # Returns
    ///
    /// A `Vec<String>` containing all file extensions to be excluded.
    pub fn get_excluded_extensions(&self) -> Vec<String> {
        self.excluded_extensions
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().to_lowercase())
            .collect()
    }

    /// Returns a vector of file extensions to include during processing.
    ///
    /// # Returns
    ///
    /// A `Vec<String>` containing all file extensions to be included.
    pub fn get_included_extensions(&self) -> Vec<String> {
        self.included_extensions
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().to_lowercase())
            .collect()
    }

    /// Applies a preset configuration to the current Config instance.
    ///
    /// # Arguments
    ///
    /// * `preset_name` - The name of the preset to apply.
    ///
    /// # Returns
    ///
    /// Returns `Result<(), String>` indicating success or failure of applying the preset.
    pub fn apply_preset(&mut self, preset_name: &str) -> Result<(), String> {
        crate::presets::apply_preset(self, preset_name)
    }

    /// Applies configuration from a file to the current Config instance.
    ///
    /// # Returns
    ///
    /// Returns `anyhow::Result<()>` indicating success or failure of applying the configuration file.
    pub fn apply_config_file(&mut self) -> anyhow::Result<()> {
        if let Some(ref config_path) = self.config_file {
            let file_config = ConfigFile::read_from_file(config_path)?;
            file_config.apply_to_config(self);
        }
        Ok(())
    }
}