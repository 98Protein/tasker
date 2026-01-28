mod tray;
mod network;
mod database;
mod settings;
mod updater;

use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            tray::create_tray(app.handle())?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == "main" {
                    let app = window.app_handle();
                    if let Ok(minimize) = settings::get_minimize_to_tray(app.clone()) {
                        if minimize {
                            api.prevent_close();
                            let _ = window.hide();
                        }
                    }
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            network::get_network_info,
            database::get_database_path,
            database::backup_database,
            database::restore_database,
            database::get_default_backup_filename,
            settings::load_settings,
            settings::save_settings,
            settings::get_minimize_to_tray,
            updater::check_for_updates,
            updater::download_and_install_update,
            updater::get_current_version,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
