use std::fs;
use std::path::PathBuf;
use chrono::Local;
use tauri::AppHandle;

fn get_app_data_dir(app: &AppHandle) -> Result<PathBuf, String> {
    app.path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))
}

#[tauri::command]
pub fn get_database_path(app: AppHandle) -> Result<String, String> {
    let data_dir = get_app_data_dir(&app)?;
    let db_path = data_dir.join("data").join("tasker.db");
    Ok(db_path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn backup_database(app: AppHandle, target_path: String) -> Result<String, String> {
    let data_dir = get_app_data_dir(&app)?;
    let db_path = data_dir.join("data").join("tasker.db");

    if !db_path.exists() {
        return Err("Database file not found".to_string());
    }

    fs::copy(&db_path, &target_path)
        .map_err(|e| format!("Failed to backup database: {}", e))?;

    Ok(target_path)
}

#[tauri::command]
pub fn restore_database(app: AppHandle, source_path: String) -> Result<(), String> {
    let data_dir = get_app_data_dir(&app)?;
    let db_path = data_dir.join("data").join("tasker.db");

    // Validate source file exists
    if !PathBuf::from(&source_path).exists() {
        return Err("Source file not found".to_string());
    }

    // Create backup before restore
    if db_path.exists() {
        let backup_name = format!(
            "tasker_pre_restore_{}.db",
            Local::now().format("%Y%m%d_%H%M%S")
        );
        let backup_path = data_dir.join("data").join(backup_name);
        fs::copy(&db_path, &backup_path)
            .map_err(|e| format!("Failed to create pre-restore backup: {}", e))?;
    }

    // Ensure data directory exists
    let data_path = data_dir.join("data");
    if !data_path.exists() {
        fs::create_dir_all(&data_path)
            .map_err(|e| format!("Failed to create data directory: {}", e))?;
    }

    // Restore database
    fs::copy(&source_path, &db_path)
        .map_err(|e| format!("Failed to restore database: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn get_default_backup_filename() -> String {
    format!(
        "tasker_backup_{}.db",
        Local::now().format("%Y%m%d_%H%M%S")
    )
}
