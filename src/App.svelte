<script lang="ts">
  import ImageModal from "./lib/ImageModal.svelte";
  import ImageSquare from "./lib/ImageSquare.svelte";
  import {
    images,
    type ImageInfo,
    selection,
    filteredImages,
    filter,
  } from "./lib/images";
  import SearchBar from "./lib/SearchBar.svelte";
  import TagModal from "./lib/TagModal.svelte";
  import SearchModal from "./lib/SearchModal.svelte";

  // let selectedIndex = 0;
  // let selectedIndices: Set<number> = new Set();

  let selected: ImageInfo | null;
  let expanded: boolean;

  $: isSingleImage = $images.length === 1;

  $: selected = $filteredImages[$selection.anchor];

  const expandImage = (event: CustomEvent<number>) => {
    $selection.anchor = event.detail;
    expanded = !expanded;
  };

  const selectImage = (
    event: CustomEvent<{ index: number; shiftKey: boolean; ctrlKey: boolean }>
  ) => {
    let newSelectedIndices = new Set($selection.indices);
    let selectedIndex = $selection.anchor;
    console.log("selectImage", event.detail);
    if (event.detail.shiftKey) {
      newSelectedIndices = new Set(
        Array.from(
          { length: Math.abs(event.detail.index - selectedIndex) + 1 },
          (_, i) =>
            event.detail.index > selectedIndex
              ? selectedIndex + i
              : selectedIndex - i
        )
      );
    } else if (event.detail.ctrlKey) {
      if (newSelectedIndices.has(event.detail.index)) {
        newSelectedIndices.delete(event.detail.index);
      } else {
        newSelectedIndices.add(event.detail.index);
      }
    } else {
      newSelectedIndices = new Set([event.detail.index]);
      selectedIndex = event.detail.index;
    }
    selection.set({
      anchor: selectedIndex,
      indices: newSelectedIndices,
    });
  };

  const searchNew = async (e) => {
    images.search(e.detail);
  };
</script>

<main class="container">
  <div id="button-section">
    <div id="open-buttons">
      <button on:click={images.openImage}>Open Image</button>
      <button on:click={images.opendir}>Open Directory</button>
      <button on:click={images.opendirRecursive}
        >Open Directory (Recursive)</button
      >
      <button on:click={images.openREFile}>Open RE</button>
    </div>
    {#if $images.length > 0}
      <button class="clear-button" on:click={images.reset}
        >Close Directory
      </button>
      <button class="save-button" on:click={images.save}> Save Images </button>
      <input
        type="text"
        bind:value={$filter}
        placeholder="Filter by name or path"
      />
    {/if}
    <!-- <SearchBar on:search={searchNew} /> -->
    <SearchModal />
    <TagModal />
  </div>

  {#if isSingleImage}
    <div id="single-image">
      <div class="image-frame">
        <ImageSquare
          index={0}
          selected={$selection.indices.has(0)}
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
      {#each $filteredImages as image, index}
        <ImageSquare
          {index}
          selected={$selection.indices.has(index)}
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
    alt={selected?.name +
      (selected?.subreddit ? ` (${selected?.subreddit})` : "")}
    path={selected?.path}
    on:close={expandImage}
    on:next={() => {
      $selection.anchor = ($selection.anchor + 1) % $filteredImages.length;
    }}
    on:prev={() => {
      $selection.anchor =
        ($selection.anchor - 1 + $filteredImages.length) %
        $filteredImages.length;
    }}
  />
{/if}

<style>
  .clear-button {
    background-color: rgb(221, 161, 161);
  }

  #open-buttons {
    display: grid;
    grid-template-columns: repeat(4, minmax(150px, 1fr));
    gap: 1px;
    width: 100%;
  }

  #button-section {
    display: flex;
    flex-direction: column;
    gap: 1px;
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
