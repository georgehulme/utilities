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
        #[arg(long, short)]
        existing: bool,
    },
    RenameWorktree {
        project_name: String,
        old_worktree_name: String,
        new_worktree_name: String,
    },
    RemoveWorktree {
        project_name: String,
        worktree_name: String,
        #[arg(long, short)]
        keep: bool,
    },
    ListProjects,
    PrintProjectPath {
        project_name: String,
    },
    AddProject {
        project_name: String,
        path: String,
        #[arg(long, short)]
        inherit: bool,
    },
    RenameProject {
        old_project_name: String,
        new_project_name: String,
    },
    RemoveProject {
        project_name: String,
        #[arg(long, short)]
        keep: bool,
    },
}
