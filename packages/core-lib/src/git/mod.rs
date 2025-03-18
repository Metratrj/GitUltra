use git2::Repository;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use specta::Type;

mod index_cache;

#[derive(Deserialize, Serialize, Debug, Type)]
pub struct CommitNode {
    pub oid: String,
    pub author: String,
    pub message: String,
    pub parents: Vec<String>,
    pub timestamp: i64,
}

pub fn open_repo(path: &PathBuf) -> Result<Repository, git2::Error> {
    Repository::open(path)
}

pub fn get_commits(repo: &Repository) -> Vec<git2::Commit> {
    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push_head().unwrap();
    revwalk.filter_map(|id| repo.find_commit(id.unwrap()).ok()).collect()
}


pub fn get_commit_graph(repo: &Repository) -> Vec<CommitNode> {
    let mut walk = repo.revwalk().unwrap();;
    walk.push_head().unwrap();

    walk.map(|oid| {
        let oid = oid.unwrap();
        let commit = repo.find_commit(oid).unwrap().to_owned();
        let x = CommitNode {
            oid: oid.to_string(),
            author: commit.author().to_string(),
            message: commit.message().unwrap_or("").to_string(),
            parents: commit.parent_ids().map(|p| p.to_string()).collect(),
            timestamp: commit.time().seconds(),
        };
        x
    }).collect()
}

pub fn get_branches(repo: &Repository) -> Vec<git2::Branch> {
    let mut branches = Vec::new();
    for branch in repo.branches(None).unwrap() {
        let (branch, _) = branch.unwrap();
        branches.push(branch);
    }
    branches
}

pub fn create_repo(path: PathBuf) -> Result<Repository, git2::Error> {
    Repository::init(path)
}
