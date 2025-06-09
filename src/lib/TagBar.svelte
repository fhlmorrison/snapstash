<script>
  import TagAdder from "./TagAdder.svelte";

  /** @type {{tags?: any}} */
  let { tags = $bindable([]) } = $props();

  let showAddTag = $state(false);
</script>

<div class="tag-text">
  {#each tags as tag}
    <button
      class="tag remove-tag"
      onclick={() => {
        tags = tags.filter((t) => t !== tag);
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
          tags = [...tags, tag];
        }}
      />
    </div>
  {:else}
    <button onclick={() => (showAddTag = true)} class="tag add-tag">+</button>
  {/if}
</div>

<style>
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
