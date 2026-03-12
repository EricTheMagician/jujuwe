import { writable, derived, get } from 'svelte/store';
import type { BranchInfo, CommitInfo, RepoInfo, DiffInfo, FileContentDiff } from '$lib/tauri/api';
import * as api from '$lib/tauri/api';
import { startFileWatcher, stopFileWatcher, onFileChange } from '$lib/tauri/api';
import type { UnlistenFn } from '@tauri-apps/api/event';

export const repoPath = writable<string | null>(null);

export const fileWatcherActive = writable<boolean>(false);

let fileWatcherUnsubscribe: UnlistenFn | null = null;

export const repoInfo = writable<RepoInfo | null>(null);

export const branches = writable<BranchInfo[]>([]);

export const activeBranch = writable<string | null>(null);

export const commits = writable<CommitInfo[]>([]);

export const workingCopyDiff = writable<DiffInfo | null>(null);

export const stagedFiles = writable<Set<string>>(new Set());

export const isLoading = writable<boolean>(false);

export const error = writable<string | null>(null);

export const currentWorkspace = writable<string | null>(null);

export const selectedFile = writable<string | null>(null);

export const selectedFileDiff = writable<FileContentDiff | null>(null);

export async function selectFileForDiff(filePath: string): Promise<void> {
  selectedFile.set(filePath);
  const diff = await getFileDiffContent(filePath);
  selectedFileDiff.set(diff);
}

export function clearSelectedFile(): void {
  selectedFile.set(null);
  selectedFileDiff.set(null);
}

export async function openRepository(path: string): Promise<void> {
  isLoading.set(true);
  error.set(null);
  
  try {
    const root = await api.openWorkspace(path);
    currentWorkspace.set(root);
    repoPath.set(path);
    
    const info = await api.getRepoInfo(path);
    repoInfo.set(info);
    
    await refreshBranches(path);
    await refreshCommits(path);
    
    await startFileWatcherForRepo(path);
  } catch (e) {
    error.set(e instanceof Error ? e.message : String(e));
    throw e;
  } finally {
    isLoading.set(false);
  }
}

async function startFileWatcherForRepo(path: string): Promise<void> {
  try {
    await startFileWatcher(path);
    fileWatcherActive.set(true);
    
    fileWatcherUnsubscribe = await onFileChange(async () => {
      await refreshDiff(path);
    });
  } catch (e) {
    console.error('Failed to start file watcher:', e);
    fileWatcherActive.set(false);
  }
}

export async function closeFileWatcher(): Promise<void> {
  const repoPathValue = get(repoPath);
  
  if (fileWatcherUnsubscribe) {
    fileWatcherUnsubscribe();
    fileWatcherUnsubscribe = null;
  }
  
  if (get(fileWatcherActive) && repoPathValue) {
    try {
      await stopFileWatcher(repoPathValue);
    } catch (e) {
      console.error('Failed to stop file watcher:', e);
    }
  }
  
  fileWatcherActive.set(false);
}

export async function refreshBranches(path: string): Promise<void> {
  try {
    const branchList = await api.listBranches(path);
    branches.set(branchList);
    
    if (branchList.length > 0 && !get(activeBranch)) {
      activeBranch.set(branchList[0].name);
    }
  } catch (e) {
    error.set(e instanceof Error ? e.message : String(e));
  }
}

export async function refreshCommits(path: string): Promise<void> {
  try {
    const commitList = await api.getCommits(path);
    commits.set(commitList);
  } catch (e) {
    error.set(e instanceof Error ? e.message : String(e));
  }
}

export async function refreshDiff(path: string): Promise<void> {
  try {
    const diff = await api.getDiff(path);
    workingCopyDiff.set(diff);
  } catch (e) {
    error.set(e instanceof Error ? e.message : String(e));
  }
}

export async function createNewBranch(path: string, name: string): Promise<BranchInfo | null> {
  isLoading.set(true);
  error.set(null);
  
  try {
    const branch = await api.createBranch(path, name);
    await refreshBranches(path);
    activeBranch.set(name);
    return branch;
  } catch (e) {
    error.set(e instanceof Error ? e.message : String(e));
    return null;
  } finally {
    isLoading.set(false);
  }
}

export async function createNewCommit(path: string, message: string): Promise<CommitInfo | null> {
  isLoading.set(true);
  error.set(null);
  
  try {
    const commit = await api.createCommit(path, message);
    await refreshCommits(path);
    return commit;
  } catch (e) {
    error.set(e instanceof Error ? e.message : String(e));
    return null;
  } finally {
    isLoading.set(false);
  }
}

