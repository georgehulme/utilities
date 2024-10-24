#[derive(Debug)]
pub struct ProjectExistsError(pub String);

impl std::fmt::Display for ProjectExistsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The project, \"{}\", already exists!", self.0)
    }
}

impl std::error::Error for ProjectExistsError {}

#[derive(Debug)]
pub struct ProjectNotExistsError(pub String);

impl std::fmt::Display for ProjectNotExistsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The project, \"{}\", does not exist!", self.0)
    }
}

impl std::error::Error for ProjectNotExistsError {}

#[derive(Debug)]
pub struct WorktreeExistsError(pub String);

impl std::fmt::Display for WorktreeExistsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The worktree, \"{}\", already exists!", self.0)
    }
}

impl std::error::Error for WorktreeExistsError {}

#[derive(Debug)]
pub struct WorktreeNotExistsError(pub String);

impl std::fmt::Display for WorktreeNotExistsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The worktree, \"{}\", does not exist!", self.0)
    }
}

impl std::error::Error for WorktreeNotExistsError {}
