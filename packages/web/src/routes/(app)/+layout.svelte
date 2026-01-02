<script lang="ts">
	import { page } from '$app/stores';
	import { auth } from '$lib/stores/auth.svelte';
	import { pagesStore } from '$lib/stores/pages.svelte';
	import Sidebar from '$lib/components/layout/Sidebar.svelte';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	let { children } = $props();

	let workspaceId = $derived($page.params.workspace ?? '');
	let pageId = $derived($page.params.page);

	onMount(() => {
		// Redirect to login if not authenticated
		if (!auth.isAuthenticated) {
			goto('/login');
			return;
		}
	});

	// Load page tree when workspace changes
	$effect(() => {
		if (workspaceId && auth.isAuthenticated) {
			pagesStore.loadPageTree(workspaceId);
		}
	});
</script>

{#if auth.isAuthenticated}
	<div class="app-layout">
		<Sidebar {workspaceId} currentPageId={pageId} />
		<main class="main-content">
			{@render children()}
		</main>
	</div>
{:else}
	<div class="loading">Loading...</div>
{/if}

<style>
	.app-layout {
		display: flex;
		height: 100vh;
		overflow: hidden;
	}

	.main-content {
		flex: 1;
		overflow-y: auto;
		background-color: var(--color-bg);
	}

	.loading {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 100vh;
		color: var(--color-text-muted);
	}
</style>
