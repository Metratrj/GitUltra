use std::path::Path;

use log::info;
use tauri::App;
use tauri::AppHandle;
use tauri::Manager;
use tauri::Runtime;
use tauri_plugin_store::JsonValue;
use tauri_plugin_store::StoreExt;

const GITULTRA_TAURI_STORE: &str = "gitultra-tauri-store";
const GITULTRA_LOADED_REPOS: &str = "gitultra-loaded-repos";

pub fn get_repos<T: tauri::Runtime>(app: &AppHandle<T>) -> Option<Vec<JsonValue>> {
    let store = app
        .get_store(GITULTRA_TAURI_STORE)
        .expect("Store should already be loaded or created");

    if let Some(loaded_repos) = store.get(GITULTRA_LOADED_REPOS) {
        let stored_repos_arr = match loaded_repos {
            JsonValue::Array(arr) => arr,
            unexpected_type => panic!("Invalid value type: {}", unexpected_type),
        };
        Some(stored_repos_arr)
    } else {
        None
    }
}

pub fn remove_repo<T: tauri::Runtime>(app: &AppHandle<T>, path: &Path) {
    let store = app
        .get_store(GITULTRA_TAURI_STORE)
        .expect("Store should already be loaded or created");

    if let Some(mut loaded_repos) = get_repos(app) {
        loaded_repos.retain(|s| !s.as_str().unwrap().contains(path.to_str().unwrap()));
        store.set(GITULTRA_LOADED_REPOS, JsonValue::Array(loaded_repos));
    }
}

pub fn add_repo<T: tauri::Runtime>(app: &AppHandle<T>, path: &Path) -> String {
    let store = app
        .get_store(GITULTRA_TAURI_STORE)
        .expect("Store should already be loaded or created");

    if let Some(loaded_repos) = store.get(GITULTRA_LOADED_REPOS) {
        let mut stored_repos_obj = match loaded_repos {
            JsonValue::Array(arr) => arr,
            unexpected_type => panic!("Invalid value type: {}", unexpected_type),
        };
        stored_repos_obj.push(JsonValue::String(path.to_str().unwrap().to_string()));
        stored_repos_obj.dedup();
        info!("stored_repos_obj: {:?}", stored_repos_obj);
        store.set(GITULTRA_LOADED_REPOS, JsonValue::Array(stored_repos_obj));
        path.to_str().unwrap().to_string()
    } else {
        // create key-value with json array
        store.set(
            GITULTRA_LOADED_REPOS,
            // need to convert this ugly shit
            JsonValue::Array(vec![JsonValue::String(path.to_str().unwrap().to_string())]),
        );
        path.to_str().unwrap().to_string()
    }
}
