import { invoke } from "@tauri-apps/api";
import { writable } from "svelte/store";

const { subscribe, set, update } = writable<string[]>([]);

async function createTag(tag: string) {
    return await invoke<string[]>("create_tag", { tag });
}

async function getTags() {
    return await invoke<string[]>("get_tags");
}

export const tags = {
    subscribe,
    set,
    update,
    refresh: async () => set(await getTags()),
    create: async (tag: string) => { await createTag(tag); await tags.refresh() },
};