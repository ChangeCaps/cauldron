<script lang=ts>
	import { rightPanel, RightPanelKind } from "../stores";
	import ItemEditor from "./ItemEditor.svelte";
</script>

{#if $rightPanel === RightPanelKind.ItemEditor}
	<ItemEditor />
{/if}
