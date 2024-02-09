<script lang="ts">
  export let locale: import("$server/locale/share/analytics/analytics.locale").AnalyticsLocale;
  import { crossfade, fade } from "svelte/transition";

  export let type: "stream" | "app" = "stream";

  export let [enter, leave] = crossfade({
    duration: 300,
    // @ts-ignore
    fallback: fade,
    intro: false,
  });
</script>

<style>
  .type-selector {
    display: flex;
    flex-direction: row;
    align-items: center;
  }

  .type-selector-item {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    border-radius: 10em;
  }

  .type-selector-btn {
    border-radius: inherit; 
    position: relative;
    z-index: 2;
    color: var(--blue);
    padding: 0.5rem 0.75rem;
  }

  .bg {
    position: absolute;
    inset: 0;
    border-radius: inherit;
    background: rgba(var(--blue-rgb), 0.1);
    z-index: 1;
  }
</style>

<div class="type-selector">
  <div class="type-selector-item" class:selected={type === "stream"}>    
    {#if type === "stream"}
      <div class="bg" in:enter={{ key: null }} out:leave={{ key: null }} />
    {/if}
    <button class="type-selector-btn" on:click={() => type = "stream"}>
      {locale.stream}
    </button>
  </div>
  <div class="type-selector-item" class:selected={type === "app"}>
    {#if type === "app"}
      <div class="bg" in:enter={{ key: null }} out:leave={{ key: null }} />
    {/if}
    <button class="type-selector-btn" on:click={() => type = "app"}>
      {locale.apps}
    </button>
  </div>
</div>