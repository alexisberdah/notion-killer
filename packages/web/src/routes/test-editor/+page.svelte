<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	let status = $state<'loading' | 'ready' | 'error'>('loading');
	let errorMessage = $state('');
	let Editor: any = $state(null);

	onMount(async () => {
		try {
			const module = await import('$lib/editor');
			Editor = module.Editor;
			status = 'ready';
		} catch (e) {
			status = 'error';
			errorMessage = e instanceof Error ? e.message : String(e);
			console.error('Failed to load editor:', e);
		}
	});
</script>

<svelte:head>
	<title>Test Editor | Notion Killer</title>
</svelte:head>

<div class="test-page">
	<header class="header">
		<h1>Editor Test Page</h1>
		<p class="subtitle">Test the block editor without backend connection</p>
		<div class="status" class:ready={status === 'ready'} class:error={status === 'error'}>
			{#if status === 'loading'}
				Loading editor...
			{:else if status === 'ready'}
				Editor ready
			{:else}
				Error: {errorMessage}
			{/if}
		</div>
	</header>

	<div class="editor-container">
		{#if status === 'ready' && Editor}
			<Editor
				placeholder="Type '/' for commands, or start typing..."
				autofocus
				onUpdate={() => console.log('Content updated')}
			/>
		{:else if status === 'loading'}
			<div class="loading">
				<div class="spinner"></div>
				<span>Loading editor...</span>
			</div>
		{:else if status === 'error'}
			<div class="error-box">
				<strong>Failed to load editor</strong>
				<p>{errorMessage}</p>
				<p class="hint">Check the browser console for details.</p>
			</div>
		{/if}
	</div>

	<div class="instructions">
		<h2>Features to Test</h2>

		<div class="section">
			<h3>Slash Commands</h3>
			<p>Type <kbd>/</kbd> to open the command menu</p>
			<ul>
				<li><code>/text</code> - Plain text</li>
				<li><code>/h1</code>, <code>/h2</code>, <code>/h3</code> - Headings</li>
				<li><code>/bullet</code> - Bullet list</li>
				<li><code>/numbered</code> - Numbered list</li>
				<li><code>/todo</code> - Todo/checkbox list</li>
				<li><code>/quote</code> - Blockquote</li>
				<li><code>/code</code> - Code block</li>
				<li><code>/divider</code> - Horizontal divider</li>
			</ul>
		</div>

		<div class="section">
			<h3>Text Formatting</h3>
			<p>Select text to show the formatting toolbar, or use keyboard shortcuts:</p>
			<ul>
				<li><kbd>Cmd+B</kbd> - Bold</li>
				<li><kbd>Cmd+I</kbd> - Italic</li>
				<li><kbd>Cmd+U</kbd> - Underline</li>
				<li><kbd>Cmd+E</kbd> - Inline code</li>
				<li><kbd>Cmd+Shift+S</kbd> - Strikethrough</li>
				<li><kbd>Cmd+Shift+H</kbd> - Highlight</li>
				<li><kbd>Cmd+K</kbd> - Add link</li>
			</ul>
		</div>

		<div class="section">
			<h3>Markdown Shortcuts</h3>
			<ul>
				<li><code># </code> - Heading 1</li>
				<li><code>## </code> - Heading 2</li>
				<li><code>### </code> - Heading 3</li>
				<li><code>- </code> or <code>* </code> - Bullet list</li>
				<li><code>1. </code> - Numbered list</li>
				<li><code>[] </code> - Todo item</li>
				<li><code>> </code> - Quote</li>
				<li><code>``` </code> - Code block</li>
				<li><code>--- </code> - Divider</li>
			</ul>
		</div>
	</div>
</div>

<style>
	.test-page {
		max-width: 900px;
		margin: 0 auto;
		padding: 2rem;
		font-family: system-ui, -apple-system, sans-serif;
	}

	.header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.header h1 {
		font-size: 2rem;
		font-weight: 700;
		margin: 0 0 0.5rem;
	}

	.subtitle {
		color: #6b7280;
		margin: 0 0 1rem;
	}

	.status {
		display: inline-block;
		padding: 0.25rem 0.75rem;
		border-radius: 9999px;
		font-size: 0.875rem;
		background: #fef3c7;
		color: #92400e;
	}

	.status.ready {
		background: #d1fae5;
		color: #065f46;
	}

	.status.error {
		background: #fee2e2;
		color: #991b1b;
	}

	.editor-container {
		border: 1px solid #e5e7eb;
		border-radius: 0.5rem;
		min-height: 400px;
		background: #fff;
		margin-bottom: 2rem;
	}

	.loading {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 400px;
		gap: 1rem;
		color: #6b7280;
	}

	.spinner {
		width: 32px;
		height: 32px;
		border: 3px solid #e5e7eb;
		border-top-color: #3b82f6;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.error-box {
		padding: 2rem;
		text-align: center;
		color: #991b1b;
	}

	.error-box .hint {
		color: #6b7280;
		font-size: 0.875rem;
	}

	.instructions {
		background: #f9fafb;
		border-radius: 0.5rem;
		padding: 1.5rem;
	}

	.instructions h2 {
		font-size: 1.25rem;
		font-weight: 600;
		margin: 0 0 1rem;
	}

	.section {
		margin-bottom: 1.5rem;
	}

	.section:last-child {
		margin-bottom: 0;
	}

	.section h3 {
		font-size: 1rem;
		font-weight: 600;
		margin: 0 0 0.5rem;
		color: #374151;
	}

	.section p {
		margin: 0 0 0.5rem;
		color: #6b7280;
	}

	.section ul {
		margin: 0;
		padding-left: 1.5rem;
	}

	.section li {
		margin: 0.25rem 0;
		color: #4b5563;
	}

	kbd {
		display: inline-block;
		padding: 0.125rem 0.375rem;
		background: #fff;
		border: 1px solid #d1d5db;
		border-radius: 0.25rem;
		font-family: ui-monospace, monospace;
		font-size: 0.875rem;
		box-shadow: 0 1px 2px rgba(0,0,0,0.05);
	}

	code {
		display: inline-block;
		padding: 0.125rem 0.375rem;
		background: #f3f4f6;
		border-radius: 0.25rem;
		font-family: ui-monospace, monospace;
		font-size: 0.875rem;
		color: #dc2626;
	}
</style>
