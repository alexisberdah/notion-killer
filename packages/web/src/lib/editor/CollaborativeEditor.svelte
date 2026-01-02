<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { Editor } from '@tiptap/core';
	import { createExtensions } from './extensions';
	import { SlashCommands } from './extensions/slash-commands';
	import { createSlashSuggestion, createSuggestionState, type SuggestionState } from './extensions/suggestion-plugin';
	import SlashMenu from './components/SlashMenu.svelte';
	import BubbleMenuWrapper from './components/BubbleMenuWrapper.svelte';
	import { crdtStore, createCollaborationExtensions, generateUserColor } from '$lib/crdt';
	import type { SlashCommand } from './extensions/slash-commands';

	interface Props {
		pageId: string;
		placeholder?: string;
		editable?: boolean;
		autofocus?: boolean;
		class?: string;
		userName?: string;
		onUpdate?: () => void;
		onCreate?: (editor: Editor) => void;
	}

	let {
		pageId,
		placeholder = "Type '/' for commands...",
		editable = true,
		autofocus = false,
		class: className = '',
		userName = 'Anonymous',
		onUpdate,
		onCreate
	}: Props = $props();

	let element: HTMLDivElement;
	let editor: Editor | null = $state(null);
	let suggestionState = $state<SuggestionState>(createSuggestionState());
	let menuPosition = $state({ top: 0, left: 0 });
	let isReady = $state(false);

	function handleStateChange(state: SuggestionState) {
		suggestionState = state;

		if (state.clientRect) {
			const rect = state.clientRect();
			if (rect) {
				menuPosition = {
					top: rect.bottom + 8,
					left: rect.left
				};
			}
		}
	}

	function handleMenuSelect(command: SlashCommand) {
		if (editor && suggestionState.range) {
			command.command({ editor, range: suggestionState.range });
		}
		suggestionState = { ...suggestionState, isOpen: false };
	}

	function handleMenuClose() {
		suggestionState = { ...suggestionState, isOpen: false };
	}

	onMount(() => {
		// Open CRDT document
		const crdtDoc = crdtStore.openDocument(pageId);

		const slashSuggestion = createSlashSuggestion({
			onStateChange: handleStateChange
		});

		// Create collaboration extensions
		const collaborationExtensions = createCollaborationExtensions({
			document: crdtDoc,
			user: {
				name: userName,
				color: generateUserColor()
			}
		});

		// Create editor with all extensions
		editor = new Editor({
			element,
			extensions: [
				...createExtensions({ placeholder }),
				...collaborationExtensions,
				SlashCommands.configure({
					suggestion: slashSuggestion
				})
			],
			editable,
			autofocus: autofocus ? 'end' : false,
			editorProps: {
				attributes: {
					class: 'prose prose-sm sm:prose lg:prose-lg focus:outline-none max-w-none'
				}
			},
			onUpdate: () => {
				onUpdate?.();
			},
			onCreate: ({ editor }) => {
				isReady = true;
				onCreate?.(editor);
			}
		});

		return () => {
			editor?.destroy();
			crdtStore.closeDocument();
		};
	});

	onDestroy(() => {
		editor?.destroy();
		crdtStore.closeDocument();
	});

	// Reactive editable updates
	$effect(() => {
		if (editor) {
			editor.setEditable(editable);
		}
	});

	export function getEditor(): Editor | null {
		return editor;
	}

	export function focus() {
		editor?.commands.focus();
	}

	export function blur() {
		editor?.commands.blur();
	}
</script>

<div class="collaborative-editor {className}">
	{#if !isReady || !crdtStore.isSynced}
		<div class="loading-overlay">
			<div class="loading-spinner"></div>
			<span class="loading-text">Loading document...</span>
		</div>
	{/if}

	<div class="sync-status" class:visible={crdtStore.isSaving}>
		<span class="sync-indicator"></span>
		<span>Saving...</span>
	</div>

	<div bind:this={element} class="editor-content" class:loading={!isReady}></div>

	{#if editor && isReady}
		<BubbleMenuWrapper {editor} />
	{/if}

	{#if suggestionState.isOpen}
		<div class="slash-menu-container" style="top: {menuPosition.top}px; left: {menuPosition.left}px;">
			<SlashMenu
				query={suggestionState.query}
				onSelect={handleMenuSelect}
				onClose={handleMenuClose}
			/>
		</div>
	{/if}
</div>

<style>
	.collaborative-editor {
		width: 100%;
		position: relative;
	}

	.loading-overlay {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		background: var(--color-bg);
		z-index: 10;
		gap: 1rem;
	}

	.loading-spinner {
		width: 32px;
		height: 32px;
		border: 3px solid var(--color-border);
		border-top-color: var(--color-text);
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	.loading-text {
		font-size: 0.875rem;
		color: var(--color-text-muted);
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.sync-status {
		position: absolute;
		top: 0.5rem;
		right: 0.5rem;
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.25rem 0.5rem;
		background: var(--color-bg);
		border: 1px solid var(--color-border);
		border-radius: 0.25rem;
		font-size: 0.75rem;
		color: var(--color-text-muted);
		opacity: 0;
		transition: opacity 0.2s;
		z-index: 5;
	}

	.sync-status.visible {
		opacity: 1;
	}

	.sync-indicator {
		width: 6px;
		height: 6px;
		background: #4ade80;
		border-radius: 50%;
		animation: pulse 1s ease-in-out infinite;
	}

	@keyframes pulse {
		0%, 100% {
			opacity: 1;
		}
		50% {
			opacity: 0.5;
		}
	}

	.editor-content {
		width: 100%;
		min-height: 200px;
	}

	.editor-content.loading {
		opacity: 0;
	}

	.editor-content :global(.ProseMirror) {
		min-height: 200px;
		padding: 1rem;
		outline: none;
	}

	.editor-content :global(.ProseMirror p.is-editor-empty:first-child::before) {
		color: #adb5bd;
		content: attr(data-placeholder);
		float: left;
		height: 0;
		pointer-events: none;
	}

	/* Remote cursor styles */
	.editor-content :global(.collaboration-cursor__caret) {
		border-left: 1px solid;
		border-right: 1px solid;
		margin-left: -1px;
		margin-right: -1px;
		pointer-events: none;
		position: relative;
		word-break: normal;
	}

	.editor-content :global(.collaboration-cursor__label) {
		font-size: 12px;
		font-weight: 600;
		left: -1px;
		line-height: normal;
		padding: 0.125rem 0.375rem;
		position: absolute;
		top: -1.4em;
		user-select: none;
		white-space: nowrap;
		border-radius: 3px 3px 3px 0;
	}

	.slash-menu-container {
		position: fixed;
		z-index: 100;
	}
</style>
