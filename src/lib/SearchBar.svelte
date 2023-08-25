<script>
    import { createEventDispatcher } from "svelte";

    export let value = "";
    const dipatch = createEventDispatcher();

    const submit = () => {
        dipatch("search", value);
    };
</script>

<div
    on:keypress={(e) => {
        if (e.key == "Enter") submit();
    }}
>
    <input type="text" bind:value />
    <button on:click={submit}>Search</button>
</div>
