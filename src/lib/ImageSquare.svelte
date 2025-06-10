<script lang="ts">
  import FaExpand from "svelte-icons/fa/FaExpand.svelte";
  import FaPlay from "svelte-icons/fa/FaPlay.svelte";
  import { configStore } from "./config.svelte";

  export interface onSelectEvent {
    index: number;
    shiftKey: boolean;
    ctrlKey: boolean;
  }

  interface Props {
    src?: string;
    path?: string;
    name?: string;
    tabindex?: number;
    index?: number;
    selected?: boolean;
    onExpand?: (index: number) => void;
    onSelect?: (e: onSelectEvent) => void;
  }

  let {
    src = "",
    path = "",
    name = "",
    tabindex = 0,
    index = 0,
    selected = false,
    onExpand,
    onSelect,
  }: Props = $props();

  let onClick = (e: { shiftKey: any; ctrlKey: any }) => {
    // openImageDialogue();
    onSelect?.({ index, shiftKey: e.shiftKey, ctrlKey: e.ctrlKey });
  };

  let video: HTMLVideoElement | undefined = $state();

  const play = () => {
    if (!video) return;

    if (video.paused) {
      video.play();
    } else {
      video.pause();
    }
  };

  let isVideo = $derived(path.includes(".mp4") || path.includes(".webm"));

  const expand = () => onExpand?.(index);

  const keyPressed = (event: KeyboardEvent) => {
    if (event.key === "Enter") {
      expand();
    }
  };
</script>

<div
  class={`image-square ${selected ? "selected" : ""} ${configStore.useMasonry ? "masonry" : "grid"}`}
  role="button"
  onclick={onClick}
  onkeydown={keyPressed}
  {tabindex}
>
  <div class="expand" onclick={expand}>
    <FaExpand />
  </div>

  {#if isVideo}
    <video {src} bind:this={video} muted></video>
    <div class="overlay" onclick={play}>
      <FaPlay />
    </div>
  {:else}
    <img loading="lazy" {src} alt={name} />
  {/if}

  <!-- <img loading="lazy" {src} alt={name} /> -->
</div>

<style>
  .image-square {
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    border: 2px solid black;
    cursor: pointer;
    position: relative;
    box-sizing: border-box;
    /* background: radial-gradient(
      circle at center,
      rgba(0, 0, 0, 0.9),
      rgba(255, 255, 255, 0.5)
    ); */
  }

  .image-square.grid {
    height: 100%;
  }

  .image-square.masonry {
    height: fit-content;
  }

  .image-square.selected {
    outline: solid 2px #72abff;
    border: 2px solid #72abff;
  }

  /* .image-square:focus {
    outline: solid 2px blue;
  } */

  .image-square:hover {
    opacity: 0.9;
    border: solid 2px #396cd8;
  }

  .image-square:hover .expand {
    display: flex;
    background-color: rgba(255, 255, 255, 0.5);
  }

  .image-square img {
    width: 100%;
    object-fit: cover;
    /* object-fit: contain; */
  }

  .image-square.grid img {
    height: 100%;
    aspect-ratio: 1 / 1;
  }

  .image-square video {
    width: 100%;
    object-fit: cover;
    /* object-fit: contain; */
  }

  .image-square.grid video {
    height: 100%;
    aspect-ratio: 1 / 1;
  }

  .expand {
    position: absolute;
    top: 0;
    right: 0;
    width: 2rem;
    height: 2rem;
    display: none;
    justify-content: center;
    align-items: center;
    z-index: 10;
  }

  .overlay {
    position: absolute;
    top: 0;
    left: 0;
    width: 2rem;
    height: 2rem;
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 1;
    background-color: rgba(255, 255, 255, 0.5);
  }

  .expand:hover {
    background-color: rgba(0, 0, 0, 0.5);
  }
</style>
