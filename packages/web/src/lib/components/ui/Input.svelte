<script lang="ts">
	interface Props {
		type?: 'text' | 'email' | 'password' | 'number' | 'search';
		name?: string;
		placeholder?: string;
		value?: string;
		disabled?: boolean;
		required?: boolean;
		error?: string;
		class?: string;
		oninput?: (e: Event) => void;
	}

	let {
		type = 'text',
		name,
		placeholder,
		value = $bindable(''),
		disabled = false,
		required = false,
		error,
		class: className = '',
		oninput
	}: Props = $props();
</script>

<div class="w-full">
	<input
		{type}
		{name}
		{placeholder}
		{disabled}
		{required}
		bind:value
		{oninput}
		class="w-full px-3 py-2 text-sm border rounded-lg bg-white dark:bg-surface-800 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:border-transparent transition-colors {error
			? 'border-red-500'
			: 'border-surface-200 dark:border-surface-700'} {className}"
	/>
	{#if error}
		<p class="mt-1 text-xs text-red-500">{error}</p>
	{/if}
</div>
