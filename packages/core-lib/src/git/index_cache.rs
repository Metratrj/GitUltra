use cache_generated::gitultra::git;
use git2::IndexEntry;
use redb::{AccessGuard, Database, Error, ReadableTable, TableDefinition};
use std::path::{Path, PathBuf};

extern crate flatbuffers;
// import generated code
#[allow(dead_code, unused_imports)]
#[path = "./cache_generated.rs"]
mod cache_generated;
pub use cache_generated::gitultra::git::*;

const INDEX_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("git_index");
const CACHE_VERSION: u32 = 1;

pub struct GitIndexCache {
    db: Database,
    repo_path: PathBuf,
}

impl GitIndexCache {
    pub fn open(repo_path: &Path) -> Result<Self, redb::Error> {
        let cache_path = repo_path.join(".git/gitultra_index.redb");
        let db = Database::create(cache_path)?;

        let write_txn = db.begin_write()?;
        {
            let _ = write_txn.open_table(INDEX_TABLE)?;
        }
        write_txn.commit()?;

        Ok(Self {
            db,
            repo_path: repo_path.to_path_buf(),
        })
    }

    pub fn get_index_entries(&self) -> Result<Vec<IndexEntry>, CacheError> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(INDEX_TABLE)?;

        let key = self.cache_key()?;

        todo!()
    }

    pub fn update_index(&self, entries: &[IndexEntry]) -> Result<(), CacheError> {
        todo!()
    }
    fn cache_key(&self) -> Result<String, CacheError> {
        todo!()
    }
    fn serialize_entries(entries: &[IndexEntry]) -> Result<Vec<u8>, CacheError> {
        todo!()
    }
    fn deserialize_entries(data: &[u8]) -> Result<CachedIndex, CacheError> {
        let cache = flatbuffers::root::<git::IndexCache>(data)?;
        let mut entries = Vec::new();
        for fb_entry in cache.entries().unwrap().iter() {
            entries.push(IndexEntry {
                path: fb_entry.path().unwrap().as_bytes().to_vec(),
                id: git2::Oid::from_bytes(fb_entry.oid().unwrap().bytes())?,
                file_size: fb_entry.size() as u32,
                ctime: todo!(),
                mtime: todo!(),
                dev: todo!(),
                ino: todo!(),
                mode: todo!(),
                uid: todo!(),
                gid: todo!(),
                flags: todo!(),
                flags_extended: todo!(),
            })
        }
        todo!()
    }
}

struct CachedIndex {
    version: u32,
    entries: Vec<IndexEntry>,
}

#[derive(Debug, thiserror::Error)]
pub enum CacheError {
    #[error("Cache version mismatch")]
    VersionMismatch,
    #[error("Cache miss")]
    CacheMiss,
    #[error("Storage error: {0}")]
    Storage(#[from] redb::Error),
    #[error("Table error: {0}")]
    Table(#[from] redb::TableError),
    #[error("Transaction error: {0}")]
    Transaction(#[from] redb::TransactionError),
    #[error("Serialization error: {0}")]
    Serialization(#[from] flatbuffers::InvalidFlatbuffer),
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),
}
