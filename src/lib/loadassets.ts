import { convertFileSrc } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { readDir } from "@tauri-apps/api/fs";
import type { FileEntry } from "@tauri-apps/api/fs";

export async function openImageDialogue() {
  const file = (await open({
    defaultPath: "~/videos",
    multiple: false,
    filters: [
      {
        name: "Video",
        extensions: ["png", "jpg", "jpeg"],
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
  })) as string;
  return directory || "";
}

export async function readDirImages(dirPath: string) {
  const entries: FileEntry[] = await readDir(dirPath);
  // console.log(`Read entries from ${dirPath}`, entries);
  return processEntries(await entries).filter(
    (entry) => [".png", ".jpg", ".jpeg", ".gif", ".webp"].some((ext) => entry.name.endsWith(ext))
  ).reverse();
}




// Mutual recursion for processing nested directories
const processEntries = (entries: FileEntry[]): FileEntry[] => entries.flatMap((entry) => processEntry(entry));
const processEntry = ({ children = [], name, path }: FileEntry): FileEntry[] => [{ name, path }, ...processEntries(children)]