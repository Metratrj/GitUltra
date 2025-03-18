use std::path::PathBuf;

use git2::Repository;
use log::{error, info};
use tauri::{AppHandle, Runtime};

#[tauri::command]
#[specta::specta]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
#[specta::specta]
pub fn open_repo_directory<T: Runtime>(app: AppHandle<T>, path: PathBuf) -> Vec<String> {
    println!("Opening repo directory: {:?}", path);

    let repo = match core_lib::git::open_repo(&path) {
        Ok(repo) => repo,
        Err(e) => {
            error!("Failed to open repo: {:?}", e);
            return vec!["".to_string()];
            // return format!("Failed to open repo: {:?}", e);
        }
    };
    info!("Repo opened: {:?}", repo.path());

    let repo_name = core_lib::store::repos::add_repo(&app, &path);
    info!("Repo added to local store: {:?}", repo_name);

    core_lib::store::repos::get_repos(&app)
        .unwrap()
        .iter()
        .map(|r| r.as_str().unwrap().to_string())
        .collect()
}

#[tauri::command]
#[specta::specta]
pub fn get_commit_graph<T: Runtime>(
    app: AppHandle<T>,
    path: String,
) -> Result<Vec<core_lib::git::CommitNode>, String> {
    info!("Getting commit graph for repo: {:?}", path);
    /*     let repo_path = match core_lib::store::repos::get_repo(&app, &name) {
        Some(repo) => repo,
        None => {
            return Err(format!("Repo not found: {:?}", name));
        }
    }; */

    let w = core_lib::store::repos::get_repos(&app).unwrap();
    let s = w
        .iter()
        .map(|r| r.as_str().unwrap().to_string());

    let v: Vec<String> = s.collect();
    println!("Repos: {:?}", v);

    let p = v.iter().find(|&x| x.contains(&path));
    if p.is_none() {
        return Err(format!("Repo not found: {:?}", path));
    }
    


    let repo = match Repository::open(p.unwrap()) {
        Ok(repo) => repo,
        Err(e) => {
            return Err(format!("Failed to open repo: {:?}", e));
        }
    };
    match core_lib::git::get_commit_graph(&repo) {
        Ok(x) => Ok(x),
        Err(e) => Err(e.to_string()),
    }
}


