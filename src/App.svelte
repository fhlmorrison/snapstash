<script lang="ts">
  import ImageModal from "./lib/ImageModal.svelte";
  import ImageSquare from "./lib/ImageSquare.svelte";
  import { openImageDialogue, images, type ImageInfo } from "./lib/images";
  import SearchBar from "./lib/SearchBar.svelte";
  import TagModal from "./lib/TagModal.svelte";
  import SearchModal from "./lib/SearchModal.svelte";

  let imgUrl: string = "";

  let selectedIndex = 0;

  let selected: ImageInfo | null;
  let expanded: boolean;

  $: selected = $images[selectedIndex];

  const expandImage = (event: CustomEvent<number>) => {
    selectedIndex = event.detail;
    expanded = !expanded;
  };

  const selectImage = (event: CustomEvent<number>) => {
    selectedIndex = event.detail;
  };

  const getImage = async () => {
    imgUrl = (await openImageDialogue()) || "";
  };

  const searchNew = async (e) => {
    images.search(e.detail);
  };
</script>

<main class="container">
  <div id="open-buttons">
    <button on:click={getImage}>Open Image</button>
    <button on:click={images.opendir}>Open Directory</button>
    <button on:click={images.opendirRecursive}
      >Open Directory (Recursive)</button
    >
  </div>
  {#if $images.length > 0}
    <button class="clear-button" on:click={images.reset}
      >Close Directory
    </button>
    <button class="save-button" on:click={images.save}> Save Images </button>
  {/if}
  <SearchBar on:search={searchNew} />
  <SearchModal />
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
    {#each $images as image, index}
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
      selectedIndex = (selectedIndex + 1) % $images.length;
    }}
    on:prev={() => {
      selectedIndex = (selectedIndex - 1 + $images.length) % $images.length;
    }}
  />
{/if}

<style>
  .clear-button {
    background-color: rgb(221, 161, 161);
  }

  #open-buttons {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 0.5em;
    width: 100%;
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
