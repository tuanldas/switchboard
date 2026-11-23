use anyhow::Context;
use log::info;
use tauri_plugin_autostart::MacosLauncher as MacOSLauncher;
#[allow(unused_imports)]
use tauri_plugin_autostart::MacosLauncher;

pub mod tray;
pub mod commands;
pub mod providers;
pub mod tracking;
pub mod crypto;
pub mod keychain;
pub mod db;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacOSLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            commands::list_accounts,
            commands::add_account,
            commands::remove_account,
            commands::switch_account,
            commands::get_active_account,
            commands::get_settings,
            commands::update_settings,
            commands::get_usage_summary,
            commands::get_usage_history,
        ])
        .setup(|app| {
            info!("Initializing database...");
            db::init().context("Failed to initialize database")?;

            info!("Setting up system tray...");
            tray::setup(app.handle()).context("Failed to setup tray")?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
