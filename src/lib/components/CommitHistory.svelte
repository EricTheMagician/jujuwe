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

<div class="p-3 bg-[var(--color-bg-secondary)] rounded-lg min-w-[300px]">
  <div class="flex justify-between items-center mb-3">
    <h3 class="m-0 text-sm font-semibold text-[var(--color-text-primary)]">Commit History</h3>
    <button class="p-1 rounded bg-transparent border-none cursor-pointer text-[var(--color-text-secondary)] flex items-center justify-center hover:bg-[var(--color-bg-hover)] hover:text-[var(--color-text-primary)] disabled:opacity-50 disabled:cursor-not-allowed" onclick={handleRefresh} title="Refresh" disabled={$isLoading}>
      <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"/>
        <path d="M21 3v5h-5"/>
      </svg>
    </button>
  </div>

  <div class="flex flex-col gap-2 max-h-[400px] overflow-y-auto">
    {#if $commits.length > 0}
      {#each $commits as commit}
        <div class="p-2.5 bg-white rounded-md border border-gray-200 hover:border-[var(--color-accent)] transition-colors">
          <div class="flex gap-2 items-center mb-1">
            <span class="font-mono text-xs text-[var(--color-accent)] font-semibold" title={commit.commit_id}>{truncateId(commit.commit_id)}</span>
            <span class="font-mono text-[11px] text-[var(--color-text-secondary)]">#{truncateId(commit.change_id, 6)}</span>
          </div>
          <div class="text-sm text-[var(--color-text-primary)] mb-1.5 leading-tight break-words">{commit.description}</div>
          <div class="flex justify-between items-center text-[11px] text-[var(--color-text-secondary)]">
            <span class="font-medium">{commit.author}</span>
            <span>{formatTimestamp(commit.timestamp)}</span>
          </div>
        </div>
      {/each}
    {:else}
      <p class="text-center text-[var(--color-text-muted)] text-sm py-5">No commits yet</p>
    {/if}
  </div>
</div>
