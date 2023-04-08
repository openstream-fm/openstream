<script lang="ts">
	import { close } from "$lib/components/Player/player";
	import { onMount } from "svelte";

  onMount(() => {
    return () => close()
  })
</script>

<slot />