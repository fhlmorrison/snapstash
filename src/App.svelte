<script lang="ts">
  import ImageModal from "./lib/ImageModal.svelte";
  import type { FileEntry } from "@tauri-apps/api/fs";
  import ImageSquare from "./lib/ImageSquare.svelte";
  import {
    openImage,
    openImageDialogue,
    openDirectory,
    readDirImages,
    saveImages,
    searchImages,
  } from "./lib/loadassets";
  import type { ImageInfo } from "./lib/types";
  import { path } from "@tauri-apps/api";
  import SearchBar from "./lib/SearchBar.svelte";
  import TagModal from "./lib/TagModal.svelte";

  let imgUrl: string = "";
  let imageFiles: FileEntry[] = [];
  // $: console.log(imageFiles);
  let imageUrls: ImageInfo[] = [];

  let selectedIndex = 0;

  let selected: ImageInfo | null;
  let expanded: boolean;

  $: selected = imageUrls[selectedIndex] || null;

  const expandImage = (event: CustomEvent<number>) => {
    selectedIndex = event.detail;
    expanded = !expanded;
  };

  const selectImage = (event: CustomEvent<number>) => {
    selectedIndex = event.detail;
  };

  const openImages = async (files: FileEntry[]) => {
    imageUrls = await Promise.all(
      imageFiles.map(async (file) => ({
        path: file.path,
        name: file.name,
        src: await openImage(file.path),
      }))
    );
  };
  $: openImages(imageFiles);

  const getImage = async () => {
    imgUrl = (await openImageDialogue()) || "";
  };

  const getDir = async () => {
    imageFiles = await readDirImages(await openDirectory());
  };

  const save = async () => {
    await saveImages(imageFiles.map((file) => file.path));
  };

  const searchNew = async (e) => {
    imageFiles = await Promise.all(
      (
        await searchImages(e.detail)
      ).map(async (filePath) => {
        return {
          name: await path.basename(filePath),
          path: filePath,
        };
      })
    );
  };
</script>

<main class="container">
  <button on:click={getImage}>Open Image</button>
  <button on:click={getDir}>Open Directory</button>
  {#if imageUrls.length > 0}
    <button
      class="clear-button"
      on:click={() => {
        imageUrls = [];
      }}
      >Close Directory
    </button>
    <button class="save-button" on:click={save}> Save Images </button>
  {/if}
  <SearchBar on:search={searchNew} />
  <TagModal />

  {#if imgUrl}
    <div class="image-frame">
      <div
        class="x"
        on:click={() => {
          imgUrl = "";
        }}
        on:keydown={(event) => {
          if (event.key === "Delete") {
            imgUrl = "";
          }
        }}
      >
        x
      </div>
      <ImageSquare
        src={imgUrl}
        name="image"
        on:expand={(image) => {
          expanded = image.detail;
        }}
      />
    </div>
  {/if}

  <div class="image-grid">
    {#each imageUrls as image, index}
      <ImageSquare
        {index}
        src={image.src}
        name={image.name}
        path={image.path}
        on:expand={expandImage}
        on:select={selectImage}
      />
    {/each}
  </div>
</main>

{#if expanded}
  <ImageModal
    src={selected?.src}
    alt={selected?.name}
    path={selected?.path}
    on:close={expandImage}
    on:next={() => {
      selectedIndex = (selectedIndex + 1) % imageUrls.length;
    }}
    on:prev={() => {
      selectedIndex = (selectedIndex - 1 + imageUrls.length) % imageUrls.length;
    }}
  />
{/if}

<style>
  .clear-button {
    background-color: rgb(221, 161, 161);
  }

  .image-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 1em;
  }

  .x {
    display: flex;
    position: absolute;
    justify-content: right;
    right: 0;
    top: 0;
    padding: 0 0.5rem 0 0;
    cursor: pointer;
  }

  .image-frame {
    position: relative;
    border: 1px solid black;
  }
</style>
