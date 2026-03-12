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

<svelte:window onkeydown={handleKeydown} />

<div class="relative inline-block">
  <button class="flex items-center gap-2 px-3 py-2 bg-[var(--color-bg-secondary)] border border-gray-200 rounded-md cursor-pointer text-[13px] text-[var(--color-text-primary)] min-w-[150px] hover:border-[var(--color-accent)] transition-colors" onclick={toggleDropdown} aria-expanded={isOpen}>
    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <line x1="6" y1="3" x2="6" y2="15"></line>
      <circle cx="18" cy="6" r="3"></circle>
      <circle cx="6" cy="18" r="3"></circle>
      <path d="M18 9a9 9 0 0 1-9 9"></path>
    </svg>
    <span class="flex-1 text-left overflow-hidden text-ellipsis whitespace-nowrap">{$activeBranch || 'Select branch'}</span>
    <svg class="text-[var(--color-text-secondary)] transition-transform" class:rotate-180={isOpen} xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <polyline points="6 9 12 15 18 9"></polyline>
    </svg>
  </button>

  {#if isOpen}
    <div class="fixed inset-0 z-10" onclick={closeDropdown} role="presentation"></div>
    <div class="absolute top-full left-0 right-0 mt-1 bg-white border border-gray-200 rounded-lg shadow-lg z-11 max-h-[300px] overflow-y-auto">
      <div class="px-3 py-2 text-[11px] font-semibold uppercase text-[var(--color-text-muted)] border-b border-gray-100">Switch branch</div>
      <ul class="list-none p-1 m-0">
        {#each $branches as branch}
          <li>
            <button 
              class="flex items-center justify-between w-full px-3 py-2 rounded text-[13px] text-[var(--color-text-primary)] border-none bg-transparent cursor-pointer text-left hover:bg-gray-100"
              class:bg-[var(--color-accent-light)]={$activeBranch === branch.name}
              class:text-[var(--color-accent)]={$activeBranch === branch.name}
              onclick={() => selectBranch(branch)}
            >
              <span class="overflow-hidden text-ellipsis whitespace-nowrap">{branch.name}</span>
              {#if $activeBranch === branch.name}
                <svg class="text-[var(--color-accent)] flex-shrink-0" xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <polyline points="20 6 9 17 4 12"></polyline>
                </svg>
              {/if}
            </button>
          </li>
        {/each}
      </ul>
      {#if $branches.length === 0}
        <p class="p-4 text-center text-[var(--color-text-muted)] text-sm">No branches available</p>
      {/if}
    </div>
  {/if}
</div>
