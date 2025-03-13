<script lang="ts">
  import FaExpand from "svelte-icons/fa/FaExpand.svelte";
  import FaPlay from "svelte-icons/fa/FaPlay.svelte";
  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();

  export let src: string = "";
  export let path: string = "";
  export let name: string = "";
  export let tabindex: number = 0;
  export let index: number = 0;

  export let onClick = () => {
    // openImageDialogue();
    dispatch("select", index);
  };

  let video;

  const play = () => {
    if (!video) return;

    if (video.paused) {
      video.play();
    } else {
      video.pause();
    }
  };

  $: isVideo = path.includes(".mp4");

  const expand = () => dispatch("expand", index);

  const keyPressed = (event) => {
    if (event.key === "Enter") {
      expand();
    }
  };
</script>

<div
  class="image-square"
  role="button"
  on:click={onClick}
  on:keydown={keyPressed}
  {tabindex}
  on:focus={() => {
    dispatch("select", index);
  }}
>
  <div class="expand" on:click={expand}>
    <FaExpand />
  </div>

  {#if isVideo}
    <video {src} bind:this={video} muted />
    <div class="overlay" on:click={play}>
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
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    border: 1px solid black;
    cursor: pointer;
    position: relative;
  }

  .image-square:focus {
    outline: solid 2px blue;
  }

  .image-square:hover {
    opacity: 0.9;
    border: solid 1px #396cd8;
  }

  .image-square:hover .expand {
    display: flex;
    background-color: rgba(255, 255, 255, 0.5);
  }

  .image-square img {
    width: 100%;
    height: 100%;
    aspect-ratio: 1 / 1;
    object-fit: cover;
  }

  .image-square video {
    width: 100%;
    height: 100%;
    aspect-ratio: 1 / 1;
    object-fit: cover;
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
