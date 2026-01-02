<script lang="ts">
	import { page } from '$app/stores';
	import { pagesStore } from '$lib/stores/pages.svelte';
	import { Editor } from '$lib/editor';
	import PageHeader from '$lib/components/layout/PageHeader.svelte';
	import { onMount } from 'svelte';

	let workspaceId = $derived($page.params.workspace ?? '');
	let pageId = $derived($page.params.page ?? '');

	let content = $state('');
	let saveTimeout: ReturnType<typeof setTimeout> | null = null;

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
			// Content would come from CRDT in the future
			content = '';
		} catch (e) {
			console.error('Failed to load page:', e);
		}
	}

	function handleContentUpdate(newContent: string) {
		content = newContent;

		// Debounced save
		if (saveTimeout) {
			clearTimeout(saveTimeout);
		}
		saveTimeout = setTimeout(() => {
			// TODO: Save to CRDT/backend
			console.log('Content updated:', newContent.slice(0, 100));
		}, 1000);
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

	onMount(() => {
		return () => {
			if (saveTimeout) {
				clearTimeout(saveTimeout);
			}
		};
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
			<Editor
				{content}
				placeholder="Press '/' for commands, or start typing..."
				onUpdate={handleContentUpdate}
				autofocus
			/>
		</div>
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
