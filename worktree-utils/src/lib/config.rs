use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectConfig {
    pub path: String,
    pub worktrees: std::collections::HashMap<String, String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub projects: std::collections::HashMap<String, ProjectConfig>,
}

impl Config {
    pub fn get_path() -> std::path::PathBuf {
        home::home_dir()
            .unwrap()
            .join(".config")
            .join("worktree-utils")
            .join("config.json")
    }
}

fn load_config_from_json(path: &std::path::Path) -> Result<Config, Box<dyn std::error::Error>> {
    let buf = std::fs::read(path)?;
    let text = String::from_utf8(buf)?;
    let config = serde_json::from_str(&text)?;
    Ok(config)
}

fn write_config_to_json(
    config: &Config,
    path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::write(path, serde_json::to_string(config)?)?;
    Ok(())
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = Config::get_path();
    std::fs::DirBuilder::new()
        .recursive(true)
        .create(config_path.parent().unwrap())?;
    let config = if !config_path.exists() {
        let config = Config::default();
        write_config_to_json(&config, &config_path)?;
        config
    } else {
        load_config_from_json(&config_path)?
    };
    Ok(config)
}

pub fn update_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = Config::get_path();
    write_config_to_json(config, &config_path)
}
