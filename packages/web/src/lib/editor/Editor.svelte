<script lang="ts">
	import { onMount, onDestroy, tick } from 'svelte';
	import { Editor } from '@tiptap/core';
	import { createExtensions } from './extensions';
	import { SlashCommands, type SlashCommand } from './extensions/slash-commands';
	import {
		createSlashSuggestion,
		createSuggestionState,
		type SuggestionState
	} from './extensions/suggestion-plugin';
	import SlashMenu from './components/SlashMenu.svelte';

	interface Props {
		content?: string;
		placeholder?: string;
		editable?: boolean;
		autofocus?: boolean;
		class?: string;
		onUpdate?: (content: string) => void;
		onCreate?: (editor: Editor) => void;
	}

	let {
		content = '',
		placeholder = "Type '/' for commands...",
		editable = true,
		autofocus = false,
		class: className = '',
		onUpdate,
		onCreate
	}: Props = $props();

	let element: HTMLDivElement;
	let editor: Editor | null = $state(null);
	let suggestionState = $state<SuggestionState>(createSuggestionState());
	let menuPosition = $state({ top: 0, left: 0 });

	function handleStateChange(state: SuggestionState) {
		suggestionState = state;

		// Calculate menu position
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
		const slashSuggestion = createSlashSuggestion({
			onStateChange: handleStateChange
		});

		editor = new Editor({
			element,
			extensions: [
				...createExtensions({ placeholder }),
				SlashCommands.configure({
					suggestion: slashSuggestion
				})
			],
			content,
			editable,
			autofocus: autofocus ? 'end' : false,
			editorProps: {
				attributes: {
					class: 'prose prose-sm sm:prose lg:prose-lg focus:outline-none max-w-none'
				}
			},
			onUpdate: ({ editor }) => {
				onUpdate?.(editor.getHTML());
			},
			onCreate: ({ editor }) => {
				onCreate?.(editor);
			}
		});

		return () => {
			editor?.destroy();
		};
	});

	onDestroy(() => {
		editor?.destroy();
	});

	export function getEditor(): Editor | null {
		return editor;
	}

	export function getContent(): string {
		return editor?.getHTML() ?? '';
	}

	export function getJSON() {
		return editor?.getJSON();
	}

	export function setContent(newContent: string) {
		editor?.commands.setContent(newContent);
	}

	export function focus() {
		editor?.commands.focus();
	}

	export function blur() {
		editor?.commands.blur();
	}

	// Reactive content updates
	$effect(() => {
		if (editor && content !== editor.getHTML()) {
			editor.commands.setContent(content, false);
		}
	});

	// Reactive editable updates
	$effect(() => {
		if (editor) {
			editor.setEditable(editable);
		}
	});
</script>

<div class="editor-wrapper {className}">
	<div bind:this={element} class="editor-content"></div>

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
	.editor-wrapper {
		width: 100%;
		position: relative;
	}

	.editor-content {
		width: 100%;
	}

	.slash-menu-container {
		position: fixed;
		z-index: 100;
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

	/* Task list styles */
	.editor-content :global(.task-list) {
		list-style: none;
		padding-left: 0;
	}

	.editor-content :global(.task-item) {
		display: flex;
		align-items: flex-start;
		gap: 0.5rem;
	}

	.editor-content :global(.task-item > label) {
		display: flex;
		align-items: center;
		user-select: none;
	}

	.editor-content :global(.task-item > label input[type='checkbox']) {
		cursor: pointer;
		width: 1rem;
		height: 1rem;
		margin: 0;
		border-radius: 0.25rem;
	}

	.editor-content :global(.task-item > div) {
		flex: 1;
	}

	.editor-content :global(.task-item[data-checked='true'] > div) {
		text-decoration: line-through;
		opacity: 0.6;
	}

	/* Code block styles */
	.editor-content :global(.code-block) {
		background-color: #1e1e1e;
		color: #d4d4d4;
		border-radius: 0.5rem;
		padding: 1rem;
		font-family: 'JetBrains Mono', 'Fira Code', monospace;
		font-size: 0.875rem;
		overflow-x: auto;
	}

	.editor-content :global(.code-block code) {
		background: none;
		padding: 0;
		color: inherit;
	}

	/* Syntax highlighting */
	.editor-content :global(.hljs-keyword) {
		color: #569cd6;
	}
	.editor-content :global(.hljs-string) {
		color: #ce9178;
	}
	.editor-content :global(.hljs-number) {
		color: #b5cea8;
	}
	.editor-content :global(.hljs-function) {
		color: #dcdcaa;
	}
	.editor-content :global(.hljs-comment) {
		color: #6a9955;
	}
	.editor-content :global(.hljs-variable) {
		color: #9cdcfe;
	}
	.editor-content :global(.hljs-built_in) {
		color: #4ec9b0;
	}
</style>
