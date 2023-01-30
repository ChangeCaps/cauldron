import { invoke } from "@tauri-apps/api";

export class Item {
	name: string = '';
	image: string = '';
	pixelated: boolean = true;
	description: string = '';
}

export type ItemId = string;

export async function searchItems(query: string): Promise<ItemId[]> {
	return invoke('search_items', { query });
}

export async function newItem(): Promise<ItemId> {
	return invoke('new_item');
}

export async function removeItem(id: ItemId): Promise<void> {
	return invoke('remove_item', { item: id });
}

export async function getItem(id: string): Promise<Item> {
	return invoke('get_item', { item: id });
}

export async function setItem(id: ItemId, item: Item): Promise<void> {
	return invoke('set_item', { item: id, value: item });
}

export async function storeItems(): Promise<void> {
	return invoke('store_items');
}

export async function loadItems(): Promise<void> {
	return invoke('load_items');
}
