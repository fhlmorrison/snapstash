<script lang="ts">
  interface Props {
    tag: string;
    onMainClick: () => void; // Click for the main tag area (green hover)
    onRemoveClick: () => void; // Click for the remove 'X' area (red hover)
  }

  let { tag, onMainClick, onRemoveClick }: Props = $props();
</script>

<div class="tag-pill">
  <div class="tag-main" onclick={onMainClick}>
    {tag}
  </div>
  <div class="tag-remove" onclick={onRemoveClick}>&times;</div>
</div>

<style>
  .tag-pill {
    position: relative; /* Essential for positioning the 'X' absolutely within it */
    display: inline-flex; /* Use inline-flex for the pill itself */
    align-items: center; /* Vertically center content */
    border-radius: 25px;
    border: 1px solid black;
    padding: 0.25em 0.5em; /* Padding for the main text area */
    margin-right: 0.5em;
    margin-bottom: 0.5em;
    color: #fff;
    overflow: hidden; /* Keep rounded corners clean */
    cursor: pointer; /* Indicate the main area is clickable */
  }

  .tag-main {
    background-color: rgba(0, 0, 0, 0); /* Default transparent */
    transition: background-color 0.2s ease-in-out;
  }

  .tag-pill:hover {
    background-color: rgba(0, 128, 0, 0.5); /* Green hover for the main area */
  }

  .tag-remove {
    position: absolute; /* Take it out of normal flow */
    top: 0;
    right: 0;
    height: 100%; /* Make it span the full height of the pill */
    width: 0; /* Initially hidden */
    display: flex; /* Use flex to center the 'X' */
    align-items: center;
    justify-content: center;
    background-color: rgba(
      255,
      0,
      0,
      0.5
    ); /* Red background for the 'X' area */
    transition:
      width 0.2s ease-in-out,
      background-color 0.2s ease-in-out,
      opacity 0.2s ease-in-out;
    opacity: 0; /* Start fully transparent */
    pointer-events: none; /* Make it unclickable when hidden */
    border-top-right-radius: 25px;
    border-bottom-right-radius: 25px;
  }

  .tag-pill:hover .tag-remove {
    width: 1.5em; /* Adjust this value to control the width of the 'X' button */
    opacity: 1; /* Fade in */
    pointer-events: auto; /* Make it clickable when visible */
  }

  .tag-remove:hover {
    background-color: rgba(255, 0, 0, 0.7); /* Darker red on direct hover */
  }
</style>
