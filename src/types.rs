use chrono::{DateTime, Duration, Utc};

#[derive(Debug)]
pub struct Context {
    pub token: String,
    pub owner: String,
    pub repo: String,
    pub repo_id: u32,
    pub default_branch: String,
    pub days_ago: u32,
    pub for_real: bool,
}

#[derive(Deserialize, Debug)]
pub struct Branch {
    pub name: String,
}

#[derive(Debug)]
pub struct BranchInfo {
    pub branch: Branch,
    pub ahead: u32,
    pub behind: u32,
    pub age: Duration,
}

#[derive(Deserialize, Debug)]
pub struct Repository {
    pub id: u32,
    pub name: String,
    pub default_branch: String,
}

#[derive(Deserialize, Debug)]
pub struct ComparisonResult {
    pub status: String,
    pub ahead_by: u32,
    pub behind_by: u32,
    pub total_commits: u32,
    pub commits: Vec<CommitWrapper>,
    pub merge_base_commit: CommitWrapper,
}

#[derive(Deserialize, Debug)]
pub struct CommitWrapper {
    pub sha: String,
    pub commit: Commit,
}

#[derive(Deserialize, Debug)]
pub struct Commit {
    pub author: Author,
    pub committer: Author,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct Author {
    pub name: String,
    pub email: String,
    pub date: DateTime<Utc>,
}
