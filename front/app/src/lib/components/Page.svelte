<script lang="ts">
  export let compact: boolean = false;
  import { fly } from "svelte/transition";
</script>

<style>
  .page:not(.compact) {
    padding: 2rem;
  }

  @media screen and (max-width: 600px) {
    .page:not(.compact) {
      padding: 2rem 1rem;
    }
  }
</style>

<div class="page" class:compact in:fly={{ duration: 250, x: -25 }}>
  <slot />
</div>
