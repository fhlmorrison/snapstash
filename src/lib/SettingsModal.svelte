<script lang="ts">
  import { configStore } from "./config.svelte";
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
    background-color: #fefefe;
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
</style>
