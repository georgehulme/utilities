use crate::{
    config::{self, Config},
    error::ProjectNotExistsError,
    worktree,
};

pub fn list_projects() {
    let config = config::load_config_from_file();
    let mut table = prettytable::Table::new();
    table.add_row(prettytable::row!["Project", "Path"]);
    for project in config.projects {
        table.add_row(prettytable::row![project.0, project.1.path]);
    }
    table.printstd();
}

pub fn print_project_path(
    config: &mut Config,
    project_name: String,
) -> Result<(), ProjectNotExistsError> {
    println!(
        "{}",
        match config.projects.get(&project_name) {
            Some(project_config) => Ok(project_config.path.to_owned()),
            None => Err(ProjectNotExistsError(project_name)),
        }?
    );
    Ok(())
}

pub fn add_project(config: &mut Config, project_name: String, path: String, inherit: bool) {
    let entry = config.get_vacant_project_entry(project_name).unwrap();
    let repo = git2::Repository::open(&path).unwrap();
    let worktrees = if inherit {
        std::collections::HashMap::from_iter(
            repo.worktrees()
                .unwrap()
                .into_iter()
                .filter(Option::is_some)
                .map(|worktree| {
                    (
                        String::from(worktree.unwrap()),
                        String::from(
                            repo.find_worktree(worktree.unwrap())
                                .unwrap()
                                .path()
                                .to_str()
                                .unwrap(),
                        ),
                    )
                }),
        )
    } else {
        std::collections::HashMap::new()
    };
    entry.insert(config::ProjectConfig { path, worktrees });
    config::write_config_to_file(config);
}

pub fn rm_project(config: &mut Config, project_name: String, keep: bool) {
    let entry = config.get_occupied_project_entry(project_name).unwrap();
    let mut project = entry.remove();
    if !keep {
        let worktrees = project.worktrees.to_owned();
        for (worktree_name, _) in worktrees {
            worktree::rm_worktree(&mut project, worktree_name, keep);
        }
    }
    config::write_config_to_file(config);
}
