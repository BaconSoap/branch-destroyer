#[derive(Debug)]
pub struct Context {
    pub token: String,
    pub owner: String,
    pub repo: String,
    pub repo_id: u32,
    pub default_branch: String,
    pub days_ago: u32,
}

#[derive(Debug)]
pub struct Branch {
    pub name: String,
}

#[derive(Debug)]
pub struct BranchInfo {
    pub branch: Branch,
    pub ahead: u32,
    pub behind: u32,
}
