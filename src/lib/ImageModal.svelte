<script lang="ts">
    import { readParameters } from "./loadassets";
    import { onMount } from "svelte";
    import { createEventDispatcher } from "svelte";
    export let src = "";
    export let alt = "";
    export let path = "";
    const dispatch = createEventDispatcher();

    let parameterText = "";

    onMount(async () => {
        parameterText = await readParameters(path);
    });

    let modal;
    onMount(() => {
        modal.focus();
    });

    let closeModal = (): void => {
        dispatch("close", null);
    };

    const keyPressed = (event) => {
        if (event.key === "Escape") {
            closeModal();
        }
    };
</script>

<div
    bind:this={modal}
    class="modal"
    role="button"
    on:click={closeModal}
    on:keydown={keyPressed}
    tabindex={0}
/>
<img class="expanded-image" {src} {alt} on:keydown={keyPressed} />
<div class="text-box">
    <h2>{alt}</h2>
    <div>{parameterText}</div>
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
        padding: 1em;
        z-index: 2;
        color: #fff;
    }
</style>
