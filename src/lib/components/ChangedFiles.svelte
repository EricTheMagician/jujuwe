<script lang="ts">
  import { workingCopyDiff, repoPath, refreshDiff, isLoadingDiff, diffError, stagedFiles, toggleFileStaged, stageAllFiles, clearStagedFiles, selectFileForDiff } from '$lib/stores/repo';
  import type { DiffInfo, FileDiff } from '$lib/tauri/api';
  import LoadingSpinner from './LoadingSpinner.svelte';

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

  let stagedCount = $derived($stagedFiles.size);
  let totalCount = $derived($workingCopyDiff?.files.length ?? 0);
</script>

<div class="p-3 bg-[var(--color-bg-secondary)] rounded-lg min-w-[250px]">
  <div class="flex justify-between items-center mb-3">
    <h3 class="m-0 text-sm font-semibold text-[var(--color-text-primary)]">Changed Files</h3>
    <div class="flex gap-1 items-center">
      <button class="px-2 py-1 rounded text-[11px] border border-gray-200 bg-transparent text-[var(--color-text-secondary)] hover:bg-[var(--color-bg-hover)] hover:text-[var(--color-text-primary)] cursor-pointer" onclick={handleStageAll} title="Stage all">
        Stage All
      </button>
      <button class="px-2 py-1 rounded text-[11px] border border-gray-200 bg-transparent text-[var(--color-text-secondary)] hover:bg-[var(--color-bg-hover)] hover:text-[var(--color-text-primary)] cursor-pointer" onclick={handleUnstageAll} title="Unstage all">
        Unstage All
      </button>
      <button 
        class="p-1 rounded bg-transparent border-none cursor-pointer text-[var(--color-text-secondary)] flex items-center justify-center hover:bg-[var(--color-bg-hover)] hover:text-[var(--color-text-primary)] disabled:opacity-50 disabled:cursor-not-allowed"
        class:animate-spin={$isLoadingDiff}
        onclick={handleRefresh} 
        title="Refresh" 
        disabled={$isLoadingDiff}
      >
        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"/>
          <path d="M21 3v5h-5"/>
        </svg>
      </button>
    </div>
  </div>

  {#if stagedCount > 0}
    <div class="text-xs text-[var(--color-accent)] px-2 py-1 bg-[var(--color-accent-light)] rounded mb-2">
      {stagedCount} of {totalCount} files staged
    </div>
  {/if}

  {#if $isLoadingDiff}
    <div class="py-5">
      <LoadingSpinner size="medium" message="Loading changes..." />
    </div>
  {:else if $diffError}
    <div class="text-center py-3">
      <p class="text-xs text-[var(--color-removed-text)] mb-2">{$diffError}</p>
      <button 
        class="px-3 py-1 text-xs rounded bg-[var(--color-accent)] text-white hover:bg-[var(--color-accent-hover)]"
        onclick={handleRefresh}
      >
        Retry
      </button>
    </div>
  {:else if !$workingCopyDiff || $workingCopyDiff.files.length === 0}
    <p class="text-center text-[var(--color-text-muted)] text-sm py-5">No changes detected</p>
  {:else}
    <div class="flex flex-col gap-0.5 max-h-[300px] overflow-y-auto">
      {#each $workingCopyDiff.files as file}
        <button 
          class="flex items-center gap-2 px-2 py-1.5 rounded text-xs font-mono border-none bg-transparent w-full cursor-pointer text-left {getFileClass(file.diff_type)}"
          class:bg-[var(--color-accent-light)]={isStaged(file.path)}
          onclick={() => handleFileClick(file)}
        >
          <span class="w-4 text-center text-[var(--color-accent)] font-bold">{isStaged(file.path) ? '✓' : ' '}</span>
          <span class="w-4 text-center font-bold {file.diff_type === 'added' ? 'text-[var(--color-added)]' : file.diff_type === 'removed' ? 'text-[var(--color-removed)]' : 'text-[var(--color-modified)]'}">{getFileIcon(file.diff_type)}</span>
          <span class="flex-1 overflow-hidden text-ellipsis whitespace-nowrap text-[var(--color-text-primary)]">{file.path}</span>
          <span class="text-[10px] px-1 py-0.5 rounded uppercase {file.diff_type === 'added' ? 'bg-[var(--color-added-bg)] text-[var(--color-added-text)]' : file.diff_type === 'removed' ? 'bg-[var(--color-removed-bg)] text-[var(--color-removed-text)]' : 'bg-[var(--color-modified-bg)] text-[var(--color-modified-text)]'}">{file.diff_type}</span>
        </button>
      {/each}
    </div>
  {/if}
</div>
