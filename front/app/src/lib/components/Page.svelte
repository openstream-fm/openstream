<script lang="ts">
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

<div class="page" in:fly={{ duration: 250, x: -25 }}>
  <slot />
</div>
