#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

// use specta_typescript::Typescript;
use std::path::PathBuf;
use tauri_specta::{collect_commands, Builder};

mod shortcuts;
mod store;

#[tauri::command]
#[specta::specta]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
#[specta::specta]
fn open_repo_directory(path: PathBuf) -> String {
    println!("Opening repo directory: {:?}", path);

    let repo = match core_lib::git::open_repo(path) {
        Ok(repo) => repo,
        Err(e) => {
            return format!("Failed to open repo: {:?}", e);
        }
    };

    println!("Repo opened: {:?}", repo.path());

    let branches = core_lib::git::get_branches(&repo);
    for branch in branches {
        println!("branch: {:?}", branch.name().unwrap());
    }

    let commits = core_lib::git::get_commits(&repo);
    for commit in commits {
        println!("commit: {:?}", commit.message().unwrap());
    }

    "Ok".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = Builder::<tauri::Wry>::new().commands(collect_commands![
        greet,
        open_repo_directory,
        shortcuts::unregister_shortcut::<tauri::Wry>,
        shortcuts::change_shortcut::<tauri::Wry>,
        shortcuts::get_current_shortcut::<tauri::Wry>,
    ]);

    /* #[cfg(debug_assertions)]
    builder
        .export(
            Typescript::default()
                .formatter(specta_typescript::formatter::prettier)
                .header("/* eslint-disable */
    \n // @ts-nocheck"),
            "../../packages/schemas/ts/gitultra/bindings.ts",
        )
        .expect("Failed to export typescript bindings");
     */

    tauri::Builder::default()
        //.plugin(tauri_plugin_cli::init())
        //.plugin(tauri_plugin_opener::init())
        //.plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(builder.invoke_handler())
        //.invoke_handler(tauri::generate_handler![greet])
        .setup(move |app| {
            // this is needed to use specta events
            //builder.mount_events(app);
            if cfg!(debug_assertions) {
                app.handle()
                    .plugin(tauri_plugin_log::Builder::default().level(log::LevelFilter::Info).build())?;
            }
            /*
                       #[cfg(desktop)]
                       {
                           use tauri_plugin_global_shortcut::{
                               Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
                           };

                           let ctrl_n_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyN);
                           app.handle().plugin(
                               tauri_plugin_global_shortcut::Builder::new()
                                   .with_handler(move |_app, shortcut, event| {
                                       println!("{:?}", shortcut);
                                       if shortcut == &ctrl_n_shortcut {
                                           match event.state() {
                                               ShortcutState::Pressed => {
                                                   println!("Ctrl-N Pressed!");
                                               }
                                               ShortcutState::Released => {
                                                   println!("Ctrl-N Released!");
                                               }
                                           }
                                       }
                                   })
                                   .build(),
                           )?;

                           app.global_shortcut().register(ctrl_n_shortcut)?;
                       }
            */
            shortcuts::enable_shortcut(app);

            Ok(())
        })
        //.menu(tauri::menu::Menu::default)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
