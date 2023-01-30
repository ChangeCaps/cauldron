<script lang=ts> 
	import DarkTheme from '../lib/DarkTheme.svelte';
	import { leftPanel, rightPanel } from '../stores';
	import { onMount } from 'svelte';
	import MainPanel from '$lib/MainPanel.svelte';
	import RightPanel from '$lib/RightPanel.svelte';
	import LeftPanel from '$lib/LeftPanel.svelte';
	import { loadItems } from '../items';

	let isVertical = false;

	const resize = () => {
		isVertical = window.innerWidth < 768;
	};

	const exitLeftPanel = () => {
		leftPanel.set(null);
	};

	const exitRightPanel = () => {
		rightPanel.set(null);
	};

	onMount(async () => {
		await loadItems();
	});
</script>

<style>
</style>

<svelte:head>
	<style>
		* {
			overflow: hidden;
		}

		body {
			padding: 0;
			margin: 0;

			width: 100vw;
			height: 100vh;

			background: var(--background-color);

			display: flex;
			flex-direction: column;

			user-select: none;
			-webkit-user-select: none;
		}

		.panels {
			display: flex;
			flex-direction: row;
			flex: 1;
		}		

		.panel {
			position: relative;
			display: flex;
			flex-direction: column;
		}

		.panel-content {
			height: fit-content;
			overflow-y: scroll;
		}

		.exit {
			position: absolute;
			top: -1.2rem;
			right: 0;

			height: 3rem;

			margin: 0;
			padding: 8px;

			z-index: 1;

			color: var(--text-color-dark);
			font-size: 3rem;
			font-weight: bold;

			cursor: pointer;
		}

		.exit:hover {
			opacity: 0.5;

			transition: opacity 0.1s;
		}

		.spacer {
			width: 8px;

			box-shadow: 0 0 8px #000 inset;
		}
	</style>
</svelte:head>

<svelte:window on:resize={resize} />

<DarkTheme />
<div class=panels>
	{#if $leftPanel !== null}
		<div class=panel>
			<div class=exit 
			 on:click={exitLeftPanel} 
			 on:keypress={exitLeftPanel}>
				&times;
			</div>

			<LeftPanel />
		</div>

		{#if !isVertical}
			<div class=spacer />
		{/if}
	{/if}

	{#if $rightPanel === null && $leftPanel === null || !isVertical}
		<MainPanel />
	{/if}

	{#if $rightPanel !== null}
		{#if !isVertical}
			<div class=spacer />
		{/if}

		<div class=panel
		 style="width: { isVertical ? '100%' : '20rem' }">
			<div class=exit 
			 on:click={exitRightPanel} 
			 on:keypress={exitRightPanel}>
				&times;
			</div>

			<div class=panel-content>
				<RightPanel />
			</div>
		</div>
	{/if}
</div>
