use std::path::{Path, PathBuf};
use std::{fs, io};
use log::{info, warn};
use walkdir::WalkDir;
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
            if path.starts_with(source_dir.join(ignored_dir)) {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    fn setup_test_directory() -> (TempDir, PathBuf) {
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().join("source");
        fs::create_dir(&source_dir).unwrap();

        // Create some test files
        let file1_path = source_dir.join("file1.txt");
        let mut file1 = File::create(file1_path).unwrap();
        writeln!(file1, "Content of file 1").unwrap();

        let subdir_path = source_dir.join("subdir");
        fs::create_dir(&subdir_path).unwrap();
        let file2_path = subdir_path.join("file2.txt");
        let mut file2 = File::create(file2_path).unwrap();
        writeln!(file2, "Content of file 2").unwrap();

        (temp_dir, source_dir)
    }

    #[test]
    fn test_artifact_new() {
        let (_temp_dir, source_dir) = setup_test_directory();
        let file_path = source_dir.join("file1.txt");

        let artifact = Artifact::new(file_path.clone(), &source_dir).unwrap();

        assert_eq!(artifact.original_path, file_path);
        assert_eq!(artifact.new_filename, "file1.txt");
        assert_eq!(artifact.content, "Content of file 1\n");
    }

    #[test]
    fn test_artifact_new_subdir() {
        let (_temp_dir, source_dir) = setup_test_directory();
        let file_path = source_dir.join("subdir").join("file2.txt");

        let artifact = Artifact::new(file_path.clone(), &source_dir).unwrap();

        assert_eq!(artifact.original_path, file_path);
        assert_eq!(artifact.new_filename, "subdir_file2.txt");
        assert_eq!(artifact.content, "Content of file 2\n");
    }

    #[test]
    fn test_artifact_write() {
        let (temp_dir, source_dir) = setup_test_directory();
        let file_path = source_dir.join("file1.txt");
        let artifact = Artifact::new(file_path, &source_dir).unwrap();

        let dest_dir = temp_dir.path().join("dest");
        fs::create_dir(&dest_dir).unwrap();

        artifact.write(&dest_dir).unwrap();

        let written_content = fs::read_to_string(dest_dir.join("file1.txt")).unwrap();
        assert_eq!(written_content, "Content of file 1\n");
    }

    #[test]
    fn test_artifact_collect() {
        let (temp_dir, source_dir) = setup_test_directory();

        let config = Config {
            source_dir: source_dir.clone(),
            dest_dir: temp_dir.path().join("dest"),
            additional_ignored_dirs: "".to_string(),
        };

        let artifacts = Artifact::collect(&config).unwrap();

        assert_eq!(artifacts.len(), 2);
        assert!(artifacts.iter().any(|a| a.new_filename == "file1.txt"));
        assert!(artifacts.iter().any(|a| a.new_filename == "subdir_file2.txt"));
    }

    #[test]
    fn test_is_ignored() {
        let (_temp_dir, source_dir) = setup_test_directory();
        let ignored_dirs = vec!["node_modules".to_string(), "target".to_string()];

        assert!(!Artifact::is_ignored(&source_dir.join("file1.txt"), &source_dir, &ignored_dirs));
        assert!(!Artifact::is_ignored(&source_dir.join("subdir").join("file2.txt"), &source_dir, &ignored_dirs));
        assert!(Artifact::is_ignored(&source_dir.join("node_modules").join("file3.txt"), &source_dir, &ignored_dirs));
        assert!(Artifact::is_ignored(&source_dir.join("target").join("debug").join("file4.txt"), &source_dir, &ignored_dirs));
    }

    #[test]
    fn test_write_all() {
        let (temp_dir, source_dir) = setup_test_directory();
        let dest_dir = temp_dir.path().join("dest");

        let artifacts = vec![
            Artifact::new(source_dir.join("file1.txt"), &source_dir).unwrap(),
            Artifact::new(source_dir.join("subdir").join("file2.txt"), &source_dir).unwrap(),
        ];

        Artifact::write_all(&artifacts, &dest_dir).unwrap();

        assert!(dest_dir.join("file1.txt").exists());
        assert!(dest_dir.join("subdir_file2.txt").exists());
        assert_eq!(fs::read_to_string(dest_dir.join("file1.txt")).unwrap(), "Content of file 1\n");
        assert_eq!(fs::read_to_string(dest_dir.join("subdir_file2.txt")).unwrap(), "Content of file 2\n");
    }
}