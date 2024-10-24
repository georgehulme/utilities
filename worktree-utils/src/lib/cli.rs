use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum CliSubCommand {
    GenerateShellCompletions {
        out_dir: String,
    },
    ListWorktrees {
        project_name: String,
    },
    PrintWorktreePath {
        project_name: String,
        worktree_name: String,
    },
    AddWorktree {
        project_name: String,
        worktree_name: String,
        branch: String,
        path: String,
    },
    RemoveWorktree {
        project_name: String,
        worktree_name: String,
    },
    ListProjects,
    PrintProjectPath {
        project_name: String,
    },
    AddProject {
        project_name: String,
        path: String,
    },
    RemoveProject {
        project_name: String,
        #[arg(long, short)]
        keep: bool,
    },
}
