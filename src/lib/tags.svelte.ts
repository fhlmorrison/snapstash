import { apiInvoke, apiGet } from "./api";

export interface TagGroup {
  id: number;
  name: string;
  isAutoTaggable: boolean;
}

export interface Tag {
  id: number;
  name: string;
  groupId: number | null;
}

async function createTag(tag: string) {
  return await apiInvoke<void>("create_tag", { tag });
}

async function getTags() {
  return await apiGet<string[]>("get_tags");
}

async function getTagsV2() {
  return await apiGet<Tag[]>("get_tags_v2");
}

async function getTagGroups() {
  return await apiGet<TagGroup[]>("get_tag_groups");
}

async function createTagGroup(name: string, isAutoTaggable: boolean) {
  return await apiInvoke<void>("create_tag_group", { name, isAutoTaggable });
}

async function createTagWithGroup(name: string, groupId: number) {
  return await apiInvoke<void>("create_tag_with_group", { name, groupId });
}

async function assignTagToGroup(tagName: string, groupId: number | null) {
  return await apiInvoke<void>("assign_tag_to_group", { tagName, groupId });
}

async function updateTagGroupAutoTaggable(groupId: number, isAutoTaggable: boolean) {
  return await apiInvoke<void>("update_tag_group_auto_taggable", { groupId, isAutoTaggable });
}

/**
 *
 * @param tag Tag string to be added to the images
 * @param images Paths to the images to be tagged
 * @returns Promise<null>
 */
async function autoTag(tag: string, images: string[], strict = true) {
  try {
    return await apiInvoke<null>("auto_tag", { tag, images, strict });
  } catch (e) {
    console.log("Error auto tagging: ", e);
  }
}

async function clipAutoTag(images: string[], threshold = 0.25) {
  try {
    return await apiInvoke<void>("clip_auto_tag", { images, threshold });
  } catch (e) {
    console.log("Error CLIP auto tagging: ", e);
  }
}

async function tagAllImages(tag: string, images: string[]) {
  for (const image of images) {
    try {
      await tagImage(image, tag);
    } catch (e) {
      console.log("Error tagging image ", image, " error: ", e);
    }
  }
}

export async function tagImage(image: string, tag: string) {
  return await apiInvoke<void>("add_tag_to_image", { image, tag });
}

export async function removeTagFromImage(image: string, tag: string) {
  return await apiInvoke<void>("remove_tag_from_image", { image, tag });
}

interface TagStore {
  tags: string[];
  tagObjects: Tag[];
  groups: TagGroup[];
  refresh: () => Promise<void>;
  create: (tag: string) => Promise<void>;
  createGroup: (name: string, isAutoTaggable: boolean) => Promise<void>;
  updateGroupAutoTaggable: (groupId: number, isAutoTaggable: boolean) => Promise<void>;
  createTagWithGroup: (name: string, groupId: number) => Promise<void>;
  assignToGroup: (tagName: string, groupId: number | null) => Promise<void>;
  autoTag: (
    tag: string,
    images: string[],
    strict?: boolean
  ) => Promise<null | undefined>;
  clipAutoTag: (images: string[], threshold?: number) => Promise<void>;
  tagAllImages: (tag: string, images: string[]) => Promise<void>;
}

class TagStoreClass implements TagStore {
  tags = $state<string[]>([]);
  tagObjects = $state<Tag[]>([]);
  groups = $state<TagGroup[]>([]);

  constructor() {
    this.refresh();
  }

  refresh = async () => {
    const [tags, tagObjects, groups] = await Promise.all([
      getTags(),
      getTagsV2(),
      getTagGroups()
    ]);
    this.tags = tags;
    this.tagObjects = tagObjects;
    this.groups = groups;
  };

  create = async (tag: string) => {
    await createTag(tag);
    await this.refresh();
  };

  createGroup = async (name: string, isAutoTaggable: boolean) => {
    await createTagGroup(name, isAutoTaggable);
    await this.refresh();
  };

  updateGroupAutoTaggable = async (groupId: number, isAutoTaggable: boolean) => {
    await updateTagGroupAutoTaggable(groupId, isAutoTaggable);
    await this.refresh();
  };

  createTagWithGroup = async (name: string, groupId: number) => {
    await createTagWithGroup(name, groupId);
    await this.refresh();
  };

  assignToGroup = async (tagName: string, groupId: number | null) => {
    await assignTagToGroup(tagName, groupId);
    await this.refresh();
  };

  autoTag = async (tag: string, images: string[], strict = true) => {
    return await autoTag(tag, images, strict);
  };

  clipAutoTag = async (images: string[], threshold = 0.25) => {
    return await clipAutoTag(images, threshold);
  };

  tagAllImages = async (tag: string, images: string[]) => {
    return await tagAllImages(tag, images);
  };
}

export const tagStore = new TagStoreClass();
