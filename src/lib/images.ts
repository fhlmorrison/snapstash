import { convertFileSrc } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { readDir, readTextFile } from "@tauri-apps/api/fs";
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
  const entries: FileEntry[] = await readDir(dirPath);
  // console.log(`Read entries from ${dirPath}`, entries);
  return processEntries(entries)
    .filter((entry) => IMAGE_EXTENSIONS.some((ext) => entry.name.endsWith(ext)))
    .reverse();
}

async function readDirImagesRecursive(dirPath: string) {
  const entries: FileEntry[] = await readDir(dirPath, { recursive: true });
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

// Mutual recursion for processing nested directories
const processEntries = (entries: FileEntry[]): FileEntry[] => {
  console.log("Processing entries", entries);
  return entries.flatMap((entry) => processEntry(entry));
};

const processEntry = ({ children, name, path }: FileEntry): FileEntry[] => {
  console.log("Processing entry", { name, path, children });
  return [{ name, path }, ...processEntries(children ?? [])];
};

// Image Store
export type ImageInfo = { src: string; name: string; path: string };

const { subscribe, set, update } = writable<ImageInfo[]>([]);

const save = () => {
  const files = get<ImageInfo[]>(images).map(({ path }) => path);
  return saveImages(files);
};

const openImage = async () => {
  const filePath = await openImageDialogue();
  console.log("Selected file:", filePath);
  if (filePath) {
    set([
      {
        name: await path.basename(filePath),
        path: filePath,
        src: await openImageFile(filePath),
      },
    ]);
  }
};

const opendir = async () => {
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
    set(mappedImages);
  }
};

const opendirRecursive = async () => {
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
        src: await openImageFile(filePath),
      };
    })
  );
  set(imageFiles);
};

const searchByTags = async (tags: string[]) => {
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
  set(imageFiles);
};

const searchByTagsAdvanced = async (
  positiveTags: string[],
  negativeTags: string[]
) => {
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
  set(imageFiles);
};

async function readImgesFromRMExportFile(src: string) {}

async function readImgesFromREFile() {}

async function openImgesFromREFile() {
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
    set(entries);
  }
}

export const images = {
  subscribe,
  set,
  update,
  save,
  openImage,
  opendir,
  opendirRecursive,
  reset: () => set([]),
  search,
  searchByTags,
  searchByTagsAdvanced,
  openREFile: openImgesFromREFile,
};
