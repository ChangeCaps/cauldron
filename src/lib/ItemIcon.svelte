<script lang=ts>
	import { listen } from '@tauri-apps/api/event'
	import { onMount } from 'svelte';
	import { getItem, Item, type ItemId } from '../items';

	export let itemId: ItemId;
	export let onclick: () => void;
	let item: Item = new Item();

	$: nameAbbreviated = item.name.length > 6 ? item.name.match(/\b([A-Z])/g)!.join('') : item.name;

	const update = async () => {
		item = await getItem(itemId);
	};

	const fallback = (event: any) => {
		event.target.src = 'https://i.imgur.com/GrhwMN1.png';
	};

	onMount(() => {
		update();
		listen('item-changed', (updated: any) => {
			if (updated.payload !== item) return;
			update();
		});
	});
	
</script>

<style>
	.item-icon {
		position: relative;
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

	.item-icon:hover {
		background-color: var(--background-color);

		transform: background 0.1s ease-in-out;
	}

	img {
		display: flex;
		margin: 4px;
		width: 100%;
		height: 100%;
		object-fit: contain;

		filter: drop-shadow(2px 2px 8px #000);
	}

	.name {
		position: absolute;

		bottom: 0;
		left: 0;

		margin: 0 0.2rem 0 0.2rem;

		font-size: 0.45em;
		font-weight: bold;
		color: var(--text-color);
	}
</style>

<div class="item-icon" on:click={onclick} on:keypress={onclick}>
	<img 
	 src={item.image}
	 on:error={fallback}
	 title={item.name} 
	 alt={item.name} 
	 style="image-rendering: {item.pixelated ? 'pixelated' : 'auto'}"
	/>

	<div class="name">{nameAbbreviated}</div>
</div>
