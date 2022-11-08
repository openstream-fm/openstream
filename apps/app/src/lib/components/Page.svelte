<script lang="ts">
  export let key: string | number | null = null;
  import { fly } from "svelte/transition";
</script>

<style>
  .page {
    padding: 1rem;
  }
</style>

{#key key}
  <div class="page" in:fly={{ duration: 250, x: -25 }}>
    <slot />
  </div>
{/key}