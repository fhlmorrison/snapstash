<script>
  import { onMount } from "svelte";
  import { tags } from "./tags";
  import { images } from "./images";

  const addTag = () => {
    tags.create(pendingTag);
    tags.refresh();
  };

  let open = false;

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
    <div class="taglist">
      {#each $tags as tag}
        <div class="tag">
          {tag}
          <button
            on:click={() =>
              tags.autoTag(
                tag,
                $images.map((i) => i.path)
              )}>AutoTag</button
          >
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
    /* margin: 0.5rem; */
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
