use std::path::PathBuf;
use git2::Repository;

pub fn open_repo(path: PathBuf) -> Result<Repository, git2::Error> {
    Repository::open(path)
}

pub fn create_repo(path: PathBuf) -> Result<Repository, git2::Error> {
    Repository::init(path)
}