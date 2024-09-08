use std::collections::HashMap;
use crate::config::Config;

#[derive(Clone, Debug)]
pub struct PresetConfig {
    pub ignored_dirs: Vec<String>,
    pub included_extensions: Vec<String>,
    pub excluded_extensions: Vec<String>,
    pub target_dirs: Vec<String>,
}

/// Returns a HashMap of preset configurations.
///
/// # Returns
///
/// A `HashMap<String, PresetConfig>` containing predefined preset configurations.
pub fn get_preset_configs() -> HashMap<String, PresetConfig> {
    let mut presets = HashMap::new();

    presets.insert(
        "nextjs".to_string(),
        PresetConfig {
            ignored_dirs: vec![
                "node_modules".to_string(),
                ".next".to_string(),
                "out".to_string(),
                ".git".to_string(),
            ],
            included_extensions: vec![
                "js".to_string(), "jsx".to_string(), "ts".to_string(), "tsx".to_string(),
                "json".to_string(), "md".to_string(), "css".to_string(),
            ],
            excluded_extensions: vec![],
            target_dirs: vec![
                ".".to_string(),
                "src".to_string(),
                "components".to_string(),
                "styles".to_string(),
                "public".to_string(),
            ],
        }
    );

    presets.insert(
        "rust".to_string(),
        PresetConfig {
            ignored_dirs: vec![
                "target".to_string(),
                ".git".to_string(),
                ".idea".to_string(),
                ".vscode".to_string(),
            ],
            included_extensions: vec![
                "rs".to_string(),
                "toml".to_string(),
                "md".to_string(),
                "json".to_string(),
                "yml".to_string(),
                "yaml".to_string(),
            ],
            excluded_extensions: vec![],
            target_dirs: vec![
                ".".to_string(),
                "src".to_string(),
                "tests".to_string(),
                "examples".to_string(),
                "benches".to_string(),
            ],
        }
    );

    presets
}

/// Applies a preset configuration to the given Config instance.
///
/// # Arguments
///
/// * `config` - The Config instance to update.
/// * `preset_name` - The name of the preset to apply.
///
/// # Returns
///
/// Returns `Result<(), String>` indicating success or failure of applying the preset.
pub fn apply_preset(config: &mut Config, preset_name: &str) -> Result<(), String> {
    if let Some(preset) = get_preset_configs().get(preset_name) {
        config.additional_ignored_dirs = preset.ignored_dirs.join(",");
        config.included_extensions = preset.included_extensions.join(",");
        config.excluded_extensions = preset.excluded_extensions.join(",");
        config.target_dirs = Some(preset.target_dirs.join(","));
        Ok(())
    } else {
        Err(format!("Preset '{}' not found", preset_name))
    }
}