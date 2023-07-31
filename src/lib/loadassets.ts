import { convertFileSrc } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { readDir } from "@tauri-apps/api/fs";
import type { FileEntry } from "@tauri-apps/api/fs";
import { invoke } from "@tauri-apps/api";


const IMAGE_EXTENSIONS = [".png", ".jpg", ".jpeg", ".gif", ".webp"]

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

export async function openImage(fileName: string) {
  const newFileName = convertFileSrc(fileName);
  // console.log(newFileName);
  return newFileName;
}

export async function openDirectory() {
  const directory: string = (await open({
    defaultPath: "~/pictures",
    multiple: false,
    directory: true,
    recursive: true,
  })) as string;
  return directory || "";
}

export async function readDirImages(dirPath: string) {
  const entries: FileEntry[] = await readDir(dirPath);
  // console.log(`Read entries from ${dirPath}`, entries);
  return processEntries(await entries).filter(
    (entry) => IMAGE_EXTENSIONS.some((ext) => entry.name.endsWith(ext))
  ).reverse();
}

export async function readParameters(src: string) {
  try {
    return await invoke<string>("read_parameters", { src });

  }
  catch (e) {
    console.log("Error reading parameters: ", e);
  }
}

export async function saveImages(images: string[]) {
  try {
    return await invoke<string>("save_images", { images });

  }
  catch (e) {
    console.log("Error saving images: ", e);
  }
}

export function searchImages(queryText: string) {
  return invoke<string[]>("search_images", { queryText });
}


// Mutual recursion for processing nested directories
const processEntries = (entries: FileEntry[]): FileEntry[] => entries.flatMap((entry) => processEntry(entry));
const processEntry = ({ children = [], name, path }: FileEntry): FileEntry[] => [{ name, path }, ...processEntries(children)]