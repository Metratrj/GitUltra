use tauri::App;
use tauri_plugin_store::StoreExt;

const GITULTRA_TAURI_STORE: &str = "gitultra-tauri-store";

pub fn enable_store(app: &App) {
    let store = app
        .store(GITULTRA_TAURI_STORE)
        .expect("Creating the store failed");

    
}
