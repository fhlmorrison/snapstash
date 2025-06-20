<script lang="ts">
  import {
    configStore,
    MAX_MIN_IMAGE_WIDTH,
    MIN_IMAGE_WIDTH_STEP,
    MIN_MIN_IMAGE_WIDTH,
  } from "./config.svelte";
</script>

{#if configStore.isModalOpen}
  <div class="modal">
    <div class="modal-content">
      <span class="close" on:click={() => (configStore.isModalOpen = false)}
        >&times;</span
      >
      <h2>Settings</h2>
      <div>
        <input
          type="checkbox"
          bind:checked={configStore.useMasonry}
          name="useMasonry"
        />
        <label for="useMasonry">Use Masonry Layout</label>
      </div>
      <div>
        <input
          type="checkbox"
          bind:checked={configStore.useREButton}
          name="useMasonry"
        />
        <label for="useMasonry">Use RE button</label>
      </div>
      <div>
        <input
          type="checkbox"
          bind:checked={configStore.useLargeImages}
          name="useLargeIcons"
        />
        <label for="useLargeIcons">Use large icons</label>
      </div>
      <div>
        <input
          type="checkbox"
          bind:checked={configStore.loopVideos}
          name="loopVideo"
        />
        <label for="loopVideo">Loop video</label>
      </div>
      <div>
        <input
          min={MIN_MIN_IMAGE_WIDTH}
          max={MAX_MIN_IMAGE_WIDTH}
          step={MIN_IMAGE_WIDTH_STEP}
          type="range"
          bind:value={configStore.minImageWidth}
          name="minImageWidth"
        />
        <label for="minImageWidth"
          >{configStore.minImageWidth}px Min image width</label
        >
      </div>

      <!-- Save and load buttons at the bottom -->
      <div>
        <button on:click={() => configStore.saveSettings()}
          >Save Settings</button
        >
        <button on:click={() => configStore.loadSettings()}
          >Load Settings</button
        >
      </div>
    </div>
  </div>
{/if}

<style>
  .modal {
    display: flex;
    position: fixed;
    z-index: 1000;
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;
    overflow: auto;
    background-color: rgba(0, 0, 0, 0.5);
  }

  .modal-content {
    background-color: var(--bg-primary);
    margin: 15% auto;
    padding: 20px;
    border: 1px solid #888;
    width: 80%;
    max-width: 500px;
    border-radius: 20px;
  }

  .close {
    color: #aaa;
    float: right;
    font-size: 28px;
    font-weight: bold;
  }

  .close:hover,
  .close:focus {
    color: black;
    text-decoration: none;
    cursor: pointer;
  }

  input[type="range"] {
    overflow: hidden;
    height: 20px;
    -webkit-appearance: none;
    background-color: #ddd;
    padding: 0;
    border-radius: 20px;
  }

  input[type="range"]::-webkit-slider-thumb {
    width: 20px;
    -webkit-appearance: none;
    height: 20px;
    cursor: ew-resize;
    background: #fff;
    border-radius: 50%;
    transition: box-shadow 0.2s ease-in-out;
    position: relative;
    background: #fff;
    box-shadow:
      -340px 0 0 330px #1597ff,
      inset 0 0 0 3px;
  }

  input[type="range"]::-webkit-slider-runnable-track {
    height: 20px;
    -webkit-appearance: none;
    color: #444;
    /* margin-top: -1px; */
    transition: box-shadow 0.2s ease-in-out;
    border-radius: 20px;
  }
</style>
