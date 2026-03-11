mod jj_ops;

use jj_ops::{JjError, load_workspace, get_workspace_root, get_repo_path, get_workspace_name};
use std::path::PathBuf;
use tauri::command;

#[command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[command]
fn open_workspace(path: String) -> Result<String, String> {
    let workspace_path = PathBuf::from(&path);
    let workspace = load_workspace(&workspace_path).map_err(|e: JjError| e.to_string())?;
    Ok(get_workspace_root(&workspace))
}

#[command]
fn get_repo_info(path: String) -> Result<serde_json::Value, String> {
    let workspace_path = PathBuf::from(&path);
    let workspace = load_workspace(&workspace_path).map_err(|e: JjError| e.to_string())?;
    
    Ok(serde_json::json!({
        "workspace_root": get_workspace_root(&workspace),
        "repo_path": get_repo_path(&workspace),
        "workspace_name": get_workspace_name(&workspace),
    }))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            open_workspace,
            get_repo_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
