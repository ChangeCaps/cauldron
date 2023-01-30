<script lang=ts>
	import { onMount } from 'svelte';
	import { focusedItem, rightPanel } from '../stores';
	import { getItem, Item, removeItem, setItem, storeItems } from '../items';

	let item: Item = new Item();	

	const update = async () => {
		if (!$focusedItem) return;
		item = await getItem($focusedItem);
	};	
	
	const store = async () => {
		if (!$focusedItem) return;
		await setItem($focusedItem, item);
		storeItems();
	};

	const remove = async () => {
		if (!$focusedItem) return;
		await removeItem($focusedItem);
		storeItems();

		rightPanel.set(null);
	};

	const fallback = (e: any) => {
		e.target.src = 'https://i.imgur.com/GrhwMN1.png';
	};

	onMount(() => {
		focusedItem.subscribe(async (_) => {
			update();
		});
	});
</script>

<style>
	.item-editor {
		display: flex;
		flex-direction: column;
		flex: 1;
		gap: 8px;

		padding: 8px;
	}

	.row {
		display: flex;
		flex-direction: row;
		align-items: center;
		font-size: 1.5em;
		gap: 8px;

		color: var(--text-color);
	}

	img {
		filter: drop-shadow(2px 2px 8px #000);
	}

	input[type=checkbox] {
		width: 2em;
		height: 2em;
	}

	input[type=text] {
		height: 24px;
		padding: 4px;
		font-size: 1em;

		background: var(--background-color-light);
		color: var(--text-color);
		border: none;
		border-radius: 4px;
	}

	textarea {
		height: 64px;
		padding: 4px;
		font-size: 1em;

		background: var(--background-color-light);
		color: var(--text-color);
		border: none;
		border-radius: 4px;

		height: 10rem;

		resize: none;
	}

	button {
		height: 32px;
		font-size: 1em;

		background: var(--background-color-light);
		color: var(--text-color);
		border: none;
		border-radius: 4px;
	}

	button:hover {
		background: var(--background-color);

		transform: background 0.1s ease-in-out;
	}

	::placeholder {
		color: var(--text-color-dark);
	}
</style>

<div class="item-editor">
	<img src={item.image} 
	 on:error={fallback} 
	 alt="item" 
	 draggable=false
	 style="image-rendering: {item.pixelated ? 'pixelated' : 'auto'}"/>

	<div class="row">
		<input type="checkbox" bind:checked={item.pixelated} on:change={store} />
		Pixelated
	</div>
	<input type="text" bind:value={item.name} on:input={store} placeholder=Name />
	<input type="text" bind:value={item.image} on:input={store} placeholder=Image />
	<textarea bind:value={item.description} on:input={store} placeholder=Description />

	<button on:click={remove}>Remove</button>
</div>
