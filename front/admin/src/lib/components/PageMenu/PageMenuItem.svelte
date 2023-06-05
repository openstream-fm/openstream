<script lang="ts">
  export let href: string | null | undefined = undefined;
  export let on_click: () => void = () => {}
  export let icon: string | null = null;
  
  import Icon from "$share/Icon.svelte";
  import { ripple } from "$share/ripple";
</script>

<style>
  .menu-item {
    padding: 0.75rem;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-start;
    transition: background-color 200ms ease;
  }

  .menu-item:hover {
    background-color: rgba(0,0,0,0.05);
    border-radius: 0.25rem;
  }

  .icon {
    font-size: 1.25rem;
    display: flex;
    margin-inline-end: 0.75rem;
  }
</style>

{#if href != null}
  <a href={href} on:click={on_click} class="na menu-item ripple-container" use:ripple>
    {#if icon != null}
      <div class="icon">
        <Icon d={icon} />
      </div>
      <slot />
    {:else}
      <slot />
    {/if}
  </a>
{:else}
  <button on:click={on_click} class="menu-item ripple-container" use:ripple>
    {#if icon != null}
      <div class="icon">
        <Icon d={icon} />
      </div>
      <slot />
    {:else}
      <slot />
    {/if}
  </button>
{/if}