<script lang="ts">
	import { page } from '$app/stores';
	import { pagesStore } from '$lib/stores/pages.svelte';
	import { auth } from '$lib/stores/auth.svelte';
	import { CollaborativeEditor } from '$lib/editor';
	import PageHeader from '$lib/components/layout/PageHeader.svelte';
	import { crdtStore } from '$lib/crdt';

	let workspaceId = $derived($page.params.workspace ?? '');
	let pageId = $derived($page.params.page ?? '');

	// Load page when pageId changes
	$effect(() => {
		if (pageId) {
			loadPage(pageId);
		}
	});

	async function loadPage(id: string) {
		try {
			await pagesStore.loadPage(id);
			await pagesStore.loadBreadcrumbs(id);
		} catch (e) {
			console.error('Failed to load page:', e);
		}
	}

	async function handleTitleChange(newTitle: string) {
		if (pagesStore.currentPage) {
			await pagesStore.updatePage(pagesStore.currentPage.id, { title: newTitle });
			// Refresh page tree to show new title
			await pagesStore.loadPageTree(workspaceId);
		}
	}

	async function handleIconChange(newIcon: string | null) {
		if (pagesStore.currentPage) {
			await pagesStore.updatePage(pagesStore.currentPage.id, { icon: newIcon ?? undefined });
		}
	}

	function handleEditorUpdate() {
		// Content is auto-saved via CRDT/IndexedDB
		console.log('Editor updated, auto-saved to IndexedDB');
	}

	// Format last saved time
	let lastSavedText = $derived(() => {
		if (!crdtStore.lastSaved) return '';
		const now = new Date();
		const diff = now.getTime() - crdtStore.lastSaved.getTime();
		if (diff < 60000) return 'Saved just now';
		if (diff < 3600000) return `Saved ${Math.floor(diff / 60000)} min ago`;
		return `Saved at ${crdtStore.lastSaved.toLocaleTimeString()}`;
	});
</script>

<svelte:head>
	<title>{pagesStore.currentPage?.title ?? 'Loading...'} | Notion Killer</title>
</svelte:head>

<div class="page-editor">
	{#if pagesStore.isLoading}
		<div class="loading">
			<div class="loading-spinner"></div>
		</div>
	{:else if pagesStore.currentPage}
		<PageHeader
			{workspaceId}
			{pageId}
			title={pagesStore.currentPage.title}
			icon={pagesStore.currentPage.icon}
			coverUrl={pagesStore.currentPage.cover_url}
			onTitleChange={handleTitleChange}
			onIconChange={handleIconChange}
		/>

		<div class="editor-container">
			{#key pageId}
				<CollaborativeEditor
					{pageId}
					placeholder="Press '/' for commands, or start typing..."
					userName={auth.user?.name ?? 'Anonymous'}
					onUpdate={handleEditorUpdate}
					autofocus
				/>
			{/key}
		</div>

		{#if lastSavedText()}
			<div class="save-status">
				{lastSavedText()}
			</div>
		{/if}
	{:else if pagesStore.error}
		<div class="error">
			<h2>Page not found</h2>
			<p>{pagesStore.error}</p>
		</div>
	{/if}
</div>

<style>
	.page-editor {
		min-height: 100vh;
		position: relative;
	}

	.editor-container {
		max-width: 900px;
		margin: 0 auto;
		padding: 0 4rem 4rem;
	}

	.loading {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 100vh;
	}

	.loading-spinner {
		width: 32px;
		height: 32px;
		border: 3px solid var(--color-border);
		border-top-color: var(--color-text);
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.save-status {
		position: fixed;
		bottom: 1rem;
		right: 1rem;
		padding: 0.5rem 0.75rem;
		background: var(--color-bg);
		border: 1px solid var(--color-border);
		border-radius: 0.375rem;
		font-size: 0.75rem;
		color: var(--color-text-muted);
	}

	.error {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100vh;
		text-align: center;
	}

	.error h2 {
		font-size: 1.5rem;
		font-weight: 600;
		margin-bottom: 0.5rem;
	}

	.error p {
		color: var(--color-text-muted);
	}
</style>
