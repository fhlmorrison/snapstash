<script lang="ts">
  import { tagStore } from "./tags.svelte";

  let queryString = $state("");

  interface Props {
    onClose: () => void;
    onAddTag: (tag: string) => void;
  }

  let { onClose, onAddTag }: Props = $props();

  $effect(() => {
    inputref?.focus();
  });

  let inputref: HTMLInputElement;

  const addTag = (tag: string) => {
    onAddTag(tag);
    onClose();
  };

  const createAndAddTag = async (tagString: string) => {
    await tagStore.create(tagString);
    onAddTag(tagString);
    onClose();
  };
  let filteredTags = $derived(
    tagStore.tags.filter((tag) =>
      tag.toLowerCase().includes(queryString.toLowerCase())
    )
  );
</script>

<div id="tag-adder">
  <div id="inputbox">
    <input
      bind:this={inputref}
      id="queryinput"
      type="text"
      placeholder="Add a tag"
      autocomplete="off"
      bind:value={queryString}
    />
    <div id="taglist">
      {#each filteredTags as tag}
        <button class="tag" onclick={() => addTag(tag)}>{tag}</button>
      {/each}
      {#if queryString.length > 0 && !filteredTags.includes(queryString)}
        <button class="tag" onclick={() => createAndAddTag(queryString)}
          >Create "{queryString}" tag</button
        >
      {/if}
    </div>
  </div>
  <button onclick={onClose}>Close</button>
</div>

<style>
  #tag-adder {
    position: relative;
    width: fit-content;
    display: flex;
    flex-direction: row;
  }

  #taglist {
    display: flex;
    gap: 5px;
    flex-direction: column;
    max-height: 300px;
    width: 100%;
    overflow-y: scroll;
    flex-wrap: nowrap;
    position: absolute;
    bottom: 50px;
    left: 0;
    /* opacity: 0.9; */
    z-index: 100;
  }

  #inputbox {
    width: fit-content;
  }
</style>
