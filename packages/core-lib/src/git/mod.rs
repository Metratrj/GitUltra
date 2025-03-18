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

pub fn get_commits(repo: &Repository) -> Result<Vec<git2::Commit>, git2::Error> {
    let mut revwalk = repo.revwalk()?;
    revwalk.set_sorting(git2::Sort::TIME | git2::Sort::TOPOLOGICAL)?;
    revwalk.push_head()?;
    
    revwalk
        .map(|id| repo.find_commit(id?))
        .collect()
}


pub fn get_commit_graph(repo: &Repository) -> Result<Vec<CommitNode>, git2::Error> {
    let mut walk = repo.revwalk()?;
    walk.set_sorting(git2::Sort::TIME | git2::Sort::TOPOLOGICAL)?;
    walk.push_head()?;

    walk.map(|oid| {
        let oid = oid?;
        let commit = repo.find_commit(oid)?.to_owned();
        let x = CommitNode {
            oid: oid.to_string(),
            author: commit.author().to_string(),
            message: commit.message().unwrap_or("").to_string(),
            parents: commit.parent_ids().map(|p| p.to_string()).collect(),
            timestamp: commit.time().seconds(),
        }; 
        Ok(x)
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
