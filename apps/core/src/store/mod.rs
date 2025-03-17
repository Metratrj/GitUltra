use core_lib::git;
use git2::Repository;
use std::sync::MutexGuard;
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

#[derive(Clone, Debug)]
pub struct RepoHandle {
    path: PathBuf,
    repo: Repository
}

impl RepoHandle {
    pub fn new(path: PathBuf, repo: Repository) -> Self {
        Self {path, repo}
    }

    pub fn path(&self)->&PathBuf {
        &self.path
    }

    pub fn repo(&self) -> &Repository {
        &self.repo
    }

}


#[derive(Clone)]
pub struct RepoStore {
    repos: Arc<Mutex<HashMap<PathBuf, RepoHandle>>>,
}

impl RepoStore {
    pub fn new() -> Self {
        Self {
            repos: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn open_repo(&self, path: PathBuf) -> Result<(), git2::Error> {
        let repo = git::open_repo(&path)?;
        let handle = RepoHandle::new(path.clone(), repo);
        self.repos.lock().unwrap().insert(path, handle);
        Ok(())
    }

    pub fn get_repo(&self, path: &PathBuf) -> Option<RepoHandle> {
        self.repos.lock().unwrap().get(path).cloned()
    }

    pub fn list_repos(&self) -> Vec<PathBuf> {
        self.repos.lock().unwrap().keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_open_and_get_repo() {
        // Create a temporary directory
        let temp_dir = std::env::temp_dir().join("gitultra_test");
        fs::create_dir_all(&temp_dir).unwrap();

        // Initialize a Git repository in the temp directory
        let repo = Repository::init(&temp_dir).unwrap();

        // Create the RepoStore
        let store = RepoStore::new();

        // Open the repository in the store
        store.open_repo(temp_dir.clone()).unwrap();

        // Retrieve the repository from the store
        let retrieved_repo = store.get_repo(&temp_dir).unwrap();

        // Verify the retrieved repository matches the original
        assert_eq!(retrieved_repo.path(), repo.path());

        // Clean up the temporary directory
        fs::remove_dir_all(&temp_dir).unwrap();
    }
}
