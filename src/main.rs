use std::process;
use log::{error, info};
use env_logger::Env;
use clap::Parser;
use artifact::Artifact;
use config::Config;

mod config;
mod artifact;
mod presets;

/// The main entry point of the Rustifacts application.
///
/// This function initializes the logger, parses the command-line arguments,
/// and runs the main logic of the application.
fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let mut config = Config::parse();

    if let Some(preset_name) = config.preset.take() {
        info!("Applying preset: {}", preset_name);
        if let Err(e) = config.apply_preset(&preset_name) {
            error!("Failed to apply preset: {}", e);
            process::exit(1);
        }
    }

    if let Err(e) = run(config) {
        error!("Application error: {}", e);
        process::exit(1);
    }
}

/// Runs the main logic of the Rustifacts application.
///
/// This function processes the files according to the provided configuration,
/// collects artifacts, and writes them to the destination directory.
///
/// # Arguments
///
/// * `config` - The configuration options parsed from command-line arguments.
///
/// # Returns
///
/// Returns `Ok(())` if the process completes successfully, or an `Err` containing
/// the error if any part of the process fails.
fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting file preparation process");
    info!("Source directory: {}", config.source_dir.display());
    info!("Destination directory: {}", config.dest_dir.display());
    info!("Ignored directories: {:?}", config.get_ignored_dirs());
    info!("Excluded file types: {:?}", config.get_excluded_extensions());

    if let Some(target_dirs) = &config.target_dirs {
        info!("Target directories: {}", target_dirs);
    } else {
        info!("Processing entire source directory");
    }

    let artifacts = Artifact::collect(&config)?;

    Artifact::write_all(&artifacts, &config.dest_dir)?;

    info!("File preparation completed successfully");
    Ok(())
}