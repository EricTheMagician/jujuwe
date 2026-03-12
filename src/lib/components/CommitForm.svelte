<script lang="ts">
  import { repoPath, stagedFiles, isLoading, error, createNewCommit, clearStagedFiles } from '$lib/stores/repo';

  let commitMessage = '';
  let showSuccess = false;

  $: stagedCount = $stagedFiles.size;

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

<div class="commit-form">
  <div class="header">
    <h3>Create Commit</h3>
    {#if stagedCount > 0}
      <span class="staged-count">{stagedCount} file{stagedCount !== 1 ? 's' : ''} staged</span>
    {/if}
  </div>

  {#if showSuccess}
    <div class="success-message">
      Commit created successfully!
    </div>
  {/if}

  {#if $error}
    <div class="error-message">
      {$error}
    </div>
  {/if}

  <textarea
    bind:value={commitMessage}
    placeholder="Enter commit message..."
    rows="4"
    disabled={$isLoading}
  ></textarea>

  <button
    class="commit-btn"
    onclick={handleCreateCommit}
    disabled={$isLoading || !commitMessage.trim() || stagedCount === 0}
  >
    {$isLoading ? 'Creating...' : 'Create Commit'}
  </button>
</div>

<style>
  .commit-form {
    background: var(--bg-secondary, #f5f5f5);
    border-radius: 8px;
    padding: 12px;
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

  .staged-count {
    font-size: 12px;
    color: var(--accent-color, #0066cc);
    padding: 4px 8px;
    background: var(--accent-bg, #e3efff);
    border-radius: 4px;
  }

  .success-message {
    font-size: 12px;
    color: #166534;
    padding: 8px;
    background: #dcfce7;
    border-radius: 4px;
    margin-bottom: 8px;
  }

  .error-message {
    font-size: 12px;
    color: #991b1b;
    padding: 8px;
    background: #fee2e1;
    border-radius: 4px;
    margin-bottom: 8px;
  }

  textarea {
    width: 100%;
    padding: 8px;
    border: 1px solid var(--border-color, #ddd);
    border-radius: 4px;
    font-family: inherit;
    font-size: 13px;
    resize: vertical;
    box-sizing: border-box;
    background: var(--bg-primary, #fff);
    color: var(--text-primary, #333);
  }

  textarea:focus {
    outline: none;
    border-color: var(--accent-color, #0066cc);
  }

  textarea:disabled {
    background: var(--bg-tertiary, #eaeaea);
    cursor: not-allowed;
  }

  .commit-btn {
    margin-top: 8px;
    width: 100%;
    padding: 8px 16px;
    background: var(--accent-color, #0066cc);
    color: white;
    border: none;
    border-radius: 4px;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.2s;
  }

  .commit-btn:hover:not(:disabled) {
    background: var(--accent-hover, #0052a3);
  }

  .commit-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
