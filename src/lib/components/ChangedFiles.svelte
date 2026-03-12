<script lang="ts">
  import { workingCopyDiff, repoPath, refreshDiff, isLoading, stagedFiles, toggleFileStaged, stageAllFiles, clearStagedFiles, selectFileForDiff } from '$lib/stores/repo';
  import type { DiffInfo, FileDiff } from '$lib/tauri/api';

  async function handleRefresh() {
    const path = $repoPath;
    if (path) {
      await refreshDiff(path);
    }
  }

  function handleFileClick(file: FileDiff) {
    toggleFileStaged(file.path);
    selectFileForDiff(file.path);
  }

  function handleStageAll() {
    stageAllFiles();
  }

  function handleUnstageAll() {
    clearStagedFiles();
  }

  function isStaged(path: string): boolean {
    return $stagedFiles.has(path);
  }

  function getFileIcon(diffType: string): string {
    switch (diffType) {
      case 'added': return '+';
      case 'removed': return '-';
      case 'modified': return '~';
      default: return ' ';
    }
  }

  function getFileClass(diffType: string): string {
    switch (diffType) {
      case 'added': return 'file-added';
      case 'removed': return 'file-removed';
      case 'modified': return 'file-modified';
      default: return '';
    }
  }

  $: stagedCount = $stagedFiles.size;
  $: totalCount = $workingCopyDiff?.files.length ?? 0;
</script>

<div class="changed-files">
  <div class="header">
    <h3>Changed Files</h3>
    <div class="actions">
      <button class="action-btn" onclick={handleStageAll} title="Stage all">
        Stage All
      </button>
      <button class="action-btn" onclick={handleUnstageAll} title="Unstage all">
        Unstage All
      </button>
      <button class="refresh-btn" onclick={handleRefresh} title="Refresh" disabled={$isLoading}>
        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"/>
          <path d="M21 3v5h-5"/>
        </svg>
      </button>
    </div>
  </div>

  {#if stagedCount > 0}
    <div class="staged-info">
      {stagedCount} of {totalCount} files staged
    </div>
  {/if}

  <div class="file-list">
    {#if $workingCopyDiff && $workingCopyDiff.files.length > 0}
      {#each $workingCopyDiff.files as file}
        <button 
          class="file-item {getFileClass(file.diff_type)}" 
          class:staged={isStaged(file.path)}
          onclick={() => handleFileClick(file)}
        >
          <span class="checkbox">{isStaged(file.path) ? '✓' : ' '}</span>
          <span class="file-icon">{getFileIcon(file.diff_type)}</span>
          <span class="file-path">{file.path}</span>
          <span class="file-type">{file.diff_type}</span>
        </button>
      {/each}
    {:else}
      <p class="empty">No changes detected</p>
    {/if}
  </div>
</div>

<style>
  .changed-files {
    background: var(--bg-secondary, #f5f5f5);
    border-radius: 8px;
    padding: 12px;
    min-width: 250px;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
  }

  .header h3 {
    margin: 0;
    font-size: 14px;
    font-weight: 600;
    color: var(--text-primary, #333);
  }

  .actions {
    display: flex;
    gap: 4px;
    align-items: center;
  }

  .action-btn {
    background: none;
    border: 1px solid var(--border-color, #ddd);
    cursor: pointer;
    padding: 4px 8px;
    border-radius: 4px;
    color: var(--text-secondary, #666);
    font-size: 11px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .action-btn:hover {
    background: var(--bg-hover, #e0e0e0);
    color: var(--text-primary, #333);
  }

  .refresh-btn {
    background: none;
    border: none;
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
    color: var(--text-secondary, #666);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .refresh-btn:hover:not(:disabled) {
    background: var(--bg-hover, #e0e0e0);
    color: var(--text-primary, #333);
  }

  .refresh-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .staged-info {
    font-size: 12px;
    color: var(--accent-color, #0066cc);
    padding: 4px 8px;
    background: var(--accent-bg, #e3efff);
    border-radius: 4px;
    margin-bottom: 8px;
  }

  .file-list {
    display: flex;
    flex-direction: column;
    gap: 2px;
    max-height: 300px;
    overflow-y: auto;
  }

  .file-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 8px;
    border-radius: 4px;
    font-size: 12px;
    font-family: monospace;
    border: none;
    background: transparent;
    width: 100%;
    cursor: pointer;
    text-align: left;
  }

  .file-item:hover {
    background: var(--bg-hover, #e8e8e8);
  }

  .file-item.staged {
    background: var(--accent-bg, #e3efff);
  }

  .checkbox {
    width: 16px;
    text-align: center;
    color: var(--accent-color, #0066cc);
    font-weight: bold;
  }

  .file-icon {
    width: 16px;
    text-align: center;
    font-weight: bold;
  }

  .file-added .file-icon {
    color: #22c55e;
  }

  .file-removed .file-icon {
    color: #ef4444;
  }

  .file-modified .file-icon {
    color: #f59e0b;
  }

  .file-path {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: var(--text-primary, #333);
  }

  .file-type {
    font-size: 10px;
    padding: 2px 4px;
    border-radius: 3px;
    background: var(--bg-tertiary, #eaeaea);
    color: var(--text-secondary, #666);
    text-transform: uppercase;
  }

  .file-added .file-type {
    background: #dcfce7;
    color: #166534;
  }

  .file-removed .file-type {
    background: #fee2e1;
    color: #991b1b;
  }

  .file-modified .file-type {
    background: #fef3c7;
    color: #92400e;
  }

  .empty {
    text-align: center;
    color: var(--text-secondary, #888);
    font-size: 13px;
    padding: 20px;
  }
</style>
