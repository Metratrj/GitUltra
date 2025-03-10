use git2::{BranchType, Repository};
use std::path::PathBuf;

pub fn open_repo(path: PathBuf) -> Result<Repository, git2::Error> {
    Repository::open(path)
}

pub fn get_commits(repo: &Repository) -> Vec<git2::Commit> {
    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push_head().unwrap();
    revwalk
        .filter_map(|id| repo.find_commit(id.unwrap()).ok())
        .collect()
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
