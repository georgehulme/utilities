use std::str::FromStr;

use crate::config;

pub fn list_worktrees(project_name: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = config::load_config()?;
    if let Some(project) = config.projects.get_mut(&project_name) {
        let mut table = prettytable::Table::new();
        table.add_row(prettytable::row!["Worktree", "Path"]);
        table.add_row(prettytable::row!["<root>", &project.path]);
        for worktree in &project.worktrees {
            table.add_row(prettytable::row![worktree.0, worktree.1]);
        }
        table.printstd();
    } else {
        println!("Project, {project_name}, does not exist!");
    }
    Ok(())
}

pub fn print_worktree_path(
    project_name: String,
    worktree_name: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = config::load_config()?;
    if let Some(project) = config.projects.get(&project_name) {
        if let Some(worktree) = project.worktrees.get(&worktree_name) {
            println!("{}", worktree);
        }
    }
    Ok(())
}

pub fn add_worktree(
    project_name: String,
    worktree_name: String,
    branch: String,
    path: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = config::load_config()?;
    if let Some(project) = config.projects.get_mut(&project_name) {
        if let std::collections::hash_map::Entry::Vacant(entry) =
            project.worktrees.entry(worktree_name.clone())
        {
            let repo = git2::Repository::open(&project.path)?;
            repo.worktree(
                &worktree_name,
                std::path::PathBuf::from_str(&path)?.as_path(),
                Some(
                    git2::WorktreeAddOptions::new()
                        .checkout_existing(true)
                        .reference(Some(
                            &repo
                                .find_branch(&branch, git2::BranchType::Local)?
                                .into_reference(),
                        )),
                ),
            )?;
            entry.insert(path);
            config::update_config(&config)?;
        } else {
            println!("Worktree, {worktree_name}, already exists!");
        }
    } else {
        println!("Project, {project_name}, does not exist!");
    }
    Ok(())
}

pub fn rm_worktree(
    project_name: String,
    worktree_name: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = config::load_config()?;
    if let Some(project) = config.projects.get_mut(&project_name) {
        if let std::collections::hash_map::Entry::Occupied(entry) =
            project.worktrees.entry(worktree_name.clone())
        {
            let repo = git2::Repository::open(&project.path)?;
            repo.find_worktree(&worktree_name)?.prune(Some(
                git2::WorktreePruneOptions::new()
                    .valid(true)
                    .working_tree(true),
            ))?;
            entry.remove();
            config::update_config(&config)?;
        } else {
            println!("Worktree, {worktree_name}, does not exist!");
        }
    } else {
        println!("Project, {project_name}, does not exist!");
    }
    Ok(())
}
