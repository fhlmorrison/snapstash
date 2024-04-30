<script lang="ts">
  import { readParameters, readTags } from "./images";
  import { onMount } from "svelte";
  import { createEventDispatcher } from "svelte";
  export let src = "";
  export let alt = "";
  export let path = "";
  const dispatch = createEventDispatcher();

  let parameterText = "";

  $: updateParameterText(path);

  const updateParameterText = async (pth: string) => {
    parameterText = await readParameters(pth);
  };

  // onMount(async () => {
  //     parameterText = await readParameters(path);
  // });

  let showTags = false;

  let tags = [];

  $: setTags(path);

  const setTags = async (src: string) => {
    tags = await readTags(src);
  };

  let modal;
  onMount(() => {
    modal.focus();
  });

  let closeModal = (): void => {
    dispatch("close", null);
  };

  const next = () => {
    dispatch("next", null);
  };

  const prev = () => {
    dispatch("prev", null);
  };

  const keyPressed = (event) => {
    switch (event.key) {
      case "Escape":
        closeModal();
        break;
      case "ArrowRight":
        next();
        break;
      case "ArrowLeft":
        prev();
        break;
    }
  };
</script>

<div on:keydown={keyPressed} tabindex="-1">
  <div
    bind:this={modal}
    class="modal"
    role="button"
    on:click={closeModal}
    tabindex={0}
    on:keydown={() => {}}
  />
  <img class="expanded-image" {src} {alt} on:keydown={keyPressed} />

  <button class="prev nav-button" on:click={prev}>&lt</button>
  <button class="next nav-button" on:click={next}>&gt</button>
  <div class="text-box">
    <h2 class="title">{alt}</h2>
    <div class="parameter-box">
      <button class="tag-button" on:click={() => (showTags = !showTags)}>
        {#if showTags}
          Tags
        {:else}
          Params
        {/if}
      </button>
      <div class="parameter-text">
        {#if showTags}
          {#each tags as tag}
            <div class="tag">{tag}</div>
          {/each}
          <button on:click={() => console.log("add tag")} class="tag add-tag"
            >+</button
          >
        {:else}
          {parameterText}
        {/if}
      </div>
    </div>
  </div>
</div>

<style>
  .modal {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    backdrop-filter: blur(5px);
    background-color: rgba(0, 0, 0, 0.5);
    z-index: 1;
  }

  .expanded-image {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    max-width: 80%;
    max-height: 80%;
    z-index: 2;
  }
  .text-box {
    position: fixed;
    bottom: 0;
    left: 0;
    padding: 1rem;
    z-index: 2;
    color: #fff;
    width: calc(100% - 50px);
  }

  .next,
  .prev {
    position: fixed;
    top: 0;
    z-index: 2;
    height: 100%;
    display: flex;
    align-items: center;
    width: clamp(10%, 100px, 20%);
  }

  .next:hover,
  .prev:hover {
    background-color: rgba(0, 0, 0, 0.5);
  }

  .nav-button {
    background-color: transparent;
    border: none;
    color: #fff;
    font-size: 2em;
    cursor: pointer;
  }

  .next {
    right: 0;
    padding-right: 5px;
  }
  .prev {
    left: 0;
    padding-left: 5px;
  }

  .title {
    max-height: 1.5rem;
    padding: 0;
  }

  .parameter-box {
    display: flex;
    flex-direction: row;
    gap: 0.5rem;
  }
  .parameter-text {
    background-color: rgba(0, 0, 0, 0.5);
    padding: 1em;
    border-radius: 1em;
    height: 3rem;
    overflow-y: scroll;
    scrollbar-width: thin;
    padding: 0.25rem 0.75rem;
    width: 100%;
  }

  .tags {
    position: fixed;
    top: 0;
    right: 50%;
    z-index: 2;
    padding: 1rem;
  }

  .tag-button {
    background-color: rgba(0, 0, 0, 0.75);
    height: auto;
    width: 10ch;
    border: none;
    color: #fff;
    font-size: 1em;
    cursor: pointer;
    padding: 0.5em;
    border-radius: 1em;
    margin-bottom: 0.5em;
  }

  .tag {
    display: inline-block;
    border-radius: 25px;
    border: 1px solid black;
    padding: 0.25em 0.5em;
    margin-right: 0.5em;
    margin-bottom: 0.5em;
    text-align: center;
  }

  .add-tag {
    background-color: rgba(0, 0, 0, 0);
    color: #fff;
    min-width: 1em;
    font-size: 1.25rem;
    font-weight: 500;
    font-family: inherit;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }
</style>
