<script lang="ts">
	import {
		filterCommands,
		groupCommandsByCategory,
		categoryLabels,
		type SlashCommand
	} from '../extensions/slash-commands';

	interface Props {
		query: string;
		onSelect: (command: SlashCommand) => void;
		onClose: () => void;
	}

	let { query, onSelect, onClose }: Props = $props();

	let selectedIndex = $state(0);
	let menuRef: HTMLDivElement;

	let filteredCommands = $derived(filterCommands(query));
	let groupedCommands = $derived(groupCommandsByCategory(filteredCommands));

	// Reset selection when query changes
	$effect(() => {
		query; // Subscribe to query changes
		selectedIndex = 0;
	});

	// Scroll selected item into view
	$effect(() => {
		if (menuRef) {
			const selectedItem = menuRef.querySelector(`[data-index="${selectedIndex}"]`);
			if (selectedItem) {
				selectedItem.scrollIntoView({ block: 'nearest' });
			}
		}
	});

	function handleKeyDown(event: KeyboardEvent) {
		if (event.key === 'ArrowDown') {
			event.preventDefault();
			selectedIndex = Math.min(selectedIndex + 1, filteredCommands.length - 1);
		} else if (event.key === 'ArrowUp') {
			event.preventDefault();
			selectedIndex = Math.max(selectedIndex - 1, 0);
		} else if (event.key === 'Enter') {
			event.preventDefault();
			if (filteredCommands[selectedIndex]) {
				onSelect(filteredCommands[selectedIndex]);
			}
		} else if (event.key === 'Escape') {
			event.preventDefault();
			onClose();
		}
	}

	function handleItemClick(command: SlashCommand) {
		onSelect(command);
	}

	// Make keyboard handler available to parent
	export { handleKeyDown };
</script>

<div class="slash-menu" bind:this={menuRef} role="listbox" aria-label="Block types">
	{#if filteredCommands.length === 0}
		<div class="no-results">No results found</div>
	{:else}
		{#each [...groupedCommands.entries()] as [category, commands]}
			<div class="category">
				<div class="category-label">{categoryLabels[category] || category}</div>
				{#each commands as command, i}
					{@const globalIndex = filteredCommands.indexOf(command)}
					<button
						class="menu-item"
						class:selected={selectedIndex === globalIndex}
						data-index={globalIndex}
						onclick={() => handleItemClick(command)}
						onmouseenter={() => (selectedIndex = globalIndex)}
						role="option"
						aria-selected={selectedIndex === globalIndex}
					>
						<span class="item-icon">{command.icon}</span>
						<div class="item-content">
							<span class="item-title">{command.title}</span>
							<span class="item-description">{command.description}</span>
						</div>
					</button>
				{/each}
			</div>
		{/each}
	{/if}
</div>

<style>
	.slash-menu {
		position: absolute;
		z-index: 50;
		width: 320px;
		max-height: 300px;
		overflow-y: auto;
		background: var(--color-bg);
		border: 1px solid var(--color-border);
		border-radius: 0.5rem;
		box-shadow:
			0 10px 15px -3px rgba(0, 0, 0, 0.1),
			0 4px 6px -2px rgba(0, 0, 0, 0.05);
		padding: 0.5rem;
	}

	.no-results {
		padding: 0.75rem 1rem;
		text-align: center;
		color: var(--color-text-muted);
		font-size: 0.875rem;
	}

	.category {
		margin-bottom: 0.5rem;
	}

	.category:last-child {
		margin-bottom: 0;
	}

	.category-label {
		padding: 0.25rem 0.5rem;
		font-size: 0.75rem;
		font-weight: 500;
		color: var(--color-text-muted);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.menu-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		width: 100%;
		padding: 0.5rem;
		background: transparent;
		border: none;
		border-radius: 0.375rem;
		cursor: pointer;
		text-align: left;
		transition: background-color 0.1s;
	}

	.menu-item:hover,
	.menu-item.selected {
		background-color: var(--color-border);
	}

	.item-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background: var(--color-border);
		border-radius: 0.25rem;
		font-size: 0.875rem;
		font-weight: 500;
		flex-shrink: 0;
	}

	.item-content {
		display: flex;
		flex-direction: column;
		min-width: 0;
	}

	.item-title {
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--color-text);
	}

	.item-description {
		font-size: 0.75rem;
		color: var(--color-text-muted);
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}
</style>
