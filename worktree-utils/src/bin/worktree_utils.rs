use clap::{CommandFactory, Parser};
use clap_complete::{generate_to, Shell};
use worktree_utils::{
    cli::{Cli, CliSubCommand},
    config, project, worktree,
};

fn main() {
    let args = worktree_utils::cli::Cli::parse();
    let mut config = config::load_config_from_file();
    match args.command {
        CliSubCommand::GenerateShellCompletions { out_dir } => {
            let cmd = &mut Cli::command();
            let shell = Shell::from_env().unwrap();
            generate_to(
                shell,
                cmd,
                cmd.get_name().to_string(),
                &out_dir,
            ).unwrap();
            println!("Please source the scripts in \"{out_dir}\" to enable code completion.")
        }
        // Project level
        CliSubCommand::ListProjects => project::list_projects(),
        CliSubCommand::PrintProjectPath { project_name } => {
            project::print_project_path(&mut config, project_name).unwrap();
        }
        CliSubCommand::AddProject { project_name, path } => {
            project::add_project(&mut config, project_name, path);
            config::write_config_to_file(&config);
        }
        CliSubCommand::RemoveProject { project_name, keep } => {
            project::rm_project(&mut config, project_name, keep);
            config::write_config_to_file(&config);
        }
        // Worktree level
        CliSubCommand::ListWorktrees { project_name } => {
            let project = config.get_occupied_project_entry(project_name).unwrap();
            worktree::list_worktrees(project.get());
        }
        CliSubCommand::PrintWorktreePath {
            project_name,
            worktree_name,
        } => {
            let project = config.get_occupied_project_entry(project_name).unwrap();
            worktree::print_worktree_path(project.get(), worktree_name).unwrap();
        }
        CliSubCommand::AddWorktree {
            project_name,
            worktree_name,
            branch,
            path,
        } => {
            let mut project = config.get_occupied_project_entry(project_name).unwrap();
            worktree::add_worktree(project.get_mut(), worktree_name, branch, path);
            config::write_config_to_file(&config);
        }
        CliSubCommand::RemoveWorktree {
            project_name,
            worktree_name,
        } => {
            let mut project = config.get_occupied_project_entry(project_name).unwrap();
            worktree::rm_worktree(project.get_mut(), worktree_name);
            config::write_config_to_file(&config);
        }
    }
}
