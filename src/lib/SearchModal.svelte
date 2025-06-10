<script lang="ts">
  import { imageStore } from "./images.svelte";
  import TagBar from "./TagBar.svelte";

  let open = $state(false);

  const toggleOpen = () => {
    open = !open;
  };

  let negativeTags = $state<string[]>([]);
  let positiveTags = $state<string[]>([]);
</script>

<button onclick={toggleOpen}>
  <span class="toggle-triangle">{open ? "▼" : "▶"}</span>
  Advanced Search
</button>

{#if open}
  <div class="search-menu">
    <div>
      Positive Tags
      <TagBar bind:tags={positiveTags} />
    </div>
    <div>
      Negative Tags
      <TagBar bind:tags={negativeTags} />
    </div>
    <!-- <button on:click={() => images.searchByTags(positiveTags)}> Search </button> -->
    <button
      onclick={() =>
        imageStore.searchByTagsAdvanced(positiveTags, negativeTags)}
    >
      Search
    </button>
  </div>
{/if}

<style>
  .toggle-triangle {
    display: inline-block;
    width: 1rem;
    height: 1rem;
  }
  .search-menu {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    margin: 0.5rem 15%;
  }
</style>
