import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { UnlistenFn } from '@tauri-apps/api/event';

export interface BranchInfo {
  name: string;
  commit_id: string | null;
}

export interface CommitInfo {
  change_id: string;
  commit_id: string;
  description: string;
  author: string;
  timestamp: number;
}

export interface FileDiff {
  path: string;
  old_id: string | null;
  new_id: string | null;
  diff_type: string;
}

export interface DiffInfo {
  files: FileDiff[];
}

export interface RepoInfo {
  workspace_root: string;
  repo_path: string;
  workspace_name: string;
}

export async function openWorkspace(path: string): Promise<string> {
  return invoke<string>('open_workspace', { path });
}

export async function getRepoInfo(path: string): Promise<RepoInfo> {
  return invoke<RepoInfo>('get_repo_info', { path });
}

export async function listBranches(path: string): Promise<BranchInfo[]> {
  return invoke<BranchInfo[]>('list_branches', { path });
}

export async function createBranch(path: string, name: string): Promise<BranchInfo> {
  return invoke<BranchInfo>('create_branch', { path, name });
}

export async function getCommits(path: string, limit?: number): Promise<CommitInfo[]> {
  return invoke<CommitInfo[]>('get_commits', { path, limit });
}

export async function uncommit(path: string): Promise<CommitInfo> {
  return invoke<CommitInfo>('uncommit_cmd', { path });
}

export async function createCommit(path: string, message: string): Promise<CommitInfo> {
  return invoke<CommitInfo>('create_commit_cmd', { path, message });
}

export async function amendCommit(path: string, message: string): Promise<CommitInfo> {
  return invoke<CommitInfo>('amend_commit_cmd', { path, message });
}

export async function rebaseCommit(path: string, commitId: string, destinationCommitId: string): Promise<CommitInfo> {
  return invoke<CommitInfo>('rebase_commit_cmd', { path, commitId, destinationCommitId });
}

export async function reorderCommits(path: string, commitIds: string[], destinationCommitId: string): Promise<CommitInfo[]> {
  return invoke<CommitInfo[]>('reorder_commits_cmd', { path, commitIds, destinationCommitId });
}

export async function mergeBranches(path: string, branch1CommitId: string, branch2CommitId: string): Promise<CommitInfo> {
  return invoke<CommitInfo>('merge_branches_cmd', { path, branch1CommitId, branch2CommitId });
}

export async function splitCommit(path: string, commitId: string, filesToSplit: string[]): Promise<CommitInfo[]> {
  return invoke<CommitInfo[]>('split_commit_cmd', { path, commitId, filesToSplit });
}

export async function getDiff(path: string): Promise<DiffInfo> {
  return invoke<DiffInfo>('get_diff', { path });
}

export async function discardChanges(path: string, files: string[]): Promise<void> {
  return invoke<void>('discard_changes_cmd', { path, files });
}

export interface FileContentDiff {
  path: string;
  old_content: string | null;
  new_content: string | null;
  hunks: DiffHunk[];
}

export interface DiffHunk {
  old_start: number;
  old_lines: number;
  new_start: number;
  new_lines: number;
  content: string;
}

export async function getFileDiff(path: string, filePath: string): Promise<FileContentDiff> {
  return invoke<FileContentDiff>('get_file_diff', { path, filePath });
}

export interface FileChangeEvent {
  path: string;
  kind: string;
}

export async function startFileWatcher(path: string): Promise<string> {
  return invoke<string>('start_file_watcher', { path });
}

export async function stopFileWatcher(path: string): Promise<string> {
  return invoke<string>('stop_file_watcher', { path });
}

export async function onFileChange(callback: (event: { payload: FileChangeEvent }) => void): Promise<UnlistenFn> {
  return listen<FileChangeEvent>('file-change', (event) => {
    callback({ payload: event.payload });
  });
}
