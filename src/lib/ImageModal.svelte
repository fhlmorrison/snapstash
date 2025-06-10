<script lang="ts">
  import { readParameters, readTags } from "./images.svelte";
  import { onMount } from "svelte";
  import TagAdder from "./TagAdder.svelte";
  import { removeTagFromImage, tagImage } from "./tags";
  interface Props {
    src?: string;
    alt?: string;
    path?: string;
    onNext?: () => void;
    onPrev?: () => void;
    onClose?: () => void;
  }

  let {
    src = "",
    alt = "",
    path = "",
    onClose,
    onNext,
    onPrev,
  }: Props = $props();

  let parameterText = $state("");

  const updateParameterText = async (pth: string) => {
    if (pth === "") {
      parameterText = "";
      return;
    }
    parameterText = (await readParameters(pth)) ?? "";
  };

  let showTags = $state(true);

  let tags: string[] = $state([]);

  const setTags = async (src: string) => {
    tags = (await readTags(src)) ?? [];
  };

  let modal: HTMLDivElement | undefined = $state();
  onMount(() => {
    modal?.focus();
  });

  const keyPressed = (event: KeyboardEvent) => {
    switch (event.key) {
      case "Escape":
        onClose?.();
        break;
      case "ArrowRight":
        onNext?.();
        break;
      case "ArrowLeft":
        onPrev?.();
        break;
    }
  };

  let showAddTag = $state(false);
  $effect(() => {
    updateParameterText(path);
    setTags(path);
  });
  let isVideo = $derived(
    path.includes(".mp4") ||
      path.includes(".webm") ||
      src.includes(".mp4") ||
      src.includes(".webm")
  );
</script>

<div onkeydown={keyPressed} tabindex="-1">
  <div
    bind:this={modal}
    class="modal"
    role="button"
    onclick={onClose}
    tabindex={0}
    onkeydown={() => {}}
  ></div>
  {#if isVideo}
    <video class="expanded-image" {src} onkeydown={keyPressed} controls></video>
  {:else}
    <img class="expanded-image" {src} {alt} onkeydown={keyPressed} />
  {/if}
  <!-- <img class="expanded-image" {src} {alt} on:keydown={keyPressed} /> -->

  <button class="prev nav-button" onclick={onPrev}>&lt</button>
  <button class="next nav-button" onclick={onNext}>&gt</button>
  <div class="text-box">
    <h2 class="title">{alt}</h2>
    <div class="parameter-box">
      <button class="tag-button" onclick={() => (showTags = !showTags)}>
        {#if showTags}
          Tags
        {:else}
          Params
        {/if}
      </button>
      {#if showTags}
        <div class="tag-text">
          {#each tags as tag}
            <button
              class="tag remove-tag"
              onclick={() => {
                removeTagFromImage(path, tag);
                setTags(path);
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
                  tagImage(path, tag);
                  setTags(path);
                }}
              />
            </div>
          {:else}
            <button onclick={() => (showAddTag = true)} class="tag add-tag"
              >+</button
            >
          {/if}
        </div>
      {:else}
        <div class="parameter-text">
          {parameterText}
        </div>
      {/if}
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
    pointer-events: none;
  }

  .next,
  .prev {
    position: fixed;
    top: 0;
    z-index: 2;
    height: 50%;
    display: flex;
    align-items: center;
    top: 50%;
    transform: translate(0, -50%);
    width: clamp(10%, 100px, 20%);
    background-color: transparent;
    border: none;
    color: #fff;
    font-size: 2em;
    cursor: pointer;
  }

  .next:hover,
  .prev:hover {
    background-color: rgba(0, 0, 0, 0.5);
  }

  .next {
    right: 0;
    padding-right: 5px;
    border-radius: 100% 0 0 100%;
  }
  .prev {
    left: 0;
    padding-left: 5px;
    border-radius: 0 100% 100% 0;
  }

  .title {
    max-height: 1.5rem;
    pointer-events: stroke;
    padding: 0;
    width: fit-content;
  }

  .parameter-box {
    display: flex;
    flex-direction: row;
    gap: 0.5rem;
    pointer-events: all;
  }
  .parameter-text,
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

  .parameter-text {
    overflow-y: scroll;
    scrollbar-width: thin;
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
