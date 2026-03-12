<script lang="ts">
  import { selectedFile, selectedFileDiff, clearSelectedFile } from '$lib/stores/repo';
  import type { DiffHunk } from '$lib/tauri/api';

  function handleClose() {
    clearSelectedFile();
  }

  function formatHunkHeader(hunk: DiffHunk): string {
    return `@@ -${hunk.old_start},${hunk.old_lines} +${hunk.new_start},${hunk.new_lines} @@`;
  }

  function getLineClass(line: string): string {
    if (line.startsWith('-')) return 'line-deleted';
    if (line.startsWith('+')) return 'line-added';
    return '';
  }
</script>

<div class="diff-viewer">
  <div class="header">
    <h3 class="file-path">{$selectedFile || 'No file selected'}</h3>
    <button class="close-btn" onclick={handleClose} title="Close">
      <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <line x1="18" y1="6" x2="6" y2="18"></line>
        <line x1="6" y1="6" x2="18" y2="18"></line>
      </svg>
    </button>
  </div>

  <div class="diff-content">
    {#if $selectedFileDiff && $selectedFileDiff.hunks.length > 0}
      {#each $selectedFileDiff.hunks as hunk}
        <div class="hunk">
          <div class="hunk-header">{formatHunkHeader(hunk)}</div>
          <div class="hunk-content">
            {#each hunk.content.split('\n') as line}
              <div class="diff-line {getLineClass(line)}">{line}</div>
            {/each}
          </div>
        </div>
      {/each}
    {:else}
      <p class="empty">No changes</p>
    {/if}
  </div>
</div>

<style>
  .diff-viewer {
    background: var(--bg-secondary, #f5f5f5);
    border-radius: 8px;
    padding: 12px;
    display: flex;
    flex-direction: column;
    max-height: 400px;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
    padding-bottom: 8px;
    border-bottom: 1px solid var(--border-color, #ddd);
  }

  .file-path {
    margin: 0;
    font-size: 14px;
    font-weight: 600;
    color: var(--text-primary, #333);
    font-family: monospace;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .close-btn {
    background: none;
    border: none;
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
    color: var(--text-secondary, #666);
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .close-btn:hover {
    background: var(--bg-hover, #e0e0e0);
    color: var(--text-primary, #333);
  }

  .diff-content {
    flex: 1;
    overflow-y: auto;
    font-family: monospace;
    font-size: 12px;
    background: var(--bg-primary, #fff);
    border-radius: 4px;
    padding: 8px;
  }

  .hunk {
    margin-bottom: 8px;
  }

  .hunk-header {
    background: var(--bg-secondary, #e8e8e8);
    padding: 4px 8px;
    border-radius: 4px;
    color: var(--text-secondary, #666);
    font-size: 11px;
    margin-bottom: 4px;
  }

  .hunk-content {
    display: flex;
    flex-direction: column;
  }

  .diff-line {
    padding: 1px 4px;
    white-space: pre;
    min-height: 18px;
    line-height: 18px;
  }

  .line-deleted {
    background: #fee2e1;
    color: #ef4444;
  }

  .line-added {
    background: #dcfce7;
    color: #22c55e;
  }

  .empty {
    text-align: center;
    color: var(--text-secondary, #888);
    font-size: 13px;
    padding: 20px;
  }
</style>
