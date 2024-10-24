use std::collections::hash_map::{Entry, OccupiedEntry, VacantEntry};

use serde::{Deserialize, Serialize};

use crate::error::{
    ProjectExistsError, ProjectNotExistsError, WorktreeExistsError, WorktreeNotExistsError,
};

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

    pub fn get_vacant_project_entry(
        &mut self,
        project_name: String,
    ) -> Result<VacantEntry<String, ProjectConfig>, ProjectExistsError> {
        let entry = self.projects.entry(project_name);
        if let Entry::Vacant(vacant_entry) = entry {
            Ok(vacant_entry)
        } else {
            Err(ProjectExistsError(entry.key().to_owned()))
        }
    }

    pub fn get_occupied_project_entry(
        &mut self,
        project_name: String,
    ) -> Result<OccupiedEntry<String, ProjectConfig>, ProjectNotExistsError> {
        let entry = self.projects.entry(project_name);
        if let Entry::Occupied(occupied_entry) = entry {
            Ok(occupied_entry)
        } else {
            Err(ProjectNotExistsError(entry.key().to_owned()))
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectConfig {
    pub path: String,
    pub worktrees: std::collections::HashMap<String, String>,
}

impl ProjectConfig {
    pub fn get_vacant_worktree_entry(
        &mut self,
        worktree_name: String,
    ) -> Result<VacantEntry<String, String>, WorktreeExistsError> {
        let entry = self.worktrees.entry(worktree_name);
        if let Entry::Vacant(vacant_entry) = entry {
            Ok(vacant_entry)
        } else {
            Err(WorktreeExistsError(entry.key().to_owned()))
        }
    }

    pub fn get_occupied_worktree_entry(
        &mut self,
        worktree_name: String,
    ) -> Result<OccupiedEntry<String, String>, WorktreeNotExistsError> {
        let entry = self.worktrees.entry(worktree_name);
        if let Entry::Occupied(occupied_entry) = entry {
            Ok(occupied_entry)
        } else {
            Err(WorktreeNotExistsError(entry.key().to_owned()))
        }
    }
}

fn load_config_from_json(path: &std::path::Path) -> Config {
    let buf = std::fs::read(path).unwrap();
    let text = String::from_utf8(buf).unwrap();
    serde_json::from_str(&text).unwrap()
}

fn write_config_to_json(config: &Config, path: &std::path::Path) {
    std::fs::write(path, serde_json::to_string(config).unwrap()).unwrap();
}

pub fn load_config_from_file() -> Config {
    let config_path = Config::get_path();
    std::fs::DirBuilder::new()
        .recursive(true)
        .create(config_path.parent().unwrap())
        .unwrap();
    if !config_path.exists() {
        let config = Config::default();
        write_config_to_json(&config, &config_path);
        config
    } else {
        load_config_from_json(&config_path)
    }
}

pub fn write_config_to_file(config: &Config) {
    let config_path = Config::get_path();
    write_config_to_json(config, &config_path);
}
