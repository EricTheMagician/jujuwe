use jj_lib::settings::UserSettings;
use jj_lib::workspace::{default_working_copy_factories, Workspace, WorkspaceLoadError};
use jj_lib::repo::StoreFactories;
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
