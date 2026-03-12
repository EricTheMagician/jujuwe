<script lang="ts">
  import { branches, sortedBranches, activeBranch, refreshBranches, repoPath, setBranchOrder, isLoadingBranches, branchesError } from '$lib/stores/repo';
  import type { BranchInfo } from '$lib/tauri/api';
  import LoadingSpinner from './LoadingSpinner.svelte';

  let { onBranchSelect, onMerge }: { onBranchSelect?: (branch: BranchInfo) => void, onMerge?: () => void } = $props();

  let draggedIndex: number | null = $state(null);
  let dragOverIndex: number | null = $state(null);

  let localBranches = $state<BranchInfo[]>([]);

  function handleDragStart(index: number) {
    draggedIndex = index;
    localBranches = [...$sortedBranches];
  }

  function handleDragEnd() {
    const path = $repoPath;
    if (draggedIndex !== null && dragOverIndex !== null && draggedIndex !== dragOverIndex && path) {
      const newOrder = [...localBranches];
      const [removed] = newOrder.splice(draggedIndex, 1);
      newOrder.splice(dragOverIndex, 0, removed);
      setBranchOrder(path, newOrder.map(b => b.name));
    }
    draggedIndex = null;
    dragOverIndex = null;
    localBranches = [];
  }

  function handleDragOver(e: DragEvent, index: number) {
    e.preventDefault();
    dragOverIndex = index;
  }

  function handleDragLeave() {
    dragOverIndex = null;
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
  }

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

<div class="p-3 bg-[var(--color-bg-secondary)] rounded-lg min-w-[200px]">
  <div class="flex justify-between items-center mb-3">
    <h3 class="m-0 text-sm font-semibold text-[var(--color-text-primary)]">Branches</h3>
    <button class="p-1 rounded bg-transparent border-none cursor-pointer text-[var(--color-text-secondary)] flex items-center justify-center hover:bg-[var(--color-bg-hover)] hover:text-[var(--color-text-primary)]" class:animate-spin={$isLoadingBranches} onclick={handleRefresh} title="Refresh branches">
      <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"/>
        <path d="M21 3v5h-5"/>
      </svg>
    </button>
    {#if onMerge}
    <button class="p-1 rounded bg-transparent border-none cursor-pointer text-[var(--color-text-secondary)] flex items-center justify-center hover:bg-[var(--color-bg-hover)] hover:text-[var(--color-text-primary)]" onclick={onMerge} title="Merge branch">
      <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="18" cy="18" r="3"/>
        <circle cx="6" cy="6" r="3"/>
        <path d="M6 21V9a9 9 0 0 0 9 9"/>
      </svg>
    </button>
    {/if}
  </div>
  
  {#if $isLoadingBranches}
    <div class="py-5">
      <LoadingSpinner size="medium" message="Loading branches..." />
    </div>
  {:else if $branchesError}
    <div class="text-center py-3">
      <p class="text-xs text-[var(--color-removed-text)] mb-2">{$branchesError}</p>
      <button 
        class="px-3 py-1 text-xs rounded bg-[var(--color-accent)] text-white hover:bg-[var(--color-accent-hover)]"
        onclick={handleRefresh}
      >
        Retry
      </button>
    </div>
  {:else if $branches.length === 0}
    <p class="text-center text-[var(--color-text-muted)] text-sm py-5">No branches found</p>
  {:else}
    <ul class="list-none p-0 m-0">
      {#each $sortedBranches as branch, index}
        <li>
          <button 
            class="flex items-center gap-2 w-full px-3 py-2 rounded-md border-none bg-transparent cursor-pointer text-left text-[var(--color-text-primary)] text-[13px] transition-colors hover:bg-[var(--color-bg-hover)]"
            class:bg-[var(--color-accent-light)]={$activeBranch === branch.name}
            class:text-[var(--color-accent)]={$activeBranch === branch.name}
            class:dragging={draggedIndex === index}
            class:drag-over={dragOverIndex === index}
            draggable="true"
            ondragstart={() => handleDragStart(index)}
            ondragend={handleDragEnd}
            ondragover={(e) => handleDragOver(e, index)}
            ondragleave={handleDragLeave}
            ondrop={handleDrop}
            onclick={() => selectBranch(branch)}
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="6" y1="3" x2="6" y2="15"></line>
              <circle cx="18" cy="6" r="3"></circle>
              <circle cx="6" cy="18" r="3"></circle>
              <path d="M18 9a9 9 0 0 1-9 9"></path>
            </svg>
            <span class="flex-1 overflow-hidden text-ellipsis whitespace-nowrap">{branch.name}</span>
            {#if branch.commit_id}
              <span class="font-mono text-[11px] text-[var(--color-text-muted)] bg-[var(--color-bg-tertiary)] px-1 rounded">{branch.commit_id.slice(0, 7)}</span>
            {/if}
          </button>
        </li>
      {/each}
    </ul>
  {/if}
</div>

<style>
  button {
    cursor: grab;
  }
  
  button.dragging {
    opacity: 0.5;
  }
  
  button.drag-over {
    border-left: 3px solid var(--color-accent);
    background-color: var(--color-bg-hover);
  }
</style>
