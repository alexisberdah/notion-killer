<script lang="ts">
	import { pagesStore, type BreadcrumbItem } from '$lib/stores/pages.svelte';
	import { goto } from '$app/navigation';

	interface Props {
		workspaceId: string;
		pageId: string;
		title: string;
		icon?: string | null;
		coverUrl?: string | null;
		onTitleChange?: (title: string) => void;
		onIconChange?: (icon: string | null) => void;
	}

	let { workspaceId, pageId, title, icon, coverUrl, onTitleChange, onIconChange }: Props = $props();

	let isEditingTitle = $state(false);
	let titleInput = $state(title);

	function handleBreadcrumbClick(item: BreadcrumbItem) {
		goto(`/${workspaceId}/${item.id}`);
	}

	function handleTitleFocus() {
		isEditingTitle = true;
		titleInput = title;
	}

	function handleTitleBlur() {
		isEditingTitle = false;
		if (titleInput.trim() && titleInput !== title) {
			onTitleChange?.(titleInput.trim());
		}
	}

	function handleTitleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			e.preventDefault();
			(e.target as HTMLInputElement).blur();
		}
		if (e.key === 'Escape') {
			titleInput = title;
			(e.target as HTMLInputElement).blur();
		}
	}
</script>

<header class="page-header">
	{#if coverUrl}
		<div class="cover-image" style="background-image: url({coverUrl})">
			<button class="change-cover-btn">Change cover</button>
		</div>
	{/if}

	<div class="header-content" class:has-cover={!!coverUrl}>
		<!-- Breadcrumbs -->
		<nav class="breadcrumbs">
			{#each pagesStore.breadcrumbs as item, i}
				{#if i > 0}
					<span class="breadcrumb-separator">/</span>
				{/if}
				<button
					class="breadcrumb-item"
					onclick={() => handleBreadcrumbClick(item)}
					disabled={item.id === pageId}
				>
					{#if item.icon}
						<span class="breadcrumb-icon">{item.icon}</span>
					{/if}
					<span>{item.title}</span>
				</button>
			{/each}
		</nav>

		<!-- Page Title with Icon -->
		<div class="title-row">
			<button class="icon-button" title="Add icon">
				{#if icon}
					<span class="page-icon">{icon}</span>
				{:else}
					<span class="add-icon">
						<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<circle cx="12" cy="12" r="10"></circle>
							<line x1="12" y1="8" x2="12" y2="16"></line>
							<line x1="8" y1="12" x2="16" y2="12"></line>
						</svg>
					</span>
				{/if}
			</button>

			<input
				type="text"
				class="title-input"
				bind:value={titleInput}
				placeholder="Untitled"
				onfocus={handleTitleFocus}
				onblur={handleTitleBlur}
				onkeydown={handleTitleKeydown}
			/>
		</div>

		<!-- Action buttons -->
		{#if !coverUrl}
			<div class="header-actions">
				<button class="action-btn">
					<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
						<circle cx="8.5" cy="8.5" r="1.5"></circle>
						<polyline points="21 15 16 10 5 21"></polyline>
					</svg>
					Add cover
				</button>
			</div>
		{/if}
	</div>
</header>

<style>
	.page-header {
		width: 100%;
	}

	.cover-image {
		height: 200px;
		background-size: cover;
		background-position: center;
		position: relative;
	}

	.change-cover-btn {
		position: absolute;
		bottom: 1rem;
		right: 1rem;
		padding: 0.375rem 0.75rem;
		background: rgba(255, 255, 255, 0.9);
		border: none;
		border-radius: 0.375rem;
		font-size: 0.75rem;
		cursor: pointer;
		opacity: 0;
		transition: opacity 0.15s;
	}

	.cover-image:hover .change-cover-btn {
		opacity: 1;
	}

	.header-content {
		max-width: 900px;
		margin: 0 auto;
		padding: 2rem 4rem;
	}

	.header-content.has-cover {
		margin-top: -3rem;
		position: relative;
		z-index: 1;
	}

	.breadcrumbs {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		margin-bottom: 0.5rem;
		font-size: 0.875rem;
	}

	.breadcrumb-item {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.25rem 0.5rem;
		background: transparent;
		border: none;
		border-radius: 0.25rem;
		color: var(--color-text-muted);
		cursor: pointer;
		transition: all 0.15s;
	}

	.breadcrumb-item:hover:not(:disabled) {
		background-color: var(--color-border);
		color: var(--color-text);
	}

	.breadcrumb-item:disabled {
		cursor: default;
		color: var(--color-text);
	}

	.breadcrumb-separator {
		color: var(--color-text-muted);
	}

	.breadcrumb-icon {
		font-size: 0.875rem;
	}

	.title-row {
		display: flex;
		align-items: flex-start;
		gap: 0.5rem;
	}

	.icon-button {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 48px;
		height: 48px;
		background: transparent;
		border: none;
		border-radius: 0.5rem;
		cursor: pointer;
		transition: background-color 0.15s;
		flex-shrink: 0;
	}

	.icon-button:hover {
		background-color: var(--color-border);
	}

	.page-icon {
		font-size: 2.5rem;
	}

	.add-icon {
		color: var(--color-text-muted);
		opacity: 0;
		transition: opacity 0.15s;
	}

	.title-row:hover .add-icon {
		opacity: 1;
	}

	.title-input {
		flex: 1;
		font-size: 2.5rem;
		font-weight: 700;
		background: transparent;
		border: none;
		outline: none;
		color: var(--color-text);
		padding: 0;
		width: 100%;
	}

	.title-input::placeholder {
		color: var(--color-text-muted);
	}

	.header-actions {
		display: flex;
		gap: 0.5rem;
		margin-top: 0.5rem;
		opacity: 0;
		transition: opacity 0.15s;
	}

	.header-content:hover .header-actions {
		opacity: 1;
	}

	.action-btn {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.5rem;
		background: transparent;
		border: none;
		border-radius: 0.25rem;
		font-size: 0.75rem;
		color: var(--color-text-muted);
		cursor: pointer;
		transition: all 0.15s;
	}

	.action-btn:hover {
		background-color: var(--color-border);
		color: var(--color-text);
	}
</style>
