<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { tagStore } from "./tags.svelte";

  let queryString = $state("");

  const dispatch = createEventDispatcher();
  const addTag = (tag: string) => {
    dispatch("addTag", tag);
    dispatch("close", null);
  };

  const createAndAddTag = async (tagString: string) => {
    await tagStore.create(tagString);
    dispatch("addTag", tagString);
    dispatch("close", null);
  };
  const onClose = () => {
    dispatch("close", null);
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
      id="queryinput"
      type="text"
      placeholder="Add a tag"
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
