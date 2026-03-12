<script lang="ts">
  import { repoPath, stagedFiles, isLoading, error, createNewCommit, clearStagedFiles } from '$lib/stores/repo';

  let commitMessage = '';
  let showSuccess = $state(false);

  let stagedCount = $derived($stagedFiles.size);
  let canSubmit = $derived(commitMessage.trim() && stagedCount > 0 && !$isLoading);

  function handleKeydown(event: KeyboardEvent) {
    if ((event.ctrlKey || event.metaKey) && event.key === 'Enter') {
      event.preventDefault();
      if (canSubmit) {
        handleCreateCommit();
      }
    }
  }

  async function handleCreateCommit() {
    const path = $repoPath;
    if (!path || !commitMessage.trim() || stagedCount === 0) {
      return;
    }

    const result = await createNewCommit(path, commitMessage);
    if (result) {
      commitMessage = '';
      clearStagedFiles();
      showSuccess = true;
      setTimeout(() => {
        showSuccess = false;
      }, 3000);
    }
  }
</script>

<div class="p-3 bg-[var(--color-bg-secondary)] rounded-lg">
  <div class="flex justify-between items-center mb-3">
    <h3 class="m-0 text-sm font-semibold text-[var(--color-text-primary)]">Create Commit</h3>
    {#if stagedCount > 0}
      <span class="text-xs text-[var(--color-accent)] px-2 py-1 bg-[var(--color-accent-light)] rounded">{stagedCount} file{stagedCount !== 1 ? 's' : ''} staged</span>
    {/if}
  </div>

  {#if showSuccess}
    <div class="text-xs text-[var(--color-added-text)] px-2 py-2 bg-[var(--color-added-bg)] rounded mb-2">
      Commit created successfully!
    </div>
  {/if}

  {#if $error}
    <div class="text-xs text-[var(--color-removed-text)] px-2 py-2 bg-[var(--color-removed-bg)] rounded mb-2">
      {$error}
    </div>
  {/if}

  <textarea
    bind:value={commitMessage}
    onkeydown={handleKeydown}
    placeholder="Enter commit message..."
    rows="4"
    disabled={$isLoading}
    class="w-full px-2 py-2 border border-gray-200 rounded text-sm resize-y box-border bg-white text-[var(--color-text-primary)] focus:outline-none focus:border-[var(--color-accent)] disabled:bg-[var(--color-bg-tertiary)] disabled:cursor-not-allowed"
  ></textarea>

  <button
    class="mt-2 w-full px-4 py-2 bg-[var(--color-accent)] text-white border-none rounded text-sm font-medium cursor-pointer hover:bg-[var(--color-accent-hover)] transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
    onclick={handleCreateCommit}
    disabled={$isLoading || !commitMessage.trim() || stagedCount === 0}
  >
    {$isLoading ? 'Creating...' : 'Create Commit'}
  </button>
</div>
