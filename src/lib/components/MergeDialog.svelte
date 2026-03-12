<script lang="ts">
  import { branches, mergeBranches, repoPath, refreshBranches, refreshCommits, activeBranch } from '$lib/stores/repo';
  import type { BranchInfo } from '$lib/tauri/api';

  interface Props {
    open: boolean;
    onClose: () => void;
  }

  let { open = $bindable(), onClose }: Props = $props();

  let targetBranch = $state<BranchInfo | null>(null);
  let sourceBranch = $state<BranchInfo | null>(null);
  let isMerging = $state(false);
  let errorMessage = $state<string | null>(null);

  let branchList = $derived($branches);
  let active = $derived($activeBranch);

  $effect(() => {
    if (open && branchList.length > 0) {
      const activeBranchInfo = branchList.find(b => b.name === active) || branchList[0];
      targetBranch = activeBranchInfo;
      
      const otherBranches = branchList.filter(b => b.name !== activeBranchInfo.name);
      sourceBranch = otherBranches.length > 0 ? otherBranches[0] : null;
      
      errorMessage = null;
    }
  });

  let availableSourceBranches = $derived(
    targetBranch 
      ? branchList.filter(b => b.name !== targetBranch?.name)
      : branchList
  );

  let canMerge = $derived(
    targetBranch !== null && 
    sourceBranch !== null && 
    targetBranch.name !== sourceBranch.name &&
    !isMerging
  );

  let sourceCommitId = $derived(sourceBranch?.commit_id || '');
  let sourceDescription = $derived('Source commit');

  async function handleMerge() {
    if (!canMerge || !targetBranch || !sourceBranch || !$repoPath) return;

    isMerging = true;
    errorMessage = null;

    try {
      const result = await mergeBranches($repoPath, targetBranch.commit_id || '', sourceCommitId);
      
      if (result) {
        await refreshBranches($repoPath);
        await refreshCommits($repoPath);
        open = false;
        onClose();
      } else {
        errorMessage = 'Merge failed';
      }
    } catch (e) {
      errorMessage = e instanceof Error ? e.message : String(e);
    } finally {
      isMerging = false;
    }
  }

  function handleCancel() {
    open = false;
    onClose();
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape' && open) {
      handleCancel();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <div class="fixed inset-0 z-50 flex items-center justify-center">
    <div class="absolute inset-0 bg-black/50" onclick={handleCancel} role="presentation"></div>
    
    <div class="relative bg-white rounded-lg shadow-xl w-[480px] max-w-[90vw]">
      <div class="flex items-center justify-between px-6 py-4 border-b border-gray-200">
        <h2 class="text-lg font-semibold text-[var(--color-text-primary)]">Merge Branches</h2>
        <button 
          class="p-1 text-[var(--color-text-muted)] hover:text-[var(--color-text-primary)] rounded transition-colors"
          onclick={handleCancel}
          aria-label="Close"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="18" y1="6" x2="6" y2="18"></line>
            <line x1="6" y1="6" x2="18" y2="18"></line>
          </svg>
        </button>
      </div>

      <div class="p-6 space-y-5">
        {#if branchList.length === 0}
          <p class="text-center text-[var(--color-text-muted)] py-4">Loading branches...</p>
        {:else}
          <div class="space-y-4">
            <div>
              <label for="target-branch" class="block text-sm font-medium text-[var(--color-text-primary)] mb-2">
                Merge into (target)
              </label>
              <select
                id="target-branch"
                class="w-full px-3 py-2 bg-[var(--color-bg-secondary)] border border-gray-200 rounded-md text-[13px] text-[var(--color-text-primary)] focus:outline-none focus:border-[var(--color-accent)]"
                bind:value={targetBranch}
              >
                {#each branchList as branch}
                  <option value={branch}>{branch.name}</option>
                {/each}
              </select>
            </div>

            <div>
              <label for="source-branch" class="block text-sm font-medium text-[var(--color-text-primary)] mb-2">
                Merge from (source)
              </label>
              <select
                id="source-branch"
                class="w-full px-3 py-2 bg-[var(--color-bg-secondary)] border border-gray-200 rounded-md text-[13px] text-[var(--color-text-primary)] focus:outline-none focus:border-[var(--color-accent)]"
                bind:value={sourceBranch}
              >
                {#each availableSourceBranches as branch}
                  <option value={branch}>{branch.name}</option>
                {/each}
              </select>
            </div>
          </div>

          <div class="p-4 bg-[var(--color-bg-secondary)] rounded-lg border border-gray-200">
            <h3 class="text-sm font-medium text-[var(--color-text-primary)] mb-2">Preview</h3>
            {#if sourceBranch?.commit_id}
              <div class="space-y-2">
                <div class="flex items-center gap-2">
                  <span class="text-xs text-[var(--color-text-muted)]">Commit:</span>
                  <code class="text-xs font-mono text-[var(--color-accent)] bg-white px-2 py-1 rounded border border-gray-200">
                    {sourceBranch.commit_id.substring(0, 7)}
                  </code>
                </div>
              </div>
            {:else}
              <p class="text-xs text-[var(--color-text-muted)]">No commit to preview</p>
            {/if}
          </div>

          {#if errorMessage}
            <div class="p-3 bg-red-50 border border-red-200 rounded-md">
              <p class="text-sm text-red-600">{errorMessage}</p>
            </div>
          {/if}
        {/if}
      </div>

      <div class="flex justify-end gap-3 px-6 py-4 border-t border-gray-200">
        <button
          class="px-4 py-2 text-sm font-medium text-[var(--color-text-primary)] bg-[var(--color-bg-secondary)] border border-gray-200 rounded-md hover:bg-gray-100 transition-colors"
          onclick={handleCancel}
          disabled={isMerging}
        >
          Cancel
        </button>
        <button
          class="px-4 py-2 text-sm font-medium text-white bg-[var(--color-accent)] rounded-md hover:opacity-90 transition-opacity disabled:opacity-50 disabled:cursor-not-allowed"
          onclick={handleMerge}
          disabled={!canMerge}
        >
          {isMerging ? 'Merging...' : 'Merge'}
        </button>
      </div>
    </div>
  </div>
{/if}
