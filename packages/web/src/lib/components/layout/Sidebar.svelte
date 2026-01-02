<script lang="ts">
	import { pagesStore, type PageTreeItem } from '$lib/stores/pages.svelte';
	import { goto } from '$app/navigation';

	interface Props {
		workspaceId: string;
		currentPageId?: string;
	}

	let { workspaceId, currentPageId }: Props = $props();

	let expandedItems = $state<Set<string>>(new Set());

	function toggleExpanded(id: string) {
		const newSet = new Set(expandedItems);
		if (newSet.has(id)) {
			newSet.delete(id);
		} else {
			newSet.add(id);
		}
		expandedItems = newSet;
	}

	async function handleCreatePage(parentId?: string) {
		const page = await pagesStore.createPage({
			workspace_id: workspaceId,
			parent_id: parentId,
			title: 'Untitled'
		});
		goto(`/${workspaceId}/${page.id}`);
	}

	function handlePageClick(pageId: string) {
		goto(`/${workspaceId}/${pageId}`);
	}
</script>

<aside class="sidebar">
	<div class="sidebar-header">
		<h2 class="workspace-name">Workspace</h2>
		<button class="btn-icon" onclick={() => handleCreatePage()} title="New page">
			<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<line x1="12" y1="5" x2="12" y2="19"></line>
				<line x1="5" y1="12" x2="19" y2="12"></line>
			</svg>
		</button>
	</div>

	<nav class="page-tree">
		{#each pagesStore.pageTree as item}
			{@render pageTreeItem(item, 0)}
		{/each}
	</nav>
</aside>

{#snippet pageTreeItem(item: PageTreeItem, depth: number)}
	<div class="tree-item" style="--depth: {depth}">
		{#if item.children.length > 0}
			<button
				class="expand-button"
				aria-label="Toggle expand"
				onclick={(e) => {
					e.stopPropagation();
					toggleExpanded(item.id);
				}}
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="12"
					height="12"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					class:rotated={expandedItems.has(item.id)}
				>
					<polyline points="9 18 15 12 9 6"></polyline>
				</svg>
			</button>
		{:else}
			<span class="expand-placeholder"></span>
		{/if}

		<button
			class="tree-item-button"
			class:active={currentPageId === item.id}
			onclick={() => handlePageClick(item.id)}
		>
			<span class="page-icon">
				{#if item.icon}
					{item.icon}
				{:else if item.is_database}
					<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
						<line x1="3" y1="9" x2="21" y2="9"></line>
						<line x1="9" y1="21" x2="9" y2="9"></line>
					</svg>
				{:else}
					<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
						<polyline points="14 2 14 8 20 8"></polyline>
					</svg>
				{/if}
			</span>

			<span class="page-title">{item.title}</span>
		</button>

		<button
			class="add-button"
			aria-label="Add subpage"
			onclick={(e) => {
				e.stopPropagation();
				handleCreatePage(item.id);
			}}
		>
			<svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<line x1="12" y1="5" x2="12" y2="19"></line>
				<line x1="5" y1="12" x2="19" y2="12"></line>
			</svg>
		</button>
	</div>

	{#if item.children.length > 0 && expandedItems.has(item.id)}
		<div class="tree-children">
			{#each item.children as child}
				{@render pageTreeItem(child, depth + 1)}
			{/each}
		</div>
	{/if}
{/snippet}

<style>
	.sidebar {
		width: 260px;
		height: 100vh;
		background-color: var(--color-bg);
		border-right: 1px solid var(--color-border);
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.sidebar-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem;
		border-bottom: 1px solid var(--color-border);
	}

	.workspace-name {
		font-size: 0.875rem;
		font-weight: 600;
		color: var(--color-text);
	}

	.btn-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		border-radius: 0.375rem;
		background: transparent;
		border: none;
		color: var(--color-text-muted);
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-icon:hover {
		background-color: var(--color-border);
		color: var(--color-text);
	}

	.page-tree {
		flex: 1;
		overflow-y: auto;
		padding: 0.5rem;
	}

	.tree-item {
		display: flex;
		align-items: center;
		padding-left: calc(var(--depth) * 1rem);
		position: relative;
	}

	.tree-item-button {
		flex: 1;
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.375rem 0.5rem;
		border-radius: 0.375rem;
		background: transparent;
		border: none;
		color: var(--color-text);
		cursor: pointer;
		transition: all 0.15s;
		text-align: left;
		font-size: 0.875rem;
		min-width: 0;
	}

	.tree-item-button:hover {
		background-color: var(--color-border);
	}

	.tree-item-button.active {
		background-color: var(--color-border);
	}

	.expand-button {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 18px;
		height: 18px;
		background: transparent;
		border: none;
		color: var(--color-text-muted);
		cursor: pointer;
		border-radius: 0.25rem;
		flex-shrink: 0;
	}

	.expand-button:hover {
		background-color: var(--color-border);
	}

	.expand-button svg {
		transition: transform 0.15s;
	}

	.expand-button svg.rotated {
		transform: rotate(90deg);
	}

	.expand-placeholder {
		width: 18px;
		flex-shrink: 0;
	}

	.page-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 18px;
		height: 18px;
		color: var(--color-text-muted);
		flex-shrink: 0;
		font-size: 14px;
	}

	.page-title {
		flex: 1;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.add-button {
		display: none;
		align-items: center;
		justify-content: center;
		width: 20px;
		height: 20px;
		background: transparent;
		border: none;
		color: var(--color-text-muted);
		cursor: pointer;
		border-radius: 0.25rem;
		position: absolute;
		right: 0.5rem;
	}

	.tree-item:hover .add-button {
		display: flex;
	}

	.add-button:hover {
		background-color: var(--color-border);
		color: var(--color-text);
	}

	.tree-children {
		/* Children inherit depth styling */
	}
</style>
