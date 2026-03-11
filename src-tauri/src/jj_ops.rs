use jj_lib::backend::CommitId;
use jj_lib::object_id::ObjectId;
use jj_lib::ref_name::RefName;
use jj_lib::settings::UserSettings;
use jj_lib::workspace::{default_working_copy_factories, Workspace, WorkspaceLoadError};
use jj_lib::repo::StoreFactories;
use serde::Serialize;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JjError {
    #[error("Workspace load error: {0}")]
    WorkspaceLoad(#[from] WorkspaceLoadError),
    #[error("Path error: {0}")]
    Path(String),
    #[error("Other error: {0}")]
    Other(String),
}

pub type JjResult<T> = Result<T, JjError>;

pub fn load_workspace(workspace_path: &Path) -> JjResult<Workspace> {
    let workspace_path = workspace_path
        .canonicalize()
        .map_err(|e| JjError::Path(e.to_string()))?;
    
    let config = jj_lib::config::StackedConfig::with_defaults();
    
    let user_settings = UserSettings::from_config(config)
        .map_err(|e| JjError::Other(format!("Settings error: {}", e)))?;
    
    let store_factories = StoreFactories::default();
    let working_copy_factories = default_working_copy_factories();
    
    let workspace = Workspace::load(
        &user_settings,
        &workspace_path,
        &store_factories,
        &working_copy_factories,
    ).map_err(JjError::WorkspaceLoad)?;
    
    Ok(workspace)
}

pub fn get_workspace_root(workspace: &Workspace) -> String {
    workspace.workspace_root().to_string_lossy().to_string()
}

pub fn get_repo_path(workspace: &Workspace) -> String {
    workspace.repo_path().to_string_lossy().to_string()
}

pub fn get_workspace_name(workspace: &Workspace) -> String {
    workspace.workspace_name().as_str().to_string()
}

#[derive(Debug, Clone, Serialize)]
pub struct BranchInfo {
    pub name: String,
    pub commit_id: Option<String>,
}

pub fn list_virtual_branches(workspace: &Workspace) -> Vec<BranchInfo> {
    // Use pollster to run async code synchronously
    let repo = pollster::block_on(async {
        let repo_loader = workspace.repo_loader();
        repo_loader.load_at_head().await.unwrap()
    });
    
    let view = repo.view();
    
    let mut branches: Vec<BranchInfo> = Vec::new();
    // In jj, virtual branches are called "bookmarks"
    for (bookmark_name, bookmark_target) in view.local_bookmarks() {
        let head_commit_id = bookmark_target.added_ids().next()
            .map(|id: &CommitId| id.hex());
        
        branches.push(BranchInfo {
            name: bookmark_name.as_str().to_string(),
            commit_id: head_commit_id,
        });
    }
    
    branches.sort_by(|a, b| a.name.cmp(&b.name));
    branches
}

/// Create a new virtual branch (bookmark) at the current working copy commit
/// Note: This requires additional jj-lib API work to complete
pub fn create_virtual_branch(_workspace: &Workspace, _branch_name: &str) -> JjResult<BranchInfo> {
    Err(JjError::Other("Create branch not fully implemented - needs jj-lib MutableRepo API work".to_string()))
}