export async function uncommitCurrent(path: string): Promise<CommitInfo | null> {
  isLoading.set(true);
  error.set(null);
  
  try {
    const commit = await api.uncommit(path);
    await refreshCommits(path);
    return commit;
  } catch (e) {
    error.set(e instanceof Error ? e.message : String(e));
    return null;
  } finally {
    isLoading.set(false);
  }
}

export async function amendCurrentCommit(path: string, message: string): Promise<CommitInfo | null> {
  isLoading.set(true);
  error.set(null);
  
  try {
    const commit = await api.amendCommit(path, message);
    await refreshCommits(path);
    return commit;
  } catch (e) {
    error.set(e instanceof Error ? e.message : String(e));
    return null;
  } finally {
    isLoading.set(false);
  }
}

export async function rebaseCommit(path: string, commitId: string, destinationCommitId: string): Promise<CommitInfo | null> {
  isLoading.set(true);
  error.set(null);
  
  try {
    const commit = await api.rebaseCommit(path, commitId, destinationCommitId);
    await refreshCommits(path);
    await refreshBranches(path);
    return commit;
  } catch (e) {
    error.set(e instanceof Error ? e.message : String(e));
    return null;
  } finally {
    isLoading.set(false);
  }
}

export async function reorderCommits(path: string, commitIds: string[], destinationCommitId: string): Promise<CommitInfo[] | null> {
  isLoading.set(true);
  error.set(null);
  
  try {
    const result = await api.reorderCommits(path, commitIds, destinationCommitId);
    await refreshCommits(path);
    await refreshBranches(path);
    return result;
  } catch (e) {
    error.set(e instanceof Error ? e.message : String(e));
    return null;
  } finally {
    isLoading.set(false);
  }
}

export async function mergeBranches(path: string, branch1CommitId: string, branch2CommitId: string): Promise<CommitInfo | null> {
  isLoading.set(true);
  error.set(null);
  
  try {
    const commit = await api.mergeBranches(path, branch1CommitId, branch2CommitId);
    await refreshCommits(path);
    await refreshBranches(path);
    return commit;
  } catch (e) {
    error.set(e instanceof Error ? e.message : String(e));
    return null;
  } finally {
    isLoading.set(false);
  }
}

export async function splitCommit(path: string, commitId: string, filesToSplit: string[]): Promise<CommitInfo[] | null> {
  isLoading.set(true);
  error.set(null);
  
  try {
    const result = await api.splitCommit(path, commitId, filesToSplit);
    await refreshCommits(path);
    await refreshBranches(path);
    return result;
  } catch (e) {
    error.set(e instanceof Error ? e.message : String(e));
    return null;
  } finally {
    isLoading.set(false);
  }
}

export function stageFile(filePath: string): void {
  stagedFiles.update(files => {
    const newSet = new Set(files);
    newSet.add(filePath);
    return newSet;
  });
}

export function unstageFile(filePath: string): void {
  stagedFiles.update(files => {
    const newSet = new Set(files);
    newSet.delete(filePath);
    return newSet;
  });
}

export function toggleFileStaged(filePath: string): void {
  stagedFiles.update(files => {
    const newSet = new Set(files);
    if (newSet.has(filePath)) {
      newSet.delete(filePath);
    } else {
      newSet.add(filePath);
    }
    return newSet;
  });
}

export function clearStagedFiles(): void {
  stagedFiles.set(new Set());
}

export function stageAllFiles(): void {
  const diff = get(workingCopyDiff);
  if (diff) {
    const allFiles = new Set(diff.files.map(f => f.path));
    stagedFiles.set(allFiles);
  }
}

export async function discardFile(path: string, filePath: string): Promise<void> {
  const repoPathValue = get(repoPath);
  if (repoPathValue) {
    try {
      await api.discardChanges(repoPathValue, [filePath]);
      unstageFile(filePath);
      await refreshDiff(repoPathValue);
    } catch (e) {
      error.set(e instanceof Error ? e.message : String(e));
    }
  }
}

export async function discardStagedFiles(): Promise<void> {
  const repoPathValue = get(repoPath);
  const staged = get(stagedFiles);
  if (repoPathValue && staged.size > 0) {
    try {
      await api.discardChanges(repoPathValue, Array.from(staged));
      clearStagedFiles();
      await refreshDiff(repoPathValue);
    } catch (e) {
      error.set(e instanceof Error ? e.message : String(e));
    }
  }
}

export async function getFileDiffContent(filePath: string): Promise<FileContentDiff | null> {
  const repoPathValue = get(repoPath);
  if (repoPathValue) {
    try {
      return await api.getFileDiff(repoPathValue, filePath);
    } catch (e) {
      error.set(e instanceof Error ? e.message : String(e));
      return null;
    }
  }
  return null;
}
