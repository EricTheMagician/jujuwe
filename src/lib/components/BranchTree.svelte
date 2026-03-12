<script lang="ts">
  import { branches, commits } from '$lib/stores/repo';
  import type { BranchInfo, CommitInfo } from '$lib/tauri/api';

  interface BranchNode {
    name: string;
    commitId: string | null;
    commit?: CommitInfo;
    children: BranchNode[];
  }

  let treeData = $derived.by(() => {
    const nodes: BranchNode[] = [];
    const commitMap = new Map<string, CommitInfo>();
    
    for (const commit of $commits) {
      commitMap.set(commit.commit_id, commit);
    }
    
    for (const branch of $branches) {
      const node: BranchNode = {
        name: branch.name,
        commitId: branch.commit_id,
        commit: branch.commit_id ? commitMap.get(branch.commit_id) : undefined,
        children: []
      };
      nodes.push(node);
    }
    
    return nodes;
  });

  function formatTimestamp(timestamp: number): string {
    const date = new Date(timestamp * 1000);
    const now = new Date();
    const diff = now.getTime() - date.getTime();
    const days = Math.floor(diff / (1000 * 60 * 60 * 24));
    
    if (days === 0) {
      return 'Today';
    } else if (days === 1) {
      return 'Yesterday';
    } else if (days < 7) {
      return `${days} days ago`;
    } else {
      return date.toLocaleDateString();
    }
  }
</script>

<div class="p-3 bg-[var(--color-bg-secondary)] rounded-lg min-w-[280px]">
  <div class="mb-3">
    <h3 class="m-0 text-sm font-semibold text-[var(--color-text-primary)]">Branch Tree</h3>
  </div>
  
  <div class="flex flex-col gap-1">
    {#each treeData as node, index}
      <div class="branch-node">
        <div class="flex items-start gap-2 p-2 rounded-md transition-colors hover:bg-[var(--color-bg-hover)]">
          <div class="text-[var(--color-accent)] flex-shrink-0 pt-0.5">
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="6" y1="3" x2="6" y2="15"></line>
              <circle cx="18" cy="6" r="3"></circle>
              <circle cx="6" cy="18" r="3"></circle>
              <path d="M18 9a9 9 0 0 1-9 9"></path>
            </svg>
          </div>
          <div class="flex-1 min-w-0">
            <div class="font-medium text-[var(--color-text-primary)] text-[13px]">{node.name}</div>
            {#if node.commit}
              <div class="flex flex-wrap gap-2 mt-1 text-xs">
                <span class="font-mono text-[var(--color-text-secondary)] bg-[var(--color-bg-tertiary)] px-1 rounded">{node.commitId?.slice(0, 7)}</span>
                <span class="text-[var(--color-text-secondary)] overflow-hidden text-ellipsis whitespace-nowrap max-w-[120px]">{node.commit.description}</span>
                <span class="text-[var(--color-text-muted)] ml-auto">{formatTimestamp(node.commit.timestamp)}</span>
              </div>
            {/if}
          </div>
        </div>
      </div>
    {/each}
    
    {#if treeData.length === 0}
      <p class="text-center text-[var(--color-text-muted)] text-sm py-5">No branches to display</p>
    {/if}
  </div>
</div>
