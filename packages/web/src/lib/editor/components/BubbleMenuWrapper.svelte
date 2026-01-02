<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import type { Editor } from '@tiptap/core';
	import { BubbleMenuPlugin, type BubbleMenuPluginProps } from '@tiptap/extension-bubble-menu';
	import SelectionToolbar from './SelectionToolbar.svelte';

	interface Props {
		editor: Editor;
	}

	let { editor }: Props = $props();

	let menuElement: HTMLDivElement;

	onMount(() => {
		if (!editor || !menuElement) return;

		const pluginKey = 'bubbleMenu';

		const plugin = BubbleMenuPlugin({
			pluginKey,
			editor,
			element: menuElement,
			tippyOptions: {
				duration: 100,
				placement: 'top',
				appendTo: () => document.body
			},
			shouldShow: ({ editor, view, state, from, to }) => {
				// Don't show for code blocks
				const { selection } = state;
				const isEmptySelection = selection.empty;

				// Check if we're in a code block
				const fromPos = selection.$from;
				const isCodeBlock = fromPos.parent.type.name === 'codeBlock';

				// Show only for text selections (not empty, not code block)
				return !isEmptySelection && !isCodeBlock && editor.isEditable;
			}
		});

		editor.registerPlugin(plugin);

		return () => {
			editor.unregisterPlugin(pluginKey);
		};
	});
</script>

<div bind:this={menuElement} class="bubble-menu-container">
	<SelectionToolbar {editor} />
</div>

<style>
	.bubble-menu-container {
		/* Container styles handled by tippy */
	}
</style>
