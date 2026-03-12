mod jj_ops;
mod file_watcher;

use jj_ops::{JjError, BranchInfo, CommitInfo, DiffInfo, FileContentDiff, DiffHunk, load_workspace, get_workspace_root, get_repo_path, get_workspace_name, list_virtual_branches, create_virtual_branch, get_commit_history, uncommit, create_commit, get_working_copy_diff, amend_commit, discard_changes};
use file_watcher::{create_file_watcher_state, start_file_watcher, stop_file_watcher};
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

#[command]
fn list_branches(path: String) -> Result<Vec<BranchInfo>, String> {
    let workspace_path = PathBuf::from(&path);
    let workspace = load_workspace(&workspace_path).map_err(|e: JjError| e.to_string())?;
    Ok(list_virtual_branches(&workspace))
}

#[command]
fn create_branch(path: String, name: String) -> Result<BranchInfo, String> {
    let workspace_path = PathBuf::from(&path);
    let workspace = load_workspace(&workspace_path).map_err(|e: JjError| e.to_string())?;
    create_virtual_branch(&workspace, &name).map_err(|e: JjError| e.to_string())
}

#[command]
fn get_commits(path: String, limit: Option<usize>) -> Result<Vec<CommitInfo>, String> {
    let workspace_path = PathBuf::from(&path);
    let workspace = load_workspace(&workspace_path).map_err(|e: JjError| e.to_string())?;
    let limit = limit.unwrap_or(50);
    get_commit_history(&workspace, limit).map_err(|e: JjError| e.to_string())
}

#[command]
fn uncommit_cmd(path: String) -> Result<CommitInfo, String> {
    let workspace_path = PathBuf::from(&path);
    uncommit(&workspace_path).map_err(|e: JjError| e.to_string())
}

#[command]
fn create_commit_cmd(path: String, message: String) -> Result<CommitInfo, String> {
    let workspace_path = PathBuf::from(&path);
    create_commit(&workspace_path, message).map_err(|e: JjError| e.to_string())
}

#[command]
fn amend_commit_cmd(path: String, message: String) -> Result<CommitInfo, String> {
    let workspace_path = PathBuf::from(&path);
    amend_commit(&workspace_path, message).map_err(|e: JjError| e.to_string())
}

#[command]
fn get_diff(path: String) -> Result<DiffInfo, String> {
    let workspace_path = PathBuf::from(&path);
    get_working_copy_diff(&workspace_path).map_err(|e: JjError| e.to_string())
}

#[command]
fn discard_changes_cmd(path: String, files: Vec<String>) -> Result<(), String> {
    let workspace_path = PathBuf::from(&path);
    discard_changes(&workspace_path, files).map_err(|e: JjError| e.to_string())
}

#[command]
fn get_file_diff(path: String, file_path: String) -> Result<FileContentDiff, String> {
    use jj_ops::get_file_diff;
    get_file_diff(path, file_path).map_err(|e: JjError| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let file_watcher_state = create_file_watcher_state();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(file_watcher_state)
        .invoke_handler(tauri::generate_handler![
            greet,
            open_workspace,
            get_repo_info,
            list_branches,
            create_branch,
            get_commits,
            uncommit_cmd,
            create_commit_cmd,
            amend_commit_cmd,
            get_diff,
            discard_changes_cmd,
            get_file_diff,
            start_file_watcher,
            stop_file_watcher
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
