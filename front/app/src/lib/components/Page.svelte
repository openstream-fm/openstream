<script lang="ts">
  export let key: string | number | null = null;
  import { fly } from "svelte/transition";
</script>

<style>
  .page {
    padding: 2rem;
  }

  @media screen and (max-width: 600px) {
    .page {
      padding: 2rem 1rem;
    }
  }
</style>

{#key key}
  <div class="page" in:fly={{ duration: 250, x: -25 }}>
    <slot />
  </div>
{/key}