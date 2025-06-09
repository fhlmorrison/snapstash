<script lang="ts">
  import { createEventDispatcher } from "svelte";

  interface Props {
    value?: string;
    onSubmit?: (value: string) => void;
  }

  let { value = $bindable(""), onSubmit }: Props = $props();
  const dipatch = createEventDispatcher();

  const submit = () => {
    onSubmit?.(value);
  };
</script>

<div
  onkeypress={(e) => {
    if (e.key == "Enter") submit();
  }}
>
  <input type="text" bind:value />
  <button onclick={submit}>Search</button>
</div>
