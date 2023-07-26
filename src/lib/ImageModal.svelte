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
        on:keydown={keyPressed}
    />
    <img class="expanded-image" {src} {alt} on:keydown={keyPressed} />
    <button class="prev" on:click={prev}>&lt</button>
    <button class="next" on:click={next}>&gt</button>
    <div class="text-box">
        <h2>{alt}</h2>
        <div>{parameterText}</div>
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
        padding: 1em;
        z-index: 2;
        color: #fff;
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

    button {
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
</style>
