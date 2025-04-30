<script>
  import { onMount } from "svelte";
  import { tags } from "./tags";
  import { images } from "./images";

  const addTag = () => {
    tags.create(pendingTag);
    tags.refresh();
  };

  $: filteredTags = $tags.filter((tag) =>
    tag.toLowerCase().includes(pendingTag.toLowerCase())
  );

  let open = false;

  let strict = true;

  const toggleOpen = () => {
    open = !open;
  };

  onMount(() => {
    tags.refresh();
  });

  let pendingTag = "";
</script>

<button on:click={toggleOpen}>
  <span class="toggle-triangle">{open ? "▼" : "▶"}</span>
  Tags
</button>

{#if open}
  <div class="tag-menu">
    <!-- markup (zero or more items) goes here -->
    <button on:click={tags.refresh}>Refresh Tags</button>
    <label for="auto-tag"
      >Strict
      <input type="checkbox" id="auto-tag" bind:checked={strict} />
    </label>
    <div class="taglist">
      {#each filteredTags as tag}
        <div class="tag">
          {tag}
          <button
            on:click={() =>
              tags.autoTag(
                tag,
                $images.map((i) => i.path),
                strict
              )}>AutoTag</button
          >
          <button
            on:click={() =>
              tags.tagAllImages(
                tag,
                $images.map((i) => i.path)
              )}>Tag All</button
          >
          <button on:click={() => images.searchByTag(tag)}>Open Tagged</button>
        </div>
      {/each}
    </div>
    <input type="text" bind:value={pendingTag} />
    <button on:click={addTag}>Add Tag</button>
  </div>
{/if}

<style>
  .tag {
    display: inline-block;
    border-radius: 25px;
    border: 1px solid black;
  }
  .taglist {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    padding: 0 10%;
    height: 50vh;
    overflow-y: scroll;
  }
  .toggle-triangle {
    display: inline-block;
    width: 1rem;
    height: 1rem;
  }
  .tag-menu {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin: 0.5rem 15%;
  }
</style>
