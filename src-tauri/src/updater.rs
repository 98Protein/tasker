use tauri::AppHandle;
use tauri_plugin_updater::UpdaterExt;

#[tauri::command]
pub async fn check_for_updates(app: AppHandle) -> Result<Option<String>, String> {
    let updater = app.updater()
        .map_err(|e| format!("Failed to get updater: {}", e))?;

    match updater.check().await {
        Ok(Some(update)) => {
            Ok(Some(update.version.clone()))
        }
        Ok(None) => Ok(None),
        Err(e) => Err(format!("Failed to check for updates: {}", e)),
    }
}

#[tauri::command]
pub async fn download_and_install_update(app: AppHandle) -> Result<(), String> {
    let updater = app.updater()
        .map_err(|e| format!("Failed to get updater: {}", e))?;

    let update = updater.check().await
        .map_err(|e| format!("Failed to check for updates: {}", e))?
        .ok_or_else(|| "No update available".to_string())?;

    update.download_and_install(|_, _| {}, || {}).await
        .map_err(|e| format!("Failed to install update: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn get_current_version(app: AppHandle) -> String {
    app.config().version.clone().unwrap_or_else(|| "0.1.0".to_string())
}
