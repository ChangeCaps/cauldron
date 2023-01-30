<script lang=ts>
	import { listen } from '@tauri-apps/api/event';
	import ItemIcon from './ItemIcon.svelte';
	import { focusedItem, rightPanel } from '../stores';
	import { onMount } from 'svelte';
	import { RightPanelKind } from '../stores';
	import { newItem, searchItems, type ItemId } from '../items';

	let query: string = '';
	let items: ItemId[] = [];

	const update = async () => {
		items = await searchItems(query);
	}

	const edit = (item: string) => {
		focusedItem.set(item);
		rightPanel.set(RightPanelKind.ItemEditor);
	}

	const addItem = async () => {
		let item = await newItem();

		update();
		edit(item);

		return item;
	}

	onMount(async () => {
		await listen('items-changed', update);
		update();
	});
</script>

<style>
	.panel {
		display: flex;
		flex-direction: column;
		flex: 1;
	}

	.search {
		display: flex;
		flex-direction: column;

		margin: 8px;
		margin-bottom: 0;
		padding: 4px;

		color: var(--text-color);
		background: var(--background-color-light);

		border: none;
		border-radius: 4px;
	}

	.search:focus {
		outline: none;
	}

	::placeholder {
		color: var(--text-color-dark);
	}

	.items {
		display: flex;
		flex-direction: row;
		flex-wrap: wrap;
		align-content: flex-start;
		flex: 1;

		overflow-y: scroll;

		margin: 8px;
		padding: 8px;
		gap: 8px;
		box-shadow: 0 0 8px #000 inset;

		background: var(--background-color);
	}

	.add-item {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;

		width: 64px;
		height: 64px;
	
		font-size: 2em;

		cursor: pointer;
		background: var(--background-color-light);

		box-shadow: 2px 2px 8px #000;
	}

	.add-item:hover {
		background-color: var(--background-color);

		transform: background 0.1s ease-in-out;
	}
</style>

<div class="panel">
	<input 
		class="search"
		type="text" 
		bind:value={query} 
		on:input={update} 
		placeholder="Search"
	/>

	<div class="items">	
		{#each items as item (item)}
			<ItemIcon itemId={item} onclick={() => edit(item)} />
		{/each}

		<div 
			class="add-item" 
			title="Add Item" 
			on:click={addItem}
			on:keypress={addItem}
		>
			+
		</div>
	</div>
</div>
