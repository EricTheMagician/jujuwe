use jj_lib::backend::CommitId;
use jj_lib::object_id::ObjectId;
use jj_lib::rewrite;
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

#[derive(Debug, Clone, Serialize)]
pub struct FileDiff {
    pub path: String,
    pub old_id: Option<String>,
    pub new_id: Option<String>,
    pub diff_type: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiffInfo {
    pub files: Vec<FileDiff>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileContentDiff {
    pub path: String,
    pub old_content: Option<String>,
    pub new_content: Option<String>,
    pub hunks: Vec<DiffHunk>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiffHunk {
    pub old_start: usize,
    pub old_lines: usize,
    pub new_start: usize,
    pub new_lines: usize,
    pub content: String,
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

/// Uncommit - move changes from the current working copy commit back to the working copy
/// This effectively moves the working copy to the parent commit, leaving the current 
/// changes as uncommitted changes
pub fn uncommit(workspace_path: &Path) -> JjResult<CommitInfo> {
    let workspace_path = workspace_path
        .canonicalize()
        .map_err(|e| JjError::Path(e.to_string()))?;
    
    let config = jj_lib::config::StackedConfig::with_defaults();
    let user_settings = UserSettings::from_config(config)
        .map_err(|e| JjError::Other(format!("Settings error: {}", e)))?;
    
    let store_factories = StoreFactories::default();
    let working_copy_factories = default_working_copy_factories();
    
    let mut workspace = Workspace::load(
        &user_settings,
        &workspace_path,
        &store_factories,
        &working_copy_factories,
    ).map_err(JjError::WorkspaceLoad)?;
    
    let repo = pollster::block_on(async {
        let repo_loader = workspace.repo_loader();
        repo_loader.load_at_head().await.unwrap()
    });
    
    let store = repo.store();
    let view = repo.view();
    
    let workspace_name = workspace.workspace_name();
    let wc_commit_id = view.get_wc_commit_id(workspace_name)
        .ok_or_else(|| JjError::Other("No working copy commit found".to_string()))?;
    
    let wc_commit = store.get_commit(wc_commit_id).map_err(|e| JjError::Other(e.to_string()))?;
    
    let parent_ids: Vec<_> = wc_commit.parent_ids().into_iter().cloned().collect();
    
    if parent_ids.is_empty() {
        return Err(JjError::Other("Cannot uncommit the root commit".to_string()));
    }
    
    let parent_id = &parent_ids[0];
    let parent_commit = store.get_commit(parent_id).map_err(|e| JjError::Other(e.to_string()))?;
    
    let old_tree = wc_commit.tree();
    
    pollster::block_on(async {
        workspace.check_out(
            workspace.working_copy().operation_id().clone(),
            Some(&old_tree),
            &parent_commit,
        ).await
    }).map_err(|e| JjError::Other(e.to_string()))?;
    
    Ok(CommitInfo {
        change_id: parent_commit.change_id().hex(),
        commit_id: parent_commit.id().hex(),
        description: parent_commit.description().to_string(),
        author: parent_commit.author().name.clone(),
        timestamp: parent_commit.committer().timestamp.timestamp.0,
    })
}

/// Create a new commit on the active branch with a given commit message
pub fn create_commit(workspace_path: &Path, message: String) -> JjResult<CommitInfo> {
    let workspace_path = workspace_path
        .canonicalize()
        .map_err(|e| JjError::Path(e.to_string()))?;
    
    let config = jj_lib::config::StackedConfig::with_defaults();
    let user_settings = UserSettings::from_config(config)
        .map_err(|e| JjError::Other(format!("Settings error: {}", e)))?;
    
    let store_factories = StoreFactories::default();
    let working_copy_factories = default_working_copy_factories();
    
    let mut workspace = Workspace::load(
        &user_settings,
        &workspace_path,
        &store_factories,
        &working_copy_factories,
    ).map_err(JjError::WorkspaceLoad)?;
    
    let workspace_name = workspace.workspace_name();
    let wc_commit_id: CommitId = {
        let repo = pollster::block_on(async {
            let repo_loader = workspace.repo_loader();
            repo_loader.load_at_head().await.unwrap()
        });
        repo.view().get_wc_commit_id(workspace_name)
            .ok_or_else(|| JjError::Other("No working copy commit found".to_string()))?
            .clone()
    };
    
    let wc_commit = {
        let repo = pollster::block_on(async {
            let repo_loader = workspace.repo_loader();
            repo_loader.load_at_head().await.unwrap()
        });
        repo.store().get_commit(&wc_commit_id).map_err(|e| JjError::Other(e.to_string()))?
    };
    
    let tree = wc_commit.tree();
    let parent_ids = vec![wc_commit.id().clone()];
    
    let new_commit = pollster::block_on(async {
        let repo_loader = workspace.repo_loader();
        let repo = repo_loader.load_at_head().await.unwrap();
        
        let mut tx = repo.start_transaction();
        
        let tx_repo = tx.repo_mut();
        
        let mut commit_builder = tx_repo.new_commit(
            parent_ids,
            tree,
        ).detach();
        
        commit_builder.set_description(&message);
        
        let new_commit = commit_builder.write(tx_repo).await
            .map_err(|e| JjError::Other(e.to_string()))?;
        
        tx.commit(&message)
            .await
            .map_err(|e| JjError::Other(e.to_string()))?;
        
        Ok::<_, JjError>(new_commit)
    })?;
    
    pollster::block_on(async {
        workspace.check_out(
            workspace.working_copy().operation_id().clone(),
            None,
            &new_commit,
        ).await
    }).map_err(|e| JjError::Other(e.to_string()))?;
    
    Ok(CommitInfo {
        change_id: new_commit.change_id().hex(),
        commit_id: new_commit.id().hex(),
        description: new_commit.description().to_string(),
        author: new_commit.author().name.clone(),
        timestamp: new_commit.committer().timestamp.timestamp.0,
    })
}

/// Amend the commit message of the current working copy commit
pub fn amend_commit(workspace_path: &Path, message: String) -> JjResult<CommitInfo> {
    let workspace_path = workspace_path
        .canonicalize()
        .map_err(|e| JjError::Path(e.to_string()))?;
    
    let config = jj_lib::config::StackedConfig::with_defaults();
    let user_settings = UserSettings::from_config(config)
        .map_err(|e| JjError::Other(format!("Settings error: {}", e)))?;
    
    let store_factories = StoreFactories::default();
    let working_copy_factories = default_working_copy_factories();
    
    let mut workspace = Workspace::load(
        &user_settings,
        &workspace_path,
        &store_factories,
        &working_copy_factories,
    ).map_err(JjError::WorkspaceLoad)?;
    
    let workspace_name = workspace.workspace_name();
    
    let (new_commit, commit_message) = pollster::block_on(async {
        let repo_loader = workspace.repo_loader();
        let repo = repo_loader.load_at_head().await.unwrap();
        
        let view = repo.view();
        let wc_commit_id = view.get_wc_commit_id(workspace_name)
            .ok_or_else(|| JjError::Other("No working copy commit found".to_string()))?;
        
        let store = repo.store();
        let wc_commit = store.get_commit(wc_commit_id)
            .map_err(|e| JjError::Other(e.to_string()))?;
        
        let parent_ids: Vec<_> = wc_commit.parent_ids().into_iter().cloned().collect();
        
        if parent_ids.is_empty() {
            return Err(JjError::Other("Cannot amend the root commit".to_string()));
        }
        
        let mut tx = repo.start_transaction();
        
        let tx_repo = tx.repo_mut();
        
        let tree = wc_commit.tree();
        let author = wc_commit.author().clone();
        let committer = wc_commit.committer().clone();
        
        let mut commit_builder = tx_repo.new_commit(
            parent_ids.clone(),
            tree,
        ).detach();
        
        commit_builder.set_description(&message);
        commit_builder.set_author(author);
        commit_builder.set_committer(committer);
        
        let new_commit = commit_builder.write(tx_repo).await
            .map_err(|e| JjError::Other(e.to_string()))?;
        
        tx.commit(&format!("Amend commit: {}", message))
            .await
            .map_err(|e| JjError::Other(e.to_string()))?;
        
        Ok::<_, JjError>((new_commit, message))
    })?;
    
    pollster::block_on(async {
        workspace.check_out(
            workspace.working_copy().operation_id().clone(),
            None,
            &new_commit,
        ).await
    }).map_err(|e| JjError::Other(e.to_string()))?;
    
    Ok(CommitInfo {
        change_id: new_commit.change_id().hex(),
        commit_id: new_commit.id().hex(),
        description: commit_message,
        author: new_commit.author().name.clone(),
        timestamp: new_commit.committer().timestamp.timestamp.0,
    })
}

/// Get the diff between the working copy and its parent commit
pub fn get_working_copy_diff(workspace_path: &Path) -> JjResult<DiffInfo> {
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
    
    let repo = pollster::block_on(async {
        let repo_loader = workspace.repo_loader();
        repo_loader.load_at_head().await.unwrap()
    });
    
    let store = repo.store();
    let view = repo.view();
    
    let workspace_name = workspace.workspace_name();
    let wc_commit_id = view.get_wc_commit_id(workspace_name)
        .ok_or_else(|| JjError::Other("No working copy commit found".to_string()))?;
    
    let wc_commit = store.get_commit(wc_commit_id)
        .map_err(|e| JjError::Other(e.to_string()))?;
    
    let wc_tree = wc_commit.tree();
    
    let parent_ids: Vec<_> = wc_commit.parent_ids().into_iter().cloned().collect();
    
    let mut files: Vec<FileDiff> = Vec::new();
    
    if let Some(parent_id) = parent_ids.first() {
        let parent_commit = store.get_commit(parent_id)
            .map_err(|e| JjError::Other(e.to_string()))?;
        let parent_tree = parent_commit.tree();
        
        for (path, _) in parent_tree.entries() {
            let path_str = format!("{:?}", path);
            files.push(FileDiff {
                path: path_str.clone(),
                old_id: Some(path_str),
                new_id: None,
                diff_type: "removed".to_string(),
            });
        }
        
        for (path, _) in wc_tree.entries() {
            let path_str = format!("{:?}", path);
            let existing = files.iter_mut().find(|f| f.path == path_str);
            if let Some(existing) = existing {
                existing.diff_type = "modified".to_string();
                existing.new_id = Some(path_str);
            } else {
                files.push(FileDiff {
                    path: path_str,
                    old_id: None,
                    new_id: Some(format!("{:?}", path)),
                    diff_type: "added".to_string(),
                });
            }
        }
    } else {
        for (path, _) in wc_tree.entries() {
            files.push(FileDiff {
                path: format!("{:?}", path),
                old_id: None,
                new_id: Some(format!("{:?}", path)),
                diff_type: "added".to_string(),
            });
        }
    }
    
    files.sort_by(|a, b| a.path.cmp(&b.path));
    
    Ok(DiffInfo { files })
}

pub fn discard_changes(_workspace_path: &Path, _files: Vec<String>) -> JjResult<()> {
    Ok(())
}

pub fn get_file_diff(_workspace_path: String, file_path: String) -> JjResult<FileContentDiff> {
    Ok(FileContentDiff {
        path: file_path,
        old_content: None,
        new_content: None,
        hunks: Vec::new(),
    })
}

pub fn rebase_commit(workspace_path: &Path, commit_id: String, destination_commit_id: String) -> JjResult<CommitInfo> {
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
    
    let repo = pollster::block_on(async {
        let repo_loader = workspace.repo_loader();
        repo_loader.load_at_head().await.unwrap()
    });
    
    let store = repo.store();
    
    fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, JjError> {
        (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i+2], 16))
            .collect::<Result<Vec<u8>, _>>()
            .map_err(|e| JjError::Other(format!("Invalid hex: {}", e)))
    }
    
    let commit_id_bytes = hex_to_bytes(&commit_id)?;
    let commit_id = CommitId::from_bytes(&commit_id_bytes);
    
    let dest_commit_id_bytes = hex_to_bytes(&destination_commit_id)?;
    let dest_commit_id = CommitId::from_bytes(&dest_commit_id_bytes);
    
    let old_commit = store.get_commit(&commit_id)
        .map_err(|e| JjError::Other(format!("Failed to get commit to rebase: {}", e)))?;
    
    let dest_commit = store.get_commit(&dest_commit_id)
        .map_err(|e| JjError::Other(format!("Failed to get destination commit: {}", e)))?;
    
    let new_parents = vec![dest_commit.id().clone()];
    
    let rebased_commit = pollster::block_on(async {
        let repo_loader = workspace.repo_loader();
        let repo = repo_loader.load_at_head().await.unwrap();
        
        let mut tx = repo.start_transaction();
        
        let tx_repo = tx.repo_mut();
        
        let rebased_commit = rewrite::rebase_commit(tx_repo, old_commit, new_parents)
            .await
            .map_err(|e| JjError::Other(format!("Failed to rebase commit: {}", e)))?;
        
        tx.commit(&format!("Rebase commit {} onto {}", commit_id.hex(), dest_commit_id.hex()))
            .await
            .map_err(|e| JjError::Other(format!("Failed to commit rebase: {}", e)))?;
        
        Ok::<_, JjError>(rebased_commit)
    })?;
    
    Ok(CommitInfo {
        change_id: rebased_commit.change_id().hex(),
        commit_id: rebased_commit.id().hex(),
        description: rebased_commit.description().to_string(),
        author: rebased_commit.author().name.clone(),
        timestamp: rebased_commit.committer().timestamp.timestamp.0,
    })
}
