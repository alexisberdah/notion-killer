<script lang="ts">
	import type { Editor } from '@tiptap/core';

	interface Props {
		editor: Editor;
	}

	let { editor }: Props = $props();

	// Check if editor is available and has selection
	let isVisible = $derived(
		editor &&
			!editor.state.selection.empty &&
			editor.isEditable &&
			!editor.state.selection.$from.parent.type.spec.code
	);

	// Active states for formatting buttons
	let isBold = $derived(editor?.isActive('bold') ?? false);
	let isItalic = $derived(editor?.isActive('italic') ?? false);
	let isUnderline = $derived(editor?.isActive('underline') ?? false);
	let isStrike = $derived(editor?.isActive('strike') ?? false);
	let isCode = $derived(editor?.isActive('code') ?? false);
	let isHighlight = $derived(editor?.isActive('highlight') ?? false);
	let isLink = $derived(editor?.isActive('link') ?? false);

	function toggleBold() {
		editor.chain().focus().toggleBold().run();
	}

	function toggleItalic() {
		editor.chain().focus().toggleItalic().run();
	}

	function toggleUnderline() {
		editor.chain().focus().toggleUnderline().run();
	}

	function toggleStrike() {
		editor.chain().focus().toggleStrike().run();
	}

	function toggleCode() {
		editor.chain().focus().toggleCode().run();
	}

	function toggleHighlight() {
		editor.chain().focus().toggleHighlight().run();
	}

	function setLink() {
		if (isLink) {
			editor.chain().focus().unsetLink().run();
		} else {
			const url = window.prompt('Enter URL:');
			if (url) {
				editor.chain().focus().setLink({ href: url }).run();
			}
		}
	}

	// Heading options
	function setHeading(level: 1 | 2 | 3) {
		editor.chain().focus().toggleHeading({ level }).run();
	}

	function setParagraph() {
		editor.chain().focus().setParagraph().run();
	}

	// Get current block type
	let currentBlockType = $derived(() => {
		if (!editor) return 'text';
		if (editor.isActive('heading', { level: 1 })) return 'h1';
		if (editor.isActive('heading', { level: 2 })) return 'h2';
		if (editor.isActive('heading', { level: 3 })) return 'h3';
		return 'text';
	});
</script>

{#if isVisible}
	<div class="selection-toolbar" role="toolbar" aria-label="Text formatting">
		<!-- Block type dropdown -->
		<div class="toolbar-group">
			<select
				class="block-type-select"
				value={currentBlockType()}
				onchange={(e) => {
					const target = e.target as HTMLSelectElement;
					const value = target.value;
					if (value === 'text') setParagraph();
					else if (value === 'h1') setHeading(1);
					else if (value === 'h2') setHeading(2);
					else if (value === 'h3') setHeading(3);
				}}
			>
				<option value="text">Text</option>
				<option value="h1">Heading 1</option>
				<option value="h2">Heading 2</option>
				<option value="h3">Heading 3</option>
			</select>
		</div>

		<div class="toolbar-divider"></div>

		<!-- Text formatting -->
		<div class="toolbar-group">
			<button
				class="toolbar-btn"
				class:active={isBold}
				onclick={toggleBold}
				title="Bold (Ctrl+B)"
				aria-pressed={isBold}
			>
				<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
					<path d="M6 4h8a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z"></path>
					<path d="M6 12h9a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z"></path>
				</svg>
			</button>

			<button
				class="toolbar-btn"
				class:active={isItalic}
				onclick={toggleItalic}
				title="Italic (Ctrl+I)"
				aria-pressed={isItalic}
			>
				<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<line x1="19" y1="4" x2="10" y2="4"></line>
					<line x1="14" y1="20" x2="5" y2="20"></line>
					<line x1="15" y1="4" x2="9" y2="20"></line>
				</svg>
			</button>

			<button
				class="toolbar-btn"
				class:active={isUnderline}
				onclick={toggleUnderline}
				title="Underline (Ctrl+U)"
				aria-pressed={isUnderline}
			>
				<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M6 3v7a6 6 0 0 0 6 6 6 6 0 0 0 6-6V3"></path>
					<line x1="4" y1="21" x2="20" y2="21"></line>
				</svg>
			</button>

			<button
				class="toolbar-btn"
				class:active={isStrike}
				onclick={toggleStrike}
				title="Strikethrough"
				aria-pressed={isStrike}
			>
				<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<line x1="4" y1="12" x2="20" y2="12"></line>
					<path d="M17.5 7.5c0-2.5-2-4.5-5.5-4.5S6.5 4.5 6.5 7c0 1.5.8 2.5 2 3.5h7c1.2-1 2-2 2-3z"></path>
					<path d="M6.5 16.5c0 2.5 2 4.5 5.5 4.5s5.5-1.5 5.5-4c0-1.5-.8-2.5-2-3.5"></path>
				</svg>
			</button>
		</div>

		<div class="toolbar-divider"></div>

		<!-- Code and highlight -->
		<div class="toolbar-group">
			<button
				class="toolbar-btn"
				class:active={isCode}
				onclick={toggleCode}
				title="Code (Ctrl+E)"
				aria-pressed={isCode}
			>
				<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<polyline points="16 18 22 12 16 6"></polyline>
					<polyline points="8 6 2 12 8 18"></polyline>
				</svg>
			</button>

			<button
				class="toolbar-btn"
				class:active={isHighlight}
				onclick={toggleHighlight}
				title="Highlight"
				aria-pressed={isHighlight}
			>
				<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M12 20h9"></path>
					<path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z"></path>
				</svg>
			</button>
		</div>

		<div class="toolbar-divider"></div>

		<!-- Link -->
		<div class="toolbar-group">
			<button
				class="toolbar-btn"
				class:active={isLink}
				onclick={setLink}
				title="Link (Ctrl+K)"
				aria-pressed={isLink}
			>
				<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"></path>
					<path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"></path>
				</svg>
			</button>
		</div>
	</div>
{/if}

<style>
	.selection-toolbar {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		padding: 0.375rem;
		background: var(--color-bg);
		border: 1px solid var(--color-border);
		border-radius: 0.5rem;
		box-shadow:
			0 10px 15px -3px rgba(0, 0, 0, 0.1),
			0 4px 6px -2px rgba(0, 0, 0, 0.05);
	}

	.toolbar-group {
		display: flex;
		align-items: center;
		gap: 0.125rem;
	}

	.toolbar-divider {
		width: 1px;
		height: 24px;
		background: var(--color-border);
		margin: 0 0.25rem;
	}

	.toolbar-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		background: transparent;
		border: none;
		border-radius: 0.25rem;
		color: var(--color-text);
		cursor: pointer;
		transition: all 0.1s;
	}

	.toolbar-btn:hover {
		background: var(--color-border);
	}

	.toolbar-btn.active {
		background: var(--color-border);
		color: var(--color-text);
	}

	.block-type-select {
		padding: 0.25rem 0.5rem;
		background: transparent;
		border: none;
		border-radius: 0.25rem;
		font-size: 0.75rem;
		color: var(--color-text);
		cursor: pointer;
		outline: none;
	}

	.block-type-select:hover {
		background: var(--color-border);
	}

	.block-type-select:focus {
		outline: 2px solid var(--color-border);
	}
</style>
