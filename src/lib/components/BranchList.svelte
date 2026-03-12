<script lang="ts">
  import { branches, activeBranch, refreshBranches, repoPath } from '$lib/stores/repo';
  import type { BranchInfo } from '$lib/tauri/api';

  let { onBranchSelect }: { onBranchSelect?: (branch: BranchInfo) => void } = $props();

  async function selectBranch(branch: BranchInfo) {
    activeBranch.set(branch.name);
    if (onBranchSelect) {
      onBranchSelect(branch);
    }
  }

  async function handleRefresh() {
    const path = $repoPath;
    if (path) {
      await refreshBranches(path);
    }
  }
</script>

<div class="branch-list">
  <div class="header">
    <h3>Branches</h3>
    <button class="refresh-btn" onclick={handleRefresh} title="Refresh branches">
      <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"/>
        <path d="M21 3v5h-5"/>
      </svg>
    </button>
  </div>
  
  <ul class="branches">
    {#each $branches as branch}
      <li>
        <button 
          class="branch-item" 
          class:active={$activeBranch === branch.name}
          onclick={() => selectBranch(branch)}
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="6" y1="3" x2="6" y2="15"></line>
            <circle cx="18" cy="6" r="3"></circle>
            <circle cx="6" cy="18" r="3"></circle>
            <path d="M18 9a9 9 0 0 1-9 9"></path>
          </svg>
          <span class="branch-name">{branch.name}</span>
          {#if branch.commit_id}
            <span class="commit-id" title={branch.commit_id}>{branch.commit_id.slice(0, 7)}</span>
          {/if}
        </button>
      </li>
    {/each}
  </ul>
  
  {#if $branches.length === 0}
    <p class="empty">No branches found</p>
  {/if}
</div>

<style>
  .branch-list {
    background: var(--bg-secondary, #f5f5f5);
    border-radius: 8px;
    padding: 12px;
    min-width: 200px;
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

  .refresh-btn:hover {
    background: var(--bg-hover, #e0e0e0);
    color: var(--text-primary, #333);
  }

  .branches {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .branch-item {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 8px 12px;
    border: none;
    background: transparent;
    border-radius: 6px;
    cursor: pointer;
    text-align: left;
    color: var(--text-primary, #333);
    font-size: 13px;
    transition: background-color 0.15s;
  }

  .branch-item:hover {
    background: var(--bg-hover, #e8e8e8);
  }

  .branch-item.active {
    background: var(--accent-bg, #e3efff);
    color: var(--accent-color, #0066cc);
  }

  .branch-name {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .commit-id {
    font-family: monospace;
    font-size: 11px;
    color: var(--text-secondary, #888);
    background: var(--bg-tertiary, #eaeaea);
    padding: 2px 4px;
    border-radius: 3px;
  }

  .empty {
    text-align: center;
    color: var(--text-secondary, #888);
    font-size: 13px;
    padding: 20px;
  }
</style>
