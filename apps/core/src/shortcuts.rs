use tauri::App;
use tauri_plugin_store::JsonValue;
use tauri_plugin_store::StoreExt;

/// Name of the tauri store
const GITULTRA_TAURI_STORE: &str = "gitultra-tauri-store";

/// Key for storing global shortcuts
const GITULTRA_GLOBAL_SHORTCUTS: &str = "global-shortcuts";

/// Default shortcut for Windows and Linux
#[cfg(any(target_os = "windows", target_os = "linux"))]
const DEFAULT_SHORTCUT: &str = "ctrl+,";

// Set shortcut during application startup
pub fn enable_shortcut(app: &App) {
    let store = app
        .store(GITULTRA_TAURI_STORE)
        .expect("Creating the store failed");

    // Use stored shortcut or default shortcut
    if let Some(stored_shortcut) = store.get(GITULTRA_GLOBAL_SHORTCUTS) {
        let stored_shortcut_str = match stored_shortcut {
            JsonValue::String(str) => str,
            unexpected_type => panic!("Invalid shortcut value type: {}", unexpected_type),
        };

        //let stored_shortcut = stored_shortcut_str.parse::<Shortcut>
    } else {

    }
}
