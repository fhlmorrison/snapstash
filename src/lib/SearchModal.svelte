<script>
  import { images } from "./images";
  import TagAdder from "./TagAdder.svelte";

  let open = false;

  const toggleOpen = () => {
    open = !open;
  };

  let negativeTags = [];
  let positiveTags = [];

  let showAddTag = false;
</script>

<button on:click={toggleOpen}>
  <span class="toggle-triangle">{open ? "▼" : "▶"}</span>
  Advanced Search
</button>

{#if open}
  <div class="search-menu">
    <div class="tag-text">
      {#each positiveTags as tag}
        <button
          class="tag remove-tag"
          on:click={() => {
            positiveTags = positiveTags.filter((t) => t !== tag);
          }}>{tag}</button
        >
      {/each}
      {#if showAddTag}
        <div class="tag">
          <TagAdder
            on:close={() => {
              showAddTag = false;
            }}
            on:addTag={({ detail: tag }) => {
              positiveTags = [...positiveTags, tag];
            }}
          />
        </div>
      {:else}
        <button on:click={() => (showAddTag = true)} class="tag add-tag"
          >+</button
        >
      {/if}
    </div>
    <button on:click={() => images.searchByTags(positiveTags)}> Search </button>
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

  .tag-text {
    background-color: rgba(0, 0, 0, 0.5);
    padding: 1em;
    border-radius: 1em;
    height: 3rem;
    padding: 0.25rem 0.75rem;
    width: 100%;
    flex-wrap: nowrap;
    overflow: visible;
  }

  .tag {
    display: inline-block;
    border-radius: 25px;
    border: 1px solid black;
    padding: 0.25em 0.5em;
    margin-right: 0.5em;
    margin-bottom: 0.5em;
    text-align: center;
    background-color: rgba(0, 0, 0, 0);
    color: #fff;
  }

  .remove-tag:hover {
    background-color: rgba(255, 0, 0, 0.5);
  }

  .add-tag {
    min-width: 1em;
    font-size: 1.25rem;
    font-weight: 500;
    font-family: inherit;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }
</style>
