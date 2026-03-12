<script lang="ts">
  import { repoPath, openRepository, workingCopyDiff, refreshDiff, repoInfo, selectFileForDiff } from '$lib/stores/repo';
  import BranchList from '$lib/components/BranchList.svelte';
  import BranchSelector from '$lib/components/BranchSelector.svelte';
  import ChangedFiles from '$lib/components/ChangedFiles.svelte';
  import DiffViewer from '$lib/components/DiffViewer.svelte';

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

<div class="app">
  <header class="toolbar">
    <div class="folder-input">
      <input 
        type="text" 
        placeholder="Enter repository path..." 
        bind:value={path}
        onkeydown={handleKeydown}
      />
      <button onclick={handleOpenRepo} disabled={isOpening || !path.trim()}>
        {isOpening ? 'Opening...' : 'Open'}
      </button>
    </div>
    {#if openError}
      <div class="error">{openError}</div>
    {/if}
  </header>

  {#if hasRepo}
    <div class="workspace">
      <aside class="sidebar">
        <div class="branch-panel">
          <BranchSelector />
        </div>
        <div class="branch-list-panel">
          <BranchList />
        </div>
      </aside>

      <main class="content">
        <div class="files-panel">
          <ChangedFiles />
        </div>
        <div class="diff-panel">
          <DiffViewer />
        </div>
      </main>
    </div>
  {:else}
    <div class="welcome">
      <h2>Welcome to Jujuwe</h2>
      <p>Enter a repository path above to get started</p>
    </div>
  {/if}
</div>

<style>
  :global(*) {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
  }

  :global(body) {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
    font-size: 14px;
    line-height: 1.5;
    color: #333;
    background: #fafafa;
  }

  .app {
    display: flex;
    flex-direction: column;
    height: 100vh;
    overflow: hidden;
  }

  .toolbar {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
    background: #fff;
    border-bottom: 1px solid #e0e0e0;
  }

  .folder-input {
    display: flex;
    gap: 8px;
    flex: 1;
    max-width: 600px;
  }

  .folder-input input {
    flex: 1;
    padding: 8px 12px;
    border: 1px solid #ddd;
    border-radius: 6px;
    font-size: 14px;
    outline: none;
    transition: border-color 0.15s;
  }

  .folder-input input:focus {
    border-color: #0066cc;
  }

  .folder-input button {
    padding: 8px 16px;
    background: #0066cc;
    color: #fff;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-size: 14px;
    font-weight: 500;
    transition: background-color 0.15s;
  }

  .folder-input button:hover:not(:disabled) {
    background: #0055aa;
  }

  .folder-input button:disabled {
    background: #ccc;
    cursor: not-allowed;
  }

  .error {
    color: #dc2626;
    font-size: 13px;
    padding: 4px 8px;
    background: #fee2e1;
    border-radius: 4px;
  }

  .workspace {
    display: flex;
    flex: 1;
    overflow: hidden;
  }

  .sidebar {
    width: 280px;
    display: flex;
    flex-direction: column;
    gap: 12px;
    padding: 12px;
    background: #f5f5f5;
    border-right: 1px solid #e0e0e0;
    overflow-y: auto;
  }

  .branch-panel {
    padding: 8px;
    background: #fff;
    border-radius: 8px;
    border: 1px solid #e0e0e0;
  }

  .branch-list-panel {
    flex: 1;
    overflow-y: auto;
  }

  .content {
    flex: 1;
    display: flex;
    gap: 12px;
    padding: 12px;
    overflow: hidden;
  }

  .files-panel {
    width: 320px;
    flex-shrink: 0;
    overflow-y: auto;
  }

  .diff-panel {
    flex: 1;
    overflow-y: auto;
  }

  .welcome {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: #666;
  }

  .welcome h2 {
    margin-bottom: 8px;
    color: #333;
  }
</style>
