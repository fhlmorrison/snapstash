<script lang="ts">
  import ImageModal from "./lib/ImageModal.svelte";
  import ImageSquare, { type onSelectEvent } from "./lib/ImageSquare.svelte";
  import {
    images,
    type ImageInfo,
    selection,
    filteredImages,
    filter,
  } from "./lib/images";
  import TagModal from "./lib/TagModal.svelte";
  import SearchModal from "./lib/SearchModal.svelte";
  import Masonry from "./lib/Masonry.svelte";
  import { configStore } from "./lib/config.svelte";
  import SettingsModal from "./lib/SettingsModal.svelte";

  // let selectedIndex = 0;
  // let selectedIndices: Set<number> = new Set();

  let selected: ImageInfo | null = $derived($filteredImages[$selection.anchor]);
  let expanded: boolean = $state(false);

  let isSingleImage = $derived($images.length === 1);

  const expandImage = (index: number | undefined) => {
    if (index != null) {
      $selection.anchor = index;
    }
    expanded = !expanded;
  };

  const selectImage = ({ index, shiftKey, ctrlKey }: onSelectEvent) => {
    let newSelectedIndices = new Set($selection.indices);
    let selectedIndex = $selection.anchor;
    if (shiftKey) {
      newSelectedIndices = new Set(
        Array.from({ length: Math.abs(index - selectedIndex) + 1 }, (_, i) =>
          index > selectedIndex ? selectedIndex + i : selectedIndex - i
        )
      );
    } else if (ctrlKey) {
      if (newSelectedIndices.has(index)) {
        newSelectedIndices.delete(index);
      } else {
        newSelectedIndices.add(index);
      }
    } else {
      newSelectedIndices = new Set([index]);
      selectedIndex = index;
    }
    selection.set({
      anchor: selectedIndex,
      indices: newSelectedIndices,
    });
  };

  // const searchNew = async (e) => {
  //   images.search(e.detail);
  // };

  let filteredIndices = $derived([...Array($filteredImages.length).keys()]);
</script>

<main class="container">
  <div id="button-section">
    <div id="open-buttons">
      <button onclick={images.openImage}>Open Image</button>
      <button onclick={images.opendir}>Open Directory</button>
      <button onclick={images.opendirRecursive}
        >Open Directory (Recursive)</button
      >
      {#if configStore.useREButton}
        <button onclick={images.openREFile}>Open RE</button>
      {/if}
      <button onclick={configStore.openSettings}>Settings</button>
    </div>
    {#if $images.length > 0}
      <button class="clear-button" onclick={images.reset}
        >Close Directory
      </button>
      <button class="save-button" onclick={images.save}> Save Images </button>
      <input
        type="text"
        bind:value={$filter}
        placeholder="Filter by name or path"
      />
    {/if}
    <!-- <SearchBar onSubmit={searchNew} /> -->
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
          onExpand={expandImage}
          onSelect={selectImage}
        />
      </div>
    </div>
  {:else if configStore.useMasonry}
    <Masonry
      items={$filteredImages}
      idKey="path"
      minColWidth={200}
      maxColWidth={245}
      gap={10}
    >
      {#snippet children({ idx, item }: { item: ImageInfo; idx: number })}
        <ImageSquare
          index={idx}
          selected={$selection.indices.has(idx)}
          src={item.src ?? ""}
          path={item.path}
          name={item.name}
          onExpand={expandImage}
          onSelect={selectImage}
        />
      {/snippet}
    </Masonry>
  {:else}
    <div class="image-grid">
      {#each $filteredImages as image, index}
        <ImageSquare
          {index}
          selected={$selection.indices.has(index)}
          src={image.src}
          path={image.path}
          name={image.name}
          onExpand={expandImage}
          onSelect={selectImage}
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
    onClose={() => expandImage(undefined)}
    onNext={() => {
      $selection.anchor = ($selection.anchor + 1) % $filteredImages.length;
    }}
    onPrev={() => {
      $selection.anchor =
        ($selection.anchor - 1 + $filteredImages.length) %
        $filteredImages.length;
    }}
  />
{/if}
<SettingsModal />

<style>
  .clear-button {
    background-color: rgb(221, 161, 161);
  }

  #open-buttons {
    /* display: grid;
    grid-template-columns: repeat(auto-fill, minmax(150px, 1fr)); */
    display: flex;
    flex-direction: row;
    gap: 1px;
    width: 100%;
  }

  #open-buttons > * {
    flex: 1;
  }

  #button-section {
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .image-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 10px;
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
