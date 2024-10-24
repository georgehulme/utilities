use clap::Parser;

use worktree_utils::{command, config, project, worktree};


fn main() {
    let args = command::Cli::parse();
    let mut config = config::load_config_from_file();
    match args.command {
        // Project level
        command::Command::ListProjects => project::list_projects(),
        command::Command::PrintProjectPath { project_name } => {
            project::print_project_path(&mut config, project_name).unwrap();
        }
        command::Command::AddProject { project_name, path } => {
            project::add_project(&mut config, project_name, path);
            config::write_config_to_file(&config);
        }
        command::Command::RemoveProject { project_name, keep } => {
            project::rm_project(&mut config, project_name, keep);
            config::write_config_to_file(&config);
        }
        // Worktree level
        command::Command::ListWorktrees { project_name } => {
            let project = config.get_occupied_project_entry(project_name).unwrap();
            worktree::list_worktrees(project.get());
        }
        command::Command::PrintWorktreePath {
            project_name,
            worktree_name,
        } => {
            let project = config.get_occupied_project_entry(project_name).unwrap();
            worktree::print_worktree_path(project.get(), worktree_name).unwrap();
        }
        command::Command::AddWorktree {
            project_name,
            worktree_name,
            branch,
            path,
        } => {
            let mut project = config.get_occupied_project_entry(project_name).unwrap();
            worktree::add_worktree(project.get_mut(), worktree_name, branch, path);
            config::write_config_to_file(&config);
        }
        command::Command::RemoveWorktree {
            project_name,
            worktree_name,
        } => {
            let mut project = config.get_occupied_project_entry(project_name).unwrap();
            worktree::rm_worktree(project.get_mut(), worktree_name);
            config::write_config_to_file(&config);
        }
    };
}
