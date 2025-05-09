<script lang="ts">
  import ImageModal from "./lib/ImageModal.svelte";
  import ImageSquare from "./lib/ImageSquare.svelte";
  import { images, type ImageInfo } from "./lib/images";
  import SearchBar from "./lib/SearchBar.svelte";
  import TagModal from "./lib/TagModal.svelte";
  import SearchModal from "./lib/SearchModal.svelte";

  let selectedIndex = 0;

  let selected: ImageInfo | null;
  let expanded: boolean;

  $: isSingleImage = $images.length === 1;

  $: console.log($images);

  $: selected = $images[selectedIndex];

  const expandImage = (event: CustomEvent<number>) => {
    selectedIndex = event.detail;
    expanded = !expanded;
  };

  const selectImage = (event: CustomEvent<number>) => {
    selectedIndex = event.detail;
  };

  const searchNew = async (e) => {
    images.search(e.detail);
  };
</script>

<main class="container">
  <div id="open-buttons">
    <button on:click={images.openImage}>Open Image</button>
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

  {#if isSingleImage}
    <div id="single-image">
      <div class="image-frame">
        <ImageSquare
          index={0}
          src={$images[0].src}
          path={$images[0].path}
          name={$images[0].name}
          on:expand={expandImage}
          on:select={selectImage}
        />
      </div>
    </div>
  {:else}
    <div class="image-grid">
      {#each $images as image, index}
        <ImageSquare
          {index}
          src={image.src}
          path={image.path}
          name={image.name}
          on:expand={expandImage}
          on:select={selectImage}
        />
      {/each}
    </div>
  {/if}
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

  #single-image {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
  }

  .image-frame {
    width: clamp(200px, 100%, 50vh);
    position: relative;
    padding: 5px;
  }
</style>
