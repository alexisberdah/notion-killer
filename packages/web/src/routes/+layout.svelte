<script lang="ts">
	import '../app.css';
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();
</script>

<svelte:head>
	<title>Notion Killer</title>
	<meta name="description" content="A fast, offline-first alternative to Notion" />
</svelte:head>

{@render children()}
