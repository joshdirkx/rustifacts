use std::path::{Path, PathBuf};
use std::{fs, io};
use log::{info, warn};
use walkdir::{WalkDir, DirEntry};
use thiserror::Error;
use crate::config::Config;

/// Represents a file artifact to be processed and written.
pub struct Artifact {
    pub original_path: PathBuf,
    pub new_filename: String,
    pub content: String,
}

/// Custom error type for artifact-related operations.
#[derive(Error, Debug)]
pub enum ArtifactError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Path strip error: {0}")]
    StripPrefix(#[from] std::path::StripPrefixError),
}

impl Artifact {
    /// Creates a new `Artifact` instance.
    ///
    /// # Arguments
    ///
    /// * `original_path` - The original path of the file.
    /// * `source_dir` - The source directory path.
    ///
    /// # Returns
    ///
    /// Returns `Result<Self, ArtifactError>` containing the new `Artifact` if successful,
    /// or an `ArtifactError` if an error occurs during creation.
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

    /// Generates a new filename by replacing path separators with underscores.
    ///
    /// # Arguments
    ///
    /// * `relative_path` - The relative path of the file.
    ///
    /// # Returns
    ///
    /// A `String` containing the new filename.
    fn generate_new_filename(relative_path: &Path) -> String {
        relative_path.to_string_lossy().replace(std::path::MAIN_SEPARATOR, "_")
    }

    /// Writes the artifact content to the destination directory.
    ///
    /// # Arguments
    ///
    /// * `dest_dir` - The destination directory path.
    ///
    /// # Returns
    ///
    /// Returns `io::Result<()>` indicating success or failure of the write operation.
    pub fn write(&self, dest_dir: &Path) -> io::Result<()> {
        let dest_path = dest_dir.join(&self.new_filename);
        fs::write(dest_path, &self.content)
    }

    /// Collects artifacts from the source directory based on the provided configuration.
    ///
    /// If target directories are specified in the configuration, only files within those
    /// directories (and their subdirectories) will be processed. Otherwise, all files in
    /// the source directory will be processed, except those in ignored directories.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration options.
    ///
    /// # Returns
    ///
    /// Returns `Result<Vec<Self>, ArtifactError>` containing a vector of collected artifacts
    /// if successful, or an `ArtifactError` if an error occurs during collection.
    pub fn collect(config: &Config) -> Result<Vec<Self>, ArtifactError> {
        info!("Starting artifact collection from {}", config.source_dir.display());
        let mut artifacts = Vec::new();
        let ignored_dirs = config.get_ignored_dirs();
        let target_dirs = config.get_target_dirs();
        let excluded_extensions = config.get_excluded_extensions();

        let walker: Box<dyn Iterator<Item = Result<DirEntry, walkdir::Error>>> = if target_dirs.is_empty() {
            info!("Processing entire source directory");
            Box::new(WalkDir::new(&config.source_dir).follow_links(true).into_iter())
        } else {
            // ... (previous code for target_dirs remains unchanged)
        };

        for entry in walker.filter_map(|e| e.ok()) {
            let path = entry.path().to_path_buf();

            if path.is_file() && !Self::is_ignored(&path, &config.source_dir, &ignored_dirs) && !Self::is_excluded(&path, &excluded_extensions) {
                info!("Processing file: {}", path.display());

                match Self::new(path.clone(), &config.source_dir) {
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

    /// Checks if a given path should be ignored based on the ignored directories list.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to check.
    /// * `source_dir` - The source directory path.
    /// * `ignored_dirs` - A slice of ignored directory names.
    ///
    /// # Returns
    ///
    /// Returns `true` if the path should be ignored, `false` otherwise.
    fn is_ignored(path: &Path, source_dir: &Path, ignored_dirs: &[String]) -> bool {
        for ignored_dir in ignored_dirs {
            let ignored_path = source_dir.join(ignored_dir);
            if path.starts_with(&ignored_path) {
                return true;
            }
        }
        false
    }

    /// Writes all artifacts to the destination directory.
    ///
    /// # Arguments
    ///
    /// * `artifacts` - A slice of `Artifact` instances to write.
    /// * `dest_dir` - The destination directory path.
    ///
    /// # Returns
    ///
    /// Returns `io::Result<()>` indicating success or failure of the write operations.
    pub fn write_all(artifacts: &[Self], dest_dir: &Path) -> io::Result<()> {
        fs::create_dir_all(dest_dir)?;
        for artifact in artifacts {
            artifact.write(dest_dir)?;
        }
        Ok(())
    }

    /// Checks if a given file should be excluded based on its extension.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to check.
    /// * `excluded_extensions` - A slice of file extensions to exclude.
    ///
    /// # Returns
    ///
    /// Returns `true` if the file should be excluded, `false` otherwise.
    fn is_excluded(path: &Path, excluded_extensions: &[String]) -> bool {
        if let Some(extension) = path.extension() {
            let ext = extension.to_string_lossy().to_lowercase();
            excluded_extensions.iter().any(|excluded| *excluded == ext)
        } else {
            false
        }
    }
}