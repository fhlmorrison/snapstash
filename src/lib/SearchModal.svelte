<script>
  import { images } from "./images";
  import TagBar from "./TagBar.svelte";

  let open = false;

  const toggleOpen = () => {
    open = !open;
  };

  let negativeTags = [];
  let positiveTags = [];
</script>

<button on:click={toggleOpen}>
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
      on:click={() => images.searchByTagsAdvanced(positiveTags, negativeTags)}
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
