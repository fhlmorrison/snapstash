import { convertFileSrc } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { readDir } from "@tauri-apps/api/fs";
import type { FileEntry } from "@tauri-apps/api/fs";
import { invoke, path } from "@tauri-apps/api";
import { writable, get } from "svelte/store";

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
  return file ? convertFileSrc(file) : "";
}

async function openImage(fileName: string) {
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
  const entries: FileEntry[] = await readDir(dirPath);
  // console.log(`Read entries from ${dirPath}`, entries);
  return processEntries(await entries)
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

// Mutual recursion for processing nested directories
const processEntries = (entries: FileEntry[]): FileEntry[] =>
  entries.flatMap((entry) => processEntry(entry));

const processEntry = ({
  children = [],
  name,
  path,
}: FileEntry): FileEntry[] => [{ name, path }, ...processEntries(children)];

// Image Store
export type ImageInfo = { src: string; name: string; path: string };

const { subscribe, set, update } = writable<ImageInfo[]>([]);

const save = () => {
  const files = get<ImageInfo[]>(images).map(({ path }) => path);
  return saveImages(files);
};

const opendir = async () => {
  const dir = await openDirectory();
  if (dir) {
    const files = await readDirImages(dir);
    const mappedImages = await Promise.all(
      files.map(async ({ name, path }) => ({
        name,
        path,
        src: await openImage(path),
      }))
    );
    set(mappedImages);
  }
};

const search = async (queryText: string) => {
  const imageFiles: ImageInfo[] = await Promise.all(
    (
      await searchImages(queryText)
    ).map(async (filePath) => {
      return {
        name: await path.basename(filePath),
        path: filePath,
        src: await openImage(filePath),
      };
    })
  );
  set(imageFiles);
};

export const images = {
  subscribe,
  set,
  update,
  save,
  opendir,
  reset: () => set([]),
  search,
};
