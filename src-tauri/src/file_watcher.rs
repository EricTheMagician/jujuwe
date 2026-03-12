use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher, Event, EventKind};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;

pub struct FileWatcherState {
    watchers: HashMap<String, WatcherHandle>,
}

struct WatcherHandle {
    #[allow(dead_code)]
    watcher: RecommendedWatcher,
}

impl Default for FileWatcherState {
    fn default() -> Self {
        Self {
            watchers: HashMap::new(),
        }
    }
}

pub type FileWatcherStateHandle = Arc<Mutex<FileWatcherState>>;

pub fn create_file_watcher_state() -> FileWatcherStateHandle {
    Arc::new(Mutex::new(FileWatcherState::default()))
}

fn is_relevant_event(event: &Event) -> bool {
    matches!(
        event.kind,
        EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
    )
}

#[tauri::command]
pub async fn start_file_watcher(
    path: String,
    app_handle: AppHandle,
    state: tauri::State<'_, FileWatcherStateHandle>,
) -> Result<String, String> {
    let watch_path = PathBuf::from(&path);
    
    if !watch_path.exists() {
        return Err(format!("Path does not exist: {}", path));
    }

    let path_for_closure = path.clone();
    let app_handle_clone = app_handle.clone();

    let watcher = RecommendedWatcher::new(
        move |result: Result<Event, notify::Error>| {
            if let Ok(event) = result {
                if is_relevant_event(&event) {
                    for path in event.paths {
                        let payload = serde_json::json!({
                            "path": path.to_string_lossy(),
                            "kind": format!("{:?}", event.kind),
                        });
                        let _ = app_handle_clone.emit("file-changed", payload);
                    }
                }
            }
        },
        Config::default(),
    ).map_err(|e| format!("Failed to create watcher: {}", e))?;

    let mut watcher = watcher;
    watcher.watch(&watch_path, RecursiveMode::Recursive)
        .map_err(|e| format!("Failed to watch path: {}", e))?;

    let mut state = state.lock().await;
    let key = path_for_closure.clone();
    state.watchers.insert(key, WatcherHandle { watcher });

    Ok(format!("Started watching: {}", path))
}

#[tauri::command]
pub async fn stop_file_watcher(
    path: String,
    state: tauri::State<'_, FileWatcherStateHandle>,
) -> Result<String, String> {
    let mut state = state.lock().await;
    
    if state.watchers.remove(&path).is_some() {
        Ok(format!("Stopped watching: {}", path))
    } else {
        Err(format!("No watcher found for path: {}", path))
    }
}
