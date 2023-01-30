<script lang=ts>
	import Categories from "./Categories.svelte";
	import Items from "./Items.svelte";

	enum MainPanelKind {
		Items,
		Categories,
	}

	let kinds = Object.values(MainPanelKind)
		.filter((k) => typeof k === "number") as MainPanelKind[];

	export let kind: MainPanelKind = MainPanelKind.Items;

	const selectPanel = (newKind: MainPanelKind) => {
		kind = newKind;
	};
</script>

<style>
	.main-panel {
		display: flex;
		flex-direction: column;
		flex: 1;
	}

	.panel-selector {
		display: flex;
		flex-direction: row;
		gap: 8px;

		padding: 4px 4px 4px 8px;

		box-shadow: 0 0 4px #000;
	}

	button {
		background: var(--background-color-light);
		border: none;
		border-radius: 0.5rem;
		
		margin: 0;
		padding: 4px;
	}

	button:hover {
		background: var(--background-color);

		transition: background 0.1s;
	}
</style>

<div class="main-panel">
	<div class="panel-selector">
		{#each kinds as enum_kind}
			<button on:click={() => selectPanel(enum_kind)}
			 style="color: {enum_kind === kind ? 'var(--primary-color)' : 'var(--text-color)'}">
				{MainPanelKind[enum_kind]}
			</button>			
		{/each}
		
	</div>

	{#if kind === MainPanelKind.Items}
		<Items />
	{:else if kind === MainPanelKind.Categories}
		<Categories />
	{/if}
</div>
