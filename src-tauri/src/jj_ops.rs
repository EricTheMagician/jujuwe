use jj_lib::backend::CommitId;
use jj_lib::object_id::ObjectId;
use jj_lib::settings::UserSettings;
use jj_lib::workspace::{default_working_copy_factories, Workspace, WorkspaceLoadError};
use jj_lib::repo::{StoreFactories, Repo};
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

#[derive(Debug, Clone, Serialize)]
pub struct CommitInfo {
    pub change_id: String,
    pub commit_id: String,
    pub description: String,
    pub author: String,
    pub timestamp: i64,
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

/// Get commit history from the repository
pub fn get_commit_history(workspace: &Workspace, limit: usize) -> JjResult<Vec<CommitInfo>> {
    let repo = pollster::block_on(async {
        let repo_loader = workspace.repo_loader();
        repo_loader.load_at_head().await.unwrap()
    });
    
    let view = repo.view();
    let store = repo.store();
    
    // Get the visible heads
    let heads = view.heads();
    
    // Build commit info from heads and their ancestors
    let mut commits: Vec<CommitInfo> = Vec::new();
    let mut visited: std::collections::HashSet<String> = std::collections::HashSet::new();
    
    // Simple BFS to get ancestors of heads
    let mut queue: Vec<CommitId> = heads.iter().cloned().collect();
    
    while let Some(commit_id) = queue.pop() {
        let hex_id = commit_id.hex();
        if visited.contains(&hex_id) {
            continue;
        }
        visited.insert(hex_id.clone());
        
        // Get commit info
        if let Ok(commit) = store.get_commit(&commit_id) {
            let description = commit.description().to_string();
            let author = commit.author().name.clone();
            let timestamp = commit.committer().timestamp.timestamp.0;
            
            commits.push(CommitInfo {
                change_id: commit.change_id().hex(),
                commit_id: hex_id,
                description: if description.is_empty() { "(empty)".to_string() } else { description },
                author,
                timestamp,
            });
            
            // Add parents to queue
            for parent_id in commit.parent_ids() {
                queue.push(parent_id.clone());
            }
        }
        
        if commits.len() >= limit {
            break;
        }
    }
    
    // Sort by timestamp descending (newest first)
    commits.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    commits.truncate(limit);
    
    Ok(commits)
}

/// Create a new virtual branch (bookmark) at the current working copy commit
/// Note: This requires additional jj-lib API work to complete
pub fn create_virtual_branch(_workspace: &Workspace, _branch_name: &str) -> JjResult<BranchInfo> {
    Err(JjError::Other("Create branch not fully implemented - needs jj-lib MutableRepo API work".to_string()))
}
