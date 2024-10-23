use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    ListWorktrees {
        project_name: String,
    },
    CDWorktree {
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
    CDProject {
        project_name: String,
    },
    AddProject {
        project_name: String,
        path: String,
    },
    RemoveProject {
        project_name: String,
        keep: bool,
    },
}
