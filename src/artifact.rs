use std::path::{Path, PathBuf};
use std::{fs, io};
use std::collections::HashSet;
use log::{debug, info, warn};
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
        debug!("Entering Artifact::collect");
        info!("Starting artifact collection from {}", config.source_dir.display());
        let mut artifacts = Vec::new();
        let ignored_dirs = config.get_ignored_dirs();
        let target_dirs = config.get_target_dirs();
        let excluded_extensions = config.get_excluded_extensions();
        let included_extensions = config.get_included_extensions();
        let mut processed_files = HashSet::new();

        debug!("Ignored dirs: {:?}", ignored_dirs);
        debug!("Target dirs: {:?}", target_dirs);
        debug!("Excluded extensions: {:?}", excluded_extensions);
        debug!("Included extensions: {:?}", included_extensions);

        let walker: Box<dyn Iterator<Item = Result<DirEntry, walkdir::Error>>> = if target_dirs.is_empty() {
            debug!("Processing entire source directory");
            Box::new(WalkDir::new(&config.source_dir).follow_links(true).into_iter())
        } else {
            debug!("Processing specified target directories: {:?}", target_dirs);
            Box::new(target_dirs.into_iter()
                .filter(|dir| config.source_dir.join(dir).exists())
                .flat_map(|dir| {
                    let full_path = config.source_dir.join(&dir);
                    debug!("Walking target directory: {}", full_path.display());
                    WalkDir::new(full_path).follow_links(true)
                })
                .into_iter())
        };

        for entry in walker.filter_map(Result::ok) {
            let path = entry.path().to_path_buf();
            debug!("Processing entry: {}", path.display());

            if path.is_file() && processed_files.insert(path.clone()) {
                let relative_path = path.strip_prefix(&config.source_dir).map_err(ArtifactError::StripPrefix)?;
                let is_ignored = Self::is_ignored(relative_path, &ignored_dirs);
                let is_excluded = Self::is_excluded(&path, &excluded_extensions);
                let is_included = Self::is_included(&path, &included_extensions);

                debug!("File: {}, ignored: {}, excluded: {}, included: {}",
                       path.display(), is_ignored, is_excluded, is_included);

                if !is_ignored && !is_excluded && is_included {
                    debug!("Creating artifact for file: {}", path.display());

                    match Self::new(path.clone(), &config.source_dir) {
                        Ok(artifact) => {
                            info!("Created artifact: {}", artifact.new_filename);
                            artifacts.push(artifact);
                        },
                        Err(e) => {
                            warn!("Failed to process file {}: {}", path.display(), e);
                        }
                    }
                } else {
                    debug!("Skipping file: {}", path.display());
                }
            }
        }

        info!("Artifact collection completed. Total artifacts: {}", artifacts.len());
        debug!("Exiting Artifact::collect");
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
    fn is_ignored(path: &Path, ignored_dirs: &[String]) -> bool {
        ignored_dirs.iter().any(|dir| path.starts_with(dir))
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
        if excluded_extensions.is_empty() {
            return false;
        }
        if let Some(extension) = path.extension() {
            let ext = extension.to_string_lossy().to_lowercase();
            excluded_extensions.iter().any(|excluded| *excluded == ext)
        } else {
            false
        }
    }

    /// Checks if a given file should be included based on its extension.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to check.
    /// * `included_extensions` - A slice of file extensions to include.
    ///
    /// # Returns
    ///
    /// Returns `true` if the file should be included, `false` otherwise.
    fn is_included(path: &Path, included_extensions: &[String]) -> bool {
        if included_extensions.is_empty() {
            return true;
        }
        if let Some(extension) = path.extension() {
            let ext = extension.to_string_lossy().to_lowercase();
            included_extensions.iter().any(|included| *included == ext)
        } else {
            false
        }
    }
}