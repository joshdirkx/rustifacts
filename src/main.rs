use std::process;
use log::{error, info, debug};
use env_logger::Env;
use clap::Parser;
use artifact::Artifact;
use config::Config;

mod config;
mod artifact;
mod presets;
mod config_file;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info,rustifacts=debug")).init();

    debug!("Starting Rustifacts");

    let mut config = Config::parse();

    debug!("Parsed initial config: {:?}", config);

    // Apply configuration file if specified
    if let Some(ref config_path) = config.config_file {
        debug!("Applying configuration from file: {}", config_path.display());
        if let Err(e) = config.apply_config_file() {
            error!("Failed to apply configuration file: {}", e);
            process::exit(1);
        }
    }

    debug!("Config after applying config file: {:?}", config);

    // Apply preset if specified
    if let Some(preset_name) = config.preset.take() {
        debug!("Applying preset: {}", preset_name);
        if let Err(e) = config.apply_preset(&preset_name) {
            error!("Failed to apply preset: {}", e);
            process::exit(1);
        }
    }

    debug!("Final config: {:?}", config);

    // Log configuration details
    info!("Starting file preparation process");
    info!("Source directory: {}", config.source_dir.display());
    info!("Destination directory: {}", config.dest_dir.display());
    info!("Ignored directories: {:?}", config.get_ignored_dirs());
    info!("Excluded file types: {:?}", config.get_excluded_extensions());
    if let Some(ref target_dirs) = config.target_dirs {
        info!("Target directories: {}", target_dirs);
    } else {
        info!("Processing entire source directory");
    }

    // Collect and process artifacts
    debug!("Starting artifact collection and processing");
    match collect_and_process_artifacts(&config) {
        Ok(_) => info!("File preparation completed successfully"),
        Err(e) => {
            error!("Error during file preparation: {}", e);
            process::exit(1);
        }
    }

    debug!("Rustifacts completed");
}

fn collect_and_process_artifacts(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    debug!("Collecting artifacts");
    let artifacts = Artifact::collect(config)?;
    debug!("Writing artifacts");
    Artifact::write_all(&artifacts, &config.dest_dir)?;
    Ok(())
}