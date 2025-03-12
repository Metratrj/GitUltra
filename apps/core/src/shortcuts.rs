use tauri::App;
use tauri::AppHandle;
use tauri::Manager;
use tauri::Runtime;
use tauri_plugin_global_shortcut::GlobalShortcutExt;
use tauri_plugin_global_shortcut::Shortcut;
use tauri_plugin_global_shortcut::ShortcutState;
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

        let stored_shortcut = stored_shortcut_str
            .parse::<Shortcut>()
            .expect("Stored shortcut string not valid");

        _register_shortcut_upon_start(app, stored_shortcut);
    } else {
        // use default shortcut if none is stored
        store.set(
            GITULTRA_GLOBAL_SHORTCUTS,
            JsonValue::String(DEFAULT_SHORTCUT.to_string()),
        );
        let default_shortcut = DEFAULT_SHORTCUT
            .parse::<Shortcut>()
            .expect("Default shortcut should be valid");
        _register_shortcut_upon_start(app, default_shortcut);
    }
}

/// Get the current stored shortcut as a string
#[tauri::command]
#[specta::specta]
pub fn get_current_shortcut<R: Runtime>(app: AppHandle<R>) -> Result<String, String> {
    let shortcut = _get_shortcut(&app);
    Ok(shortcut)
}

/// Unregister the current shortcut in Tauri
#[tauri::command]
#[specta::specta]
pub fn unregister_shortcut<T: tauri::Runtime>(app: tauri::AppHandle<T>) {
    let shortcut_str = _get_shortcut(&app);
    let shortcut = shortcut_str
        .parse::<Shortcut>()
        .expect("Stored shortcut string not valid");

    // unregister the shortcut
    app.global_shortcut()
        .unregister(shortcut)
        .expect("Failed to unregister shortcut");
}

/// Change the global shortcut
#[tauri::command]
#[specta::specta]
pub fn change_shortcut<R: Runtime>(
    app: AppHandle<R>,
    _window: tauri::Window<R>,
    key: String,
) -> Result<(), String> {
    println!("Key: {}", key);

    let shortcut = match key.parse::<Shortcut>() {
        Ok(shortcut) => shortcut,
        Err(_) => return Err(format!("Invalid shortcut {}", key)),
    };

    // Store the new shortcut
    let store = app
        .get_store(GITULTRA_TAURI_STORE)
        .expect("Store should already be loaded or created");

    store.set(GITULTRA_GLOBAL_SHORTCUTS, JsonValue::String(key));

    _register_shortcut(&app, shortcut);

    Ok(())
}

// Helper function to register shortcut, maily for updating shortcuts
fn _register_shortcut<R: Runtime>(app: &AppHandle<R>, shortcut: Shortcut) {
    let main_window = app.get_webview_window("main").unwrap();

    // Register shortcut and define its behavior
    app.global_shortcut()
        .on_shortcut(shortcut, move |_app, scut, event| {
            if scut == &shortcut {
                if let ShortcutState::Pressed = event.state() {
                    // Toggle window visibility
                    if main_window.is_visible().unwrap() {
                        main_window.hide().unwrap(); // Hide window
                    } else {
                        main_window.show().unwrap(); // Show window
                        main_window.set_focus().unwrap(); // Focus window
                    }
                }
            }
        })
        .map_err(|err| format!("Failed to register new shortcut '{}'", err))
        .unwrap();
}

// Helper function to register shortcuts during application startup
fn _register_shortcut_upon_start(app: &App, shortcut: Shortcut) {
    let window = app.get_webview_window("main").unwrap();

    // Initialuze global shortcut and set its handler
    app.handle()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |_app, scut, event| {
                    if scut == &shortcut {
                        if let ShortcutState::Pressed = event.state() {
                            // Toggle window visibility
                            if window.is_visible().unwrap() {
                                window.hide().unwrap(); // hide window
                            } else {
                                window.show().unwrap(); // show window
                                window.set_focus().unwrap(); // focus window
                            }
                        }
                    }
                })
                .build(),
        )
        .unwrap();
    app.global_shortcut().register(shortcut).unwrap();
}

// Recieve the stored global shortcut as a string
pub fn _get_shortcut<R: Runtime>(app: &AppHandle<R>) -> String {
    let store = app
        .get_store(GITULTRA_TAURI_STORE)
        .expect("Store should already be loaded or created");

    match store
        .get(GITULTRA_GLOBAL_SHORTCUTS)
        .expect("Shortcut should already be stored")
    {
        JsonValue::String(str) => str,
        unexpected_type => panic!("Invalid shortcut value type: {}", unexpected_type),
    }
}
