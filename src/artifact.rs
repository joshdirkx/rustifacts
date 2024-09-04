use std::path::{Path, PathBuf};
use std::{fs, io};
use log::{info, warn};
use walkdir::WalkDir;
use std::fs::File;
use std::io::Write;
use thiserror::Error;
use crate::config::Config;

pub struct Artifact {
    pub original_path: PathBuf,
    pub new_filename: String,
    pub content: String,
}

#[derive(Error, Debug)]
pub enum ArtifactError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Path strip error: {0}")]
    StripPrefix(#[from] std::path::StripPrefixError),
}

impl Artifact {
    pub fn new(original_path: PathBuf, source_dir: &Path) -> Result<Self, ArtifactError> {
        let relative_path = original_path.strip_prefix(source_dir)?;
        let new_filename = Self::generate_new_filename(relative_path);
        let content = fs::read_to_string(&original_path)?;

        Ok(Self {
            original_path,
            new_filename,
            content,
        })
    }

    fn generate_new_filename(relative_path: &Path) -> String {
        relative_path.to_string_lossy().replace(std::path::MAIN_SEPARATOR, "_")
    }

    pub fn write(&self, dest_dir: &Path) -> io::Result<()> {
        let dest_path = dest_dir.join(&self.new_filename);
        fs::write(dest_path, &self.content)
    }

    pub fn collect(config: &Config) -> Result<Vec<Self>, ArtifactError> {
        info!("Starting artifact collection from {}", config.source_dir.display());
        let mut artifacts = Vec::new();
        let ignored_dirs = config.get_ignored_dirs();

        for entry in WalkDir::new(&config.source_dir).follow_links(true).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();

            if path.is_file() && !Self::is_ignored(path, &config.source_dir, &ignored_dirs) {
                info!("Processing file: {}", path.display());

                match Self::new(path.to_path_buf(), &config.source_dir) {
                    Ok(artifact) => artifacts.push(artifact),
                    Err(e) => {
                        warn!("Failed to process file {}: {}", path.display(), e);
                        continue;
                    }
                }
            }
        }

        info!("Artifact collection completed. Total artifacts: {}", artifacts.len());
        Ok(artifacts)
    }

    fn is_ignored(path: &Path, source_dir: &Path, ignored_dirs: &[String]) -> bool {
        for ignored_dir in ignored_dirs {
            if path.starts_with(source_dir.join(ignored_dir)) {
                return true;
            }
        }
        false
    }

    pub fn write_all(artifacts: &[Self], dest_dir: &Path) -> io::Result<()> {
        fs::create_dir_all(dest_dir)?;
        for artifact in artifacts {
            artifact.write(dest_dir)?;
        }
        Ok(())
    }
}