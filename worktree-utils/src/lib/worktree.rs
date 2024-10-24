use std::str::FromStr;

use crate::{config::ProjectConfig, error::WorktreeNotExistsError};

pub fn list_worktrees(project: &ProjectConfig) {
    let mut table = prettytable::Table::new();
    table.add_row(prettytable::row!["Worktree", "Path"]);
    table.add_row(prettytable::row!["<root>", &project.path]);
    for worktree in &project.worktrees {
        table.add_row(prettytable::row![worktree.0, worktree.1]);
    }
    table.printstd();
}

pub fn print_worktree_path(
    project: &ProjectConfig,
    worktree_name: String,
) -> Result<(), WorktreeNotExistsError> {
    println!(
        "{}",
        match project.worktrees.get(&worktree_name) {
            Some(path) => Ok(path),
            None => Err(WorktreeNotExistsError(worktree_name)),
        }?
    );
    Ok(())
}

pub fn add_worktree(
    project: &mut ProjectConfig,
    worktree_name: String,
    branch: String,
    path: String,
) {
    let repo = git2::Repository::open(&project.path).unwrap();
    repo.worktree(
        &worktree_name,
        std::path::PathBuf::from_str(&path).unwrap().as_path(),
        Some(
            git2::WorktreeAddOptions::new()
                .checkout_existing(true)
                .reference(Some(
                    &repo
                        .find_branch(&branch, git2::BranchType::Local)
                        .unwrap()
                        .into_reference(),
                )),
        ),
    )
    .unwrap();
    project
        .get_vacant_worktree_entry(worktree_name)
        .unwrap()
        .insert(path);
}

pub fn rm_worktree(project: &mut ProjectConfig, worktree_name: String) {
    git2::Repository::open(&project.path)
        .unwrap()
        .find_worktree(&worktree_name)
        .unwrap()
        .prune(Some(
            git2::WorktreePruneOptions::new()
                .valid(true)
                .working_tree(true),
        ))
        .unwrap();
    project
        .get_occupied_worktree_entry(worktree_name)
        .unwrap()
        .remove();
}
