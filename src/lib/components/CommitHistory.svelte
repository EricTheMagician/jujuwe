<script lang="ts">
  import { commits, repoPath, refreshCommits, isLoading } from '$lib/stores/repo';

  async function handleRefresh() {
    const path = $repoPath;
    if (path) {
      await refreshCommits(path);
    }
  }

  function formatTimestamp(ts: number): string {
    const date = new Date(ts * 1000);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));

    if (diffDays === 0) {
      const hours = date.getHours().toString().padStart(2, '0');
      const minutes = date.getMinutes().toString().padStart(2, '0');
      return `Today at ${hours}:${minutes}`;
    } else if (diffDays === 1) {
      const hours = date.getHours().toString().padStart(2, '0');
      const minutes = date.getMinutes().toString().padStart(2, '0');
      return `Yesterday at ${hours}:${minutes}`;
    } else if (diffDays < 7) {
      return `${diffDays} days ago`;
    } else {
      const year = date.getFullYear();
      const month = (date.getMonth() + 1).toString().padStart(2, '0');
      const day = date.getDate().toString().padStart(2, '0');
      return `${year}-${month}-${day}`;
    }
  }

  function truncateId(id: string, len: number = 8): string {
    return id.substring(0, len);
  }
</script>

<div class="commit-history">
  <div class="header">
    <h3>Commit History</h3>
    <button class="refresh-btn" onclick={handleRefresh} title="Refresh" disabled={$isLoading}>
      <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"/>
        <path d="M21 3v5h-5"/>
      </svg>
    </button>
  </div>

  <div class="commit-list">
    {#if $commits.length > 0}
      {#each $commits as commit}
        <div class="commit-item">
          <div class="commit-header">
            <span class="commit-id" title={commit.commit_id}>{truncateId(commit.commit_id)}</span>
            <span class="change-id" title={commit.change_id}>#{truncateId(commit.change_id, 6)}</span>
          </div>
          <div class="commit-description">{commit.description}</div>
          <div class="commit-meta">
            <span class="commit-author">{commit.author}</span>
            <span class="commit-timestamp">{formatTimestamp(commit.timestamp)}</span>
          </div>
        </div>
      {/each}
    {:else}
      <p class="empty">No commits yet</p>
    {/if}
  </div>
</div>

<style>
  .commit-history {
    background: var(--bg-secondary, #f5f5f5);
    border-radius: 8px;
    padding: 12px;
    min-width: 300px;
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

  .refresh-btn:hover:not(:disabled) {
    background: var(--bg-hover, #e0e0e0);
    color: var(--text-primary, #333);
  }

  .refresh-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .commit-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
    max-height: 400px;
    overflow-y: auto;
  }

  .commit-item {
    padding: 10px;
    background: var(--bg-primary, #fff);
    border-radius: 6px;
    border: 1px solid var(--border-color, #e0e0e0);
  }

  .commit-item:hover {
    border-color: var(--accent-color, #0066cc);
  }

  .commit-header {
    display: flex;
    gap: 8px;
    align-items: center;
    margin-bottom: 4px;
  }

  .commit-id {
    font-family: monospace;
    font-size: 12px;
    color: var(--accent-color, #0066cc);
    font-weight: 600;
  }

  .change-id {
    font-family: monospace;
    font-size: 11px;
    color: var(--text-secondary, #666);
  }

  .commit-description {
    font-size: 13px;
    color: var(--text-primary, #333);
    margin-bottom: 6px;
    line-height: 1.3;
    word-break: break-word;
  }

  .commit-meta {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 11px;
    color: var(--text-secondary, #888);
  }

  .commit-author {
    font-weight: 500;
  }

  .commit-timestamp {
    color: var(--text-secondary, #888);
  }

  .empty {
    text-align: center;
    color: var(--text-secondary, #888);
    font-size: 13px;
    padding: 20px;
  }
</style>
