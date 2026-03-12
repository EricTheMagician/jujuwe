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

<div class="branch-tree">
  <div class="header">
    <h3>Branch Tree</h3>
  </div>
  
  <div class="tree-container">
    {#each treeData as node, index}
      <div class="branch-node">
        <div class="node-line" style="--index: {index}">
          <div class="node-marker">
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="6" y1="3" x2="6" y2="15"></line>
              <circle cx="18" cy="6" r="3"></circle>
              <circle cx="6" cy="18" r="3"></circle>
              <path d="M18 9a9 9 0 0 1-9 9"></path>
            </svg>
          </div>
          <div class="node-content">
            <div class="branch-name">{node.name}</div>
            {#if node.commit}
              <div class="commit-info">
                <span class="commit-id">{node.commitId?.slice(0, 7)}</span>
                <span class="commit-desc">{node.commit.description}</span>
                <span class="timestamp">{formatTimestamp(node.commit.timestamp)}</span>
              </div>
            {/if}
          </div>
        </div>
      </div>
    {/each}
    
    {#if treeData.length === 0}
      <p class="empty">No branches to display</p>
    {/if}
  </div>
</div>

<style>
  .branch-tree {
    background: var(--bg-secondary, #f5f5f5);
    border-radius: 8px;
    padding: 12px;
    min-width: 280px;
  }

  .header {
    margin-bottom: 12px;
  }

  .header h3 {
    margin: 0;
    font-size: 14px;
    font-weight: 600;
    color: var(--text-primary, #333);
  }

  .tree-container {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .branch-node {
    position: relative;
  }

  .node-line {
    display: flex;
    align-items: flex-start;
    gap: 8px;
    padding: 8px;
    border-radius: 6px;
    transition: background-color 0.15s;
  }

  .node-line:hover {
    background: var(--bg-hover, #e8e8e8);
  }

  .node-marker {
    flex-shrink: 0;
    color: var(--accent-color, #0066cc);
    padding-top: 2px;
  }

  .node-content {
    flex: 1;
    min-width: 0;
  }

  .branch-name {
    font-weight: 500;
    color: var(--text-primary, #333);
    font-size: 13px;
  }

  .commit-info {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    margin-top: 4px;
    font-size: 12px;
  }

  .commit-id {
    font-family: monospace;
    color: var(--text-secondary, #666);
    background: var(--bg-tertiary, #eaeaea);
    padding: 1px 4px;
    border-radius: 3px;
  }

  .commit-desc {
    color: var(--text-secondary, #666);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 120px;
  }

  .timestamp {
    color: var(--text-secondary, #888);
    margin-left: auto;
  }

  .empty {
    text-align: center;
    color: var(--text-secondary, #888);
    font-size: 13px;
    padding: 20px;
  }
</style>
