<script lang="ts">
  import { repoPath, openRepository, workingCopyDiff, refreshDiff, repoInfo, selectFileForDiff } from '$lib/stores/repo';
  import CommitForm from '$lib/components/CommitForm.svelte';
  import BranchList from '$lib/components/BranchList.svelte';
  import BranchSelector from '$lib/components/BranchSelector.svelte';
  import ChangedFiles from '$lib/components/ChangedFiles.svelte';
  import DiffViewer from '$lib/components/DiffViewer.svelte';
  import CommitHistory from '$lib/components/CommitHistory.svelte';

  let path = $state('');
  let isOpening = $state(false);
  let openError = $state<string | null>(null);

  async function handleOpenRepo() {
    if (!path.trim()) return;
    
    isOpening = true;
    openError = null;
    
    try {
      await openRepository(path);
      await refreshDiff(path);
    } catch (e) {
      openError = e instanceof Error ? e.message : String(e);
    } finally {
      isOpening = false;
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Enter') {
      handleOpenRepo();
    }
  }

  let hasRepo = $derived($repoPath !== null);
</script>

<div class="flex flex-col h-screen overflow-hidden">
  <header class="flex items-center gap-3 px-4 py-3 bg-white border-b border-gray-200">
    <div class="flex gap-2 flex-1 max-w-[600px]">
      <input 
        type="text" 
        placeholder="Enter repository path..." 
        bind:value={path}
        onkeydown={handleKeydown}
        class="flex-1 px-3 py-2 text-sm border border-gray-300 rounded-md outline-none focus:border-[var(--color-accent)] transition-colors"
      />
      <button 
        onclick={handleOpenRepo} 
        disabled={isOpening || !path.trim()}
        class="px-4 py-2 text-sm font-medium text-white bg-[var(--color-accent)] rounded-md hover:bg-[var(--color-accent-hover)] disabled:bg-gray-300 disabled:cursor-not-allowed transition-colors"
      >
        {isOpening ? 'Opening...' : 'Open'}
      </button>
    </div>
    {#if openError}
      <div class="text-sm text-[var(--color-removed-text)] px-2 py-1 bg-[var(--color-removed-bg)] rounded">{openError}</div>
    {/if}
  </header>

  {#if hasRepo}
    <div class="flex flex-1 overflow-hidden">
      <aside class="w-[280px] flex flex-col gap-3 p-3 bg-[var(--color-bg-secondary)] border-r border-gray-200 overflow-y-auto">
        <div class="p-2 bg-white rounded-lg border border-gray-200">
          <BranchSelector />
        </div>
        <div class="flex-1 overflow-y-auto">
          <BranchList />
        </div>
      </aside>

      <main class="flex-1 flex gap-3 p-3 overflow-hidden">
        <div class="w-[320px] flex-shrink-0 overflow-y-auto">
          <ChangedFiles />
        </div>
        <div class="flex-1 overflow-y-auto">
          <DiffViewer />
        </div>
      </main>
    </div>
    
    <div class="flex flex-col">
      <div class="p-3 bg-[var(--color-bg-secondary)] border-b border-gray-200">
        <CommitForm />
      </div>
      <div class="p-3 bg-[var(--color-bg-secondary)] border-t border-gray-200 max-h-[200px] overflow-y-auto">
        <CommitHistory />
      </div>
    </div>
  {:else}
    <div class="flex-1 flex flex-col items-center justify-center text-[var(--color-text-secondary)]">
      <h2 class="mb-2 text-xl font-semibold text-[var(--color-text-primary)]">Welcome to Jujuwe</h2>
      <p>Enter a repository path above to get started</p>
    </div>
  {/if}
</div>
