import { convertFileSrc } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { BaseDirectory, readDir, readTextFile } from "@tauri-apps/plugin-fs";
import type { DirEntry, FileHandle } from "@tauri-apps/plugin-fs";
import { invoke } from "@tauri-apps/api/core";
import { path } from "@tauri-apps/api";
import { writable, get, derived } from "svelte/store";
import { join } from "@tauri-apps/api/path";

const IMAGE_EXTENSIONS = ["png", "jpg", "jpeg", "gif", "webp", "mp4", "webm"];

export async function openImageDialogue() {
  const file = (await open({
    multiple: false,
    filters: [
      {
        name: "Images",
        extensions: IMAGE_EXTENSIONS,
      },
    ],
  })) as string;
  return file;
}

async function openImageFile(fileName: string) {
  const newFileName = convertFileSrc(fileName);
  // console.log(newFileName);
  return newFileName;
}

async function openDirectory() {
  const directory: string = (await open({
    defaultPath: "~/pictures",
    multiple: false,
    directory: true,
    recursive: true,
  })) as string;
  return directory || "";
}

async function readDirImages(dirPath: string) {
  const entries: DirEntry[] = await readDir(dirPath);
  // console.log(`Read entries from ${dirPath}`, entries);
  return (await processEntries(dirPath, entries))
    .filter((entry) => IMAGE_EXTENSIONS.some((ext) => entry.name.endsWith(ext)))
    .reverse();
}

async function readDirImagesRecursive(dirPath: string) {
  const entries: DirEntry[] = await readDir(dirPath);
  // console.log(`Read entries from ${dirPath}`, entries);
  return (await processEntriesRecursively(dirPath, entries))
    .filter((entry) => IMAGE_EXTENSIONS.some((ext) => entry.name.endsWith(ext)))
    .reverse();
}

export async function readParameters(src: string) {
  try {
    return await invoke<string>("read_parameters", { src });
  } catch (e) {
    console.log("Error reading parameters: ", e);
  }
}

export async function readTags(src: string) {
  try {
    return await invoke<Array<string>>("read_tags", { src });
  } catch (e) {
    console.log("Error reading tags: ", e);
  }
}

async function saveImages(images: string[]) {
  try {
    return await invoke<string>("save_images", { images });
  } catch (e) {
    console.log("Error saving images: ", e);
  }
}

function searchImages(queryText: string) {
  return invoke<string[]>("search_images", { queryText });
}

function searchImagesWithTags(tags: string[]) {
  return invoke<string[]>("search_with_tags", { tags });
}

function searchImagesWithTagsAdvanced(
  positiveTags: string[],
  negativeTags: string[]
) {
  return invoke<string[]>("search_with_tags_advanced", {
    positiveTags,
    negativeTags,
  });
}

async function processEntriesRecursively(
  parent: string,
  entries: DirEntry[]
): Promise<ImageInfo[]> {
  const fileEntryPromises = entries.map(async (entry) => {
    const entryPath = await join(parent, entry.name);

    if (entry.isDirectory) {
      const subDirEntries = await readDir(entryPath);
      return await processEntriesRecursively(entryPath, subDirEntries);
    } else {
      return [
        {
          name: entry.name,
          path: entryPath,
          src: convertFileSrc(entryPath),
        },
      ];
    }
  });

  // Await all promises and then flatten the resulting array of arrays
  const nestedResults = await Promise.all(fileEntryPromises);
  return nestedResults.flat();
}

async function processEntries(parent: string, entries: DirEntry[]) {
  return Promise.all(
    entries
      .filter((e) => e.isFile)
      .map(async (entry) => {
        const path = await join(parent, entry.name);
        return {
          name: entry.name,
          path,
          src: convertFileSrc(path),
        };
      })
  );
}

// Image Store
export type ImageInfo = {
  src: string;
  name: string;
  path: string;
  subreddit?: string;
  threadUrl?: string;
};

