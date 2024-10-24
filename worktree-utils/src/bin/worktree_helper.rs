use clap::Parser;

use worktree_utils::{command, project, worktree};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = worktree_utils::command::Cli::parse();
    match args.command {
        // Project level
        command::Command::ListProjects => project::list_projects()?,
        command::Command::PrintProjectPath { project_name } => project::print_project_path(project_name)?,
        command::Command::AddProject { project_name, path } => {
            project::add_project(project_name, path)?
        }
        command::Command::RemoveProject { project_name, keep } => {
            project::rm_project(project_name, keep)?
        }
        // Worktree level
        command::Command::ListWorktrees { project_name } => worktree::list_worktrees(project_name)?,
        command::Command::PrintWorktreePath {
            project_name,
            worktree_name,
        } => worktree::print_worktree_path(project_name, worktree_name)?,
        command::Command::AddWorktree {
            project_name,
            worktree_name,
            branch,
            path,
        } => worktree::add_worktree(project_name, worktree_name, branch, path)?,
        command::Command::RemoveWorktree {
            project_name,
            worktree_name,
        } => worktree::rm_worktree(project_name, worktree_name)?,
    }
    Ok(())
}
