<script lang="ts">
  import { repoPath, openRepository, workingCopyDiff, refreshDiff, repoInfo, selectFileForDiff, stageAllFiles, clearStagedFiles, refreshBranches, refreshCommits } from '$lib/stores/repo';
  import CommitForm from '$lib/components/CommitForm.svelte';
  import BranchList from '$lib/components/BranchList.svelte';
  import BranchSelector from '$lib/components/BranchSelector.svelte';
  import ChangedFiles from '$lib/components/ChangedFiles.svelte';
  import DiffViewer from '$lib/components/DiffViewer.svelte';
  import CommitHistory from '$lib/components/CommitHistory.svelte';
  import MergeDialog from '$lib/components/MergeDialog.svelte';

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
    if (event.key === 'Escape') {
      if (showKeyboardHints) {
        showKeyboardHints = false;
        event.preventDefault();
      }
      return;
    }

    if (event.key === '?' || (event.ctrlKey && event.key === '/')) {
      event.preventDefault();
      showKeyboardHints = !showKeyboardHints;
      return;
    }

    if (event.key === 'Enter' && !event.ctrlKey && !event.metaKey && !event.shiftKey) {
      handleOpenRepo();
      return;
    }

    const isMod = event.ctrlKey || event.metaKey;
    const isShift = event.shiftKey;

    if (!hasRepo) return;

    if (isMod && isShift && event.key === 's') {
      event.preventDefault();
      stageAllFiles();
      return;
    }

    if (isMod && isShift && event.key === 'u') {
      event.preventDefault();
      clearStagedFiles();
      return;
    }

    if (isMod && !isShift && event.key === 'r') {
      event.preventDefault();
      const currentPath = $repoPath;
      if (currentPath) {
        refreshDiff(currentPath);
        refreshBranches(currentPath);
        refreshCommits(currentPath);
      }
      return;
    }
  }

  let hasRepo = $derived($repoPath !== null);
  let showMergeDialog = $state(false);
  let showKeyboardHints = $state(false);

  function closeKeyboardHints() {
    showKeyboardHints = false;
  }
</script>

<svelte:window onkeydown={handleKeydown} />

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
    <button
      onclick={() => showKeyboardHints = true}
      class="px-2.5 py-1.5 text-sm text-[var(--color-text-secondary)] bg-transparent border border-gray-200 rounded hover:bg-gray-100 transition-colors"
      title="Keyboard shortcuts (?)"
    >
      ?
    </button>
  </header>

  {#if hasRepo}
    <div class="flex flex-1 overflow-hidden">
      <aside class="w-[280px] flex flex-col gap-3 p-3 bg-[var(--color-bg-secondary)] border-r border-gray-200 overflow-y-auto">
        <div class="p-2 bg-white rounded-lg border border-gray-200">
          <BranchSelector />
        </div>
        <div class="flex-1 overflow-y-auto">
          <BranchList onMerge={() => showMergeDialog = true} />
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
    <MergeDialog open={showMergeDialog} onClose={() => showMergeDialog = false} />
  {:else}
    <div class="flex-1 flex flex-col items-center justify-center text-[var(--color-text-secondary)]">
      <h2 class="mb-2 text-xl font-semibold text-[var(--color-text-primary)]">Welcome to Jujuwe</h2>
      <p>Enter a repository path above to get started</p>
    </div>
  {/if}

  {#if showKeyboardHints}
    <div 
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
      onclick={closeKeyboardHints}
      onkeydown={(e) => e.key === 'Escape' && closeKeyboardHints()}
      role="dialog"
      tabindex="-1"
    >
      <div 
        class="bg-white rounded-lg shadow-xl w-full max-w-md mx-4 overflow-hidden"
        onclick={(e) => e.stopPropagation()}
        onkeydown={() => {}}
        role="dialog"
        tabindex="-1"
      >
        <div class="flex items-center justify-between px-4 py-3 border-b border-gray-200">
          <h2 class="text-lg font-semibold text-[var(--color-text-primary)]">Keyboard Shortcuts</h2>
          <button 
            onclick={closeKeyboardHints}
            class="p-1 text-[var(--color-text-secondary)] hover:text-[var(--color-text-primary)] transition-colors"
          >
            ✕
          </button>
        </div>
        
        <div class="p-4">
          <div class="mb-4">
            <h3 class="text-xs font-semibold text-[var(--color-text-secondary)] uppercase tracking-wider mb-2">General</h3>
            <div class="space-y-2">
              <div class="flex justify-between items-center">
                <span class="text-sm text-[var(--color-text-primary)]">Show keyboard shortcuts</span>
                <kbd class="px-2 py-1 text-xs bg-gray-100 border border-gray-200 rounded font-mono">?</kbd>
              </div>
              <div class="flex justify-between items-center">
                <span class="text-sm text-[var(--color-text-primary)]">Open repository</span>
                <kbd class="px-2 py-1 text-xs bg-gray-100 border border-gray-200 rounded font-mono">Enter</kbd>
              </div>
              <div class="flex justify-between items-center">
                <span class="text-sm text-[var(--color-text-primary)]">Refresh all data</span>
                <kbd class="px-2 py-1 text-xs bg-gray-100 border border-gray-200 rounded font-mono">Ctrl+R</kbd>
              </div>
            </div>
          </div>
          
          {#if hasRepo}
          <div class="mb-4">
            <h3 class="text-xs font-semibold text-[var(--color-text-secondary)] uppercase tracking-wider mb-2">Files</h3>
            <div class="space-y-2">
              <div class="flex justify-between items-center">
                <span class="text-sm text-[var(--color-text-primary)]">Stage all files</span>
                <kbd class="px-2 py-1 text-xs bg-gray-100 border border-gray-200 rounded font-mono">Ctrl+Shift+S</kbd>
              </div>
              <div class="flex justify-between items-center">
                <span class="text-sm text-[var(--color-text-primary)]">Unstage all files</span>
                <kbd class="px-2 py-1 text-xs bg-gray-100 border border-gray-200 rounded font-mono">Ctrl+Shift+U</kbd>
              </div>
            </div>
          </div>
          
          <div>
            <h3 class="text-xs font-semibold text-[var(--color-text-secondary)] uppercase tracking-wider mb-2">Commit</h3>
            <div class="space-y-2">
              <div class="flex justify-between items-center">
                <span class="text-sm text-[var(--color-text-primary)]">Create commit</span>
                <kbd class="px-2 py-1 text-xs bg-gray-100 border border-gray-200 rounded font-mono">Ctrl+Enter</kbd>
              </div>
            </div>
          </div>
          {/if}
        </div>
        
        <div class="px-4 py-3 bg-gray-50 border-t border-gray-200">
          <p class="text-xs text-[var(--color-text-secondary)]">Press <kbd class="px-1 py-0.5 text-xs bg-gray-200 rounded">Esc</kbd> to close</p>
        </div>
      </div>
    </div>
  {/if}
</div>
