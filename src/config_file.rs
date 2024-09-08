use std::fs;
use std::path::Path;
use serde::Deserialize;
use toml;
use anyhow::{Result, Context};
use crate::config::Config;

#[derive(Deserialize, Debug)]
pub struct ConfigFile {
    pub source_dir: Option<String>,
    pub dest_dir: Option<String>,
    pub additional_ignored_dirs: Option<Vec<String>>,
    pub target_dirs: Option<Vec<String>>,
    pub excluded_extensions: Option<Vec<String>>,
    pub included_extensions: Option<Vec<String>>,
}

impl ConfigFile {
    /// Reads and parses a configuration file.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the configuration file.
    ///
    /// # Returns
    ///
    /// Returns `Result<Self, anyhow::Error>` containing the parsed ConfigFile if successful,
    /// or an error if reading or parsing fails.
    pub fn read_from_file(path: &Path) -> Result<Self> {
        let contents = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;
        let config: ConfigFile = toml::from_str(&contents)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))?;
        Ok(config)
    }

    /// Applies the configuration from the file to the given Config instance.
    ///
    /// # Arguments
    ///
    /// * `config` - The Config instance to update.
    pub fn apply_to_config(&self, config: &mut Config) {
        if let Some(ref source_dir) = self.source_dir {
            config.source_dir = source_dir.into();
        }
        if let Some(ref dest_dir) = self.dest_dir {
            config.dest_dir = dest_dir.into();
        }
        if let Some(ref ignored_dirs) = self.additional_ignored_dirs {
            config.additional_ignored_dirs = ignored_dirs.join(",");
        }
        if let Some(ref target_dirs) = self.target_dirs {
            config.target_dirs = Some(target_dirs.join(","));
        }
        if let Some(ref excluded_exts) = self.excluded_extensions {
            config.excluded_extensions = excluded_exts.join(",");
        }
        if let Some(ref included_exts) = self.included_extensions {
            config.included_extensions = included_exts.join(",");
        }
    }
}