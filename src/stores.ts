import type { Writable } from 'svelte/store';
import { writable } from 'svelte/store';
import type { ItemId } from './items';

export enum LeftPanelKind {}

export enum RightPanelKind {
	ItemEditor,
}

export const leftPanel: Writable<LeftPanelKind | null> = writable(null);
export const rightPanel: Writable<RightPanelKind | null> = writable(null);

export const focusedItem: Writable<ItemId | null> = writable(null);
