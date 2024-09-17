import { invoke } from "@tauri-apps/api";
import { writable } from "svelte/store";

const { subscribe, set, update } = writable<string[]>([]);

async function createTag(tag: string) {
  return await invoke<string[]>("create_tag", { tag });
}

async function getTags() {
  return await invoke<string[]>("get_tags");
}

/**
 *
 * @param tag Tag string to be added to the images
 * @param images Paths to the images to be tagged
 * @returns Promise<null>
 */
async function autoTag(tag: string, images: string[], strict = true) {
  try {
    return await invoke<null>("auto_tag", { tag, images, strict });
  } catch (e) {
    console.log("Error auto tagging: ", e);
  }
}

export const tags = {
  subscribe,
  set,
  update,
  refresh: async () => set(await getTags()),
  create: async (tag: string) => {
    await createTag(tag);
    await tags.refresh();
  },
  autoTag,
};

export async function tagImage(image: string, tag: string) {
  return await invoke<void>("add_tag_to_image", { image, tag });
}