interface ImageStore {
  images: ImageInfo[];
  filter: string;
  selection: { anchor: number; indices: Set<number> };
  save: () => Promise<string | undefined>;
  openImage: () => Promise<void>;
  opendir: () => Promise<void>;
  opendirRecursive: () => Promise<void>;
  reset: () => void;
  search: (queryText: string) => Promise<void>;
  searchByTags: (tags: string[]) => Promise<void>;
  searchByTagsAdvanced: (
    positiveTags: string[],
    negativeTags: string[]
  ) => Promise<void>;
  openREFile: () => Promise<void>;
}

class ImageStoreClass implements ImageStore {
  images = $state<ImageInfo[]>([]);
  filter = $state<string>("");
  selection = $state<{ anchor: number; indices: Set<number> }>({
    anchor: -1,
    indices: new Set(),
  });

  selectedImages = $derived(
    this.images.filter((_, index) => this.selection.indices.has(index))
  );

  filteredImages = $derived(
    this.images.filter(
      (image) =>
        image.name.toLowerCase().includes(this.filter.toLowerCase()) ||
        image.path.toLowerCase().includes(this.filter.toLowerCase()) ||
        image.subreddit?.toLowerCase().includes(this.filter.toLowerCase())
    )
  );

  /// Saves the current images into the database
  async save() {
    const files = this.images.map(({ path }) => path);
    return await saveImages(files);
  }

  /// Opens a dialogue to select a single image file to open
  async openImage() {
    const filePath = await openImageDialogue();
    console.log("Selected file:", filePath);
    if (filePath) {
      this.images = [
        {
          name: await path.basename(filePath),
          path: filePath,
          src: await openImageFile(filePath),
        },
      ];
    }
  }

  async opendir() {
    const dir = await openDirectory();
    console.log("Selected directory:", dir);
    if (dir) {
      const files = await readDirImages(dir);
      const mappedImages = await Promise.all(
        files.map(async ({ name, path }) => ({
          name,
          path,
          src: await openImageFile(path),
        }))
      );
      this.images = mappedImages;
    }
  }

  async opendirRecursive() {
    const dir = await openDirectory();
    if (dir) {
      const files = await readDirImagesRecursive(dir);
      const mappedImages = await Promise.all(
        files.map(async ({ name, path }) => ({
          name,
          path,
          src: await openImageFile(path),
        }))
      );
      this.images = mappedImages;
    }
  }

  reset() {
    this.images = [];
  }

  async search(queryText: string) {
    const imageFiles: ImageInfo[] = await Promise.all(
      (
        await searchImages(queryText)
      ).map(async (filePath) => {
        return {
          name: await path.basename(filePath),
          path: filePath,
          src: await openImageFile(filePath),
        };
      })
    );
    this.images = imageFiles;
  }

  async searchByTags(tags: string[]) {
    const imageFiles: ImageInfo[] = await Promise.all(
      (
        await searchImagesWithTags(tags)
      ).map(async (filePath) => {
        return {
          name: await path.basename(filePath),
          path: filePath,
          src: await openImageFile(filePath),
        };
      })
    );
    this.images = imageFiles;
  }

  async searchByTagsAdvanced(positiveTags: string[], negativeTags: string[]) {
    const imageFiles: ImageInfo[] = await Promise.all(
      (
        await searchImagesWithTagsAdvanced(positiveTags, negativeTags)
      ).map(async (filePath) => {
        return {
          name: await path.basename(filePath),
          path: filePath,
          src: await openImageFile(filePath),
        };
      })
    );
    this.images = imageFiles;
  }

  async openREFile() {
    const file = await open({
      multiple: false,
      filters: [
        {
          name: "text",
          extensions: ["txt"],
        },
      ],
    });

    if (file) {
      const content = await readTextFile(file as string);
      console.log("File content:", content);
      const lines = content.split("\n");
      console.log("Lines:", lines);
      const entries = lines
        .filter((e) => e.length > 0)
        .map((line) => {
          const [_, url, subreddit, __, title, threadUrl] = line.split("|");
          return {
            name: title,
            src: url,
            path: "",
            subreddit: subreddit,
            threadUrl: threadUrl,
          };
        });
      this.images = entries;
    }
  }
}

export const imageStore = new ImageStoreClass();
