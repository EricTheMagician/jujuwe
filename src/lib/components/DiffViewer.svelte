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
    if (line.startsWith('-')) return 'bg-[var(--color-removed-bg)] text-[var(--color-removed)]';
    if (line.startsWith('+')) return 'bg-[var(--color-added-bg)] text-[var(--color-added)]';
    return '';
  }
</script>

<div class="p-3 bg-[var(--color-bg-secondary)] rounded-lg flex flex-col max-h-[400px]">
  <div class="flex justify-between items-center mb-3 pb-2 border-b border-gray-200">
    <h3 class="m-0 text-sm font-semibold text-[var(--color-text-primary)] font-mono overflow-hidden text-ellipsis whitespace-nowrap">{$selectedFile || 'No file selected'}</h3>
    <button class="p-1 rounded bg-transparent border-none cursor-pointer text-[var(--color-text-secondary)] flex items-center justify-center hover:bg-[var(--color-bg-hover)] hover:text-[var(--color-text-primary)]" onclick={handleClose} title="Close">
      <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <line x1="18" y1="6" x2="6" y2="18"></line>
        <line x1="6" y1="6" x2="18" y2="18"></line>
      </svg>
    </button>
  </div>

  <div class="flex-1 overflow-y-auto font-mono text-xs bg-white rounded p-2">
    {#if $selectedFileDiff && $selectedFileDiff.hunks.length > 0}
      {#each $selectedFileDiff.hunks as hunk}
        <div class="mb-2">
          <div class="bg-gray-200 px-2 py-1 rounded text-gray-600 text-[11px] mb-1">{formatHunkHeader(hunk)}</div>
          <div class="flex flex-col">
            {#each hunk.content.split('\n') as line}
              <div class="px-1 whitespace-pre min-h-[18px] leading-[18px] {getLineClass(line)}">{line}</div>
            {/each}
          </div>
        </div>
      {/each}
    {:else}
      <p class="text-center text-[var(--color-text-muted)] text-sm py-5">No changes</p>
    {/if}
  </div>
</div>
