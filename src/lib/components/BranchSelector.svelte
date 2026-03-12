<script lang="ts">
  import { branches, activeBranch } from '$lib/stores/repo';
  import type { BranchInfo } from '$lib/tauri/api';

  let isOpen = $state(false);

  function toggleDropdown() {
    isOpen = !isOpen;
  }

  function closeDropdown() {
    isOpen = false;
  }

  function selectBranch(branch: BranchInfo) {
    activeBranch.set(branch.name);
    closeDropdown();
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      closeDropdown();
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="branch-selector">
  <button class="selector-button" onclick={toggleDropdown} aria-expanded={isOpen}>
    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <line x1="6" y1="3" x2="6" y2="15"></line>
      <circle cx="18" cy="6" r="3"></circle>
      <circle cx="6" cy="18" r="3"></circle>
      <path d="M18 9a9 9 0 0 1-9 9"></path>
    </svg>
    <span class="current-branch">{$activeBranch || 'Select branch'}</span>
    <svg class="chevron" class:open={isOpen} xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <polyline points="6 9 12 15 18 9"></polyline>
    </svg>
  </button>

  {#if isOpen}
    <div class="dropdown-overlay" onclick={closeDropdown} role="presentation"></div>
    <div class="dropdown-menu">
      <div class="dropdown-header">Switch branch</div>
      <ul class="branch-options">
        {#each $branches as branch}
          <li>
            <button 
              class="branch-option" 
              class:selected={$activeBranch === branch.name}
              onclick={() => selectBranch(branch)}
            >
              <span class="branch-name">{branch.name}</span>
              {#if $activeBranch === branch.name}
                <svg class="check" xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <polyline points="20 6 9 17 4 12"></polyline>
                </svg>
              {/if}
            </button>
          </li>
        {/each}
      </ul>
      {#if $branches.length === 0}
        <p class="empty">No branches available</p>
      {/if}
    </div>
  {/if}
</div>

<style>
  .branch-selector {
    position: relative;
    display: inline-block;
  }

  .selector-button {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    background: var(--bg-secondary, #f5f5f5);
    border: 1px solid var(--border-color, #ddd);
    border-radius: 6px;
    cursor: pointer;
    font-size: 13px;
    color: var(--text-primary, #333);
    min-width: 150px;
    transition: border-color 0.15s;
  }

  .selector-button:hover {
    border-color: var(--accent-color, #0066cc);
  }

  .current-branch {
    flex: 1;
    text-align: left;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .chevron {
    transition: transform 0.2s;
    color: var(--text-secondary, #666);
  }

  .chevron.open {
    transform: rotate(180deg);
  }

  .dropdown-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    z-index: 10;
  }

  .dropdown-menu {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    margin-top: 4px;
    background: var(--bg-primary, #fff);
    border: 1px solid var(--border-color, #ddd);
    border-radius: 8px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    z-index: 11;
    max-height: 300px;
    overflow-y: auto;
  }

  .dropdown-header {
    padding: 8px 12px;
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    color: var(--text-secondary, #888);
    border-bottom: 1px solid var(--border-color, #eee);
  }

  .branch-options {
    list-style: none;
    padding: 4px;
    margin: 0;
  }

  .branch-option {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 8px 12px;
    border: none;
    background: transparent;
    border-radius: 4px;
    cursor: pointer;
    font-size: 13px;
    color: var(--text-primary, #333);
    text-align: left;
  }

  .branch-option:hover {
    background: var(--bg-hover, #f0f0f0);
  }

  .branch-option.selected {
    background: var(--accent-bg, #e3efff);
    color: var(--accent-color, #0066cc);
  }

  .branch-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .check {
    color: var(--accent-color, #0066cc);
    flex-shrink: 0;
  }

  .empty {
    padding: 16px;
    text-align: center;
    color: var(--text-secondary, #888);
    font-size: 13px;
  }
</style>
