<script lang="ts">
  import ImageModal from "./lib/ImageModal.svelte";
  import ImageSquare, { type onSelectEvent } from "./lib/ImageSquare.svelte";
  import { type ImageInfo, imageStore } from "./lib/images.svelte";
  import TagModal from "./lib/TagModal.svelte";
  import SearchModal from "./lib/SearchModal.svelte";
  import Masonry from "./lib/Masonry.svelte";
  import { configStore } from "./lib/config.svelte";
  import SettingsModal from "./lib/SettingsModal.svelte";

  // let selectedIndex = 0;
  // let selectedIndices: Set<number> = new Set();

  let selected: ImageInfo | null = $derived(
    imageStore.filteredImages[imageStore.selection.anchor]
  );
  let expanded: boolean = $state(false);

  let isSingleImage = $derived(imageStore.images.length === 1);

  const expandImage = (index: number | undefined) => {
    if (index != null) {
      imageStore.selection.anchor = index;
    }
    expanded = !expanded;
  };

  const selectImage = ({ index, shiftKey, ctrlKey }: onSelectEvent) => {
    let newSelectedIndices = new Set(imageStore.selection.indices);
    let selectedIndex = imageStore.selection.anchor;
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
    imageStore.selection = {
      anchor: selectedIndex,
      indices: newSelectedIndices,
    };
  };

  // const searchNew = async (e) => {
  //   images.search(e.detail);
  // };

  let filteredIndices = $derived([
    ...Array(imageStore.filteredImages.length).keys(),
  ]);
</script>

<main class="container">
  <div id="button-section">
    <div id="open-buttons">
      <button onclick={imageStore.openImage}>Open Image</button>
      <button onclick={imageStore.opendir}>Open Directory</button>
      <button onclick={imageStore.opendirRecursive}
        >Open Directory (Recursive)</button
      >
      {#if configStore.useREButton}
        <button onclick={imageStore.openREFile}>Open RE</button>
      {/if}
      <button onclick={configStore.openSettings}>Settings</button>
    </div>
    {#if imageStore.images.length > 0}
      <button class="clear-button" onclick={imageStore.reset}
        >Close Directory
      </button>
      <button class="save-button" onclick={imageStore.save}>
        Save Images
      </button>
      <input
        type="text"
        bind:value={imageStore.filter}
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
          selected={imageStore.selection.indices.has(0)}
          src={imageStore.images[0].src}
          path={imageStore.images[0].path}
          name={imageStore.images[0].name}
          onExpand={expandImage}
          onSelect={selectImage}
        />
      </div>
    </div>
  {:else if configStore.useMasonry}
    <Masonry
      items={imageStore.filteredImages}
      idKey="path"
      minColWidth={200}
      maxColWidth={245}
      gap={10}
    >
      {#snippet children({ idx, item }: { item: ImageInfo; idx: number })}
        <ImageSquare
          index={idx}
          selected={imageStore.selection.indices.has(idx)}
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
      {#each imageStore.filteredImages as image, index}
        <ImageSquare
          {index}
          selected={imageStore.selection.indices.has(index)}
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
      imageStore.selection.anchor =
        (imageStore.selection.anchor + 1) % imageStore.filteredImages.length;
    }}
    onPrev={() => {
      imageStore.selection.anchor =
        (imageStore.selection.anchor - 1 + imageStore.filteredImages.length) %
        imageStore.filteredImages.length;
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
