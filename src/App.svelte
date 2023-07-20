<script lang="ts">
  import ImageModal from "./lib/ImageModal.svelte";
  import type { FileEntry } from "@tauri-apps/api/fs";
  import ImageSquare from "./lib/ImageSquare.svelte";
  import {
    openImage,
    openImageDialogue,
    openDirectory,
    readDirImages,
  } from "./lib/loadassets";
  import type { ImageInfo } from "./lib/types";

  let imgUrl: string = "";
  let imageFiles: FileEntry[] = [];
  // $: console.log(imageFiles);
  let imageUrls: (FileEntry & { url: string })[] = [];
  let expanded: ImageInfo | null;
  let selected: ImageInfo | null;

  const expandImage = (event: CustomEvent<ImageInfo>) => {
    expanded = event.detail;
  };

  const selectImage = (event: CustomEvent<ImageInfo>) => {
    selected = event.detail;
  };

  const openImages = async (files: FileEntry[]) => {
    imageUrls = await Promise.all(
      imageFiles.map(async (file) => ({
        ...file,
        url: await openImage(file.path),
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
  {/if}

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
        alt="image"
        on:expand={(image) => {
          expanded = image.detail;
        }}
      />
    </div>
  {/if}

  <div class="image-grid">
    {#each imageUrls as image}
      <ImageSquare
        src={image.url}
        alt={image.name}
        path={image.path}
        on:expand={expandImage}
        on:select={selectImage}
      />
    {/each}
  </div>
</main>

{#if expanded}
  <ImageModal
    src={expanded?.src}
    alt={expanded?.alt}
    path={expanded?.path}
    on:close={expandImage}
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
