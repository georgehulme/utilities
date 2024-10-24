use crate::{config, worktree};

pub fn list_projects() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::load_config()?;
    let mut table = prettytable::Table::new();
    table.add_row(prettytable::row!["Project", "Path"]);
    for project in config.projects {
        table.add_row(prettytable::row![project.0, project.1.path]);
    }
    table.printstd();
    Ok(())
}

pub fn print_project_path(
    project_name: String
) -> Result<(), Box<dyn std::error::Error>> {
    let config = config::load_config()?;
    if let Some(project) = config.projects.get(&project_name) {
        println!("{}", &project.path);
    }
    Ok(())
}

pub fn add_project(project_name: String, path: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = config::load_config()?;
    if let std::collections::hash_map::Entry::Vacant(entry) =
        config.projects.entry(project_name.clone())
    {
        entry.insert(config::ProjectConfig {
            path,
            worktrees: std::collections::HashMap::new(),
        });
        config::update_config(&config)?;
    } else {
        println!("Project, {}, already exists", project_name);
    };
    Ok(())
}

pub fn rm_project(project_name: String, keep: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = config::load_config()?;
    if let std::collections::hash_map::Entry::Occupied(entry) =
        config.projects.entry(project_name.clone())
    {
        let project = entry.remove();
        if !keep {
            for worktree_name in project.worktrees.values() {
                worktree::rm_worktree(project_name.clone(), worktree_name.clone())?;
            }
        }
        config::update_config(&config)?;
    } else {
        println!("Project, {}, does not exist", project_name);
    };
    Ok(())
}
