<script lang="ts">
  export let open = false;

  import Icon from "$share/Icon.svelte";
	import { click_out } from "$share/actions";
  import { ripple } from "$share/ripple";
	import { logical_fly } from "$share/transition";
	import { mdiDotsVertical } from "@mdi/js";

  const on_click_out = () => {
    setTimeout(() => open = false, 1);
  } 
</script>

<style>
  .page-menu {
    display: flex;
    flex-direction: column;
    position: relative;
  }

  .btn {
    width: 3rem;
    height: 3rem;
    flex: none;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.75rem;
    transition: background-color 200ms ease;
    border-radius: 50%;
  }

  .btn:hover {
    background-color: rgba(0,0,0,0.05);
  }

  .menu {
    background: #fff;
    box-shadow: var(--some-shadow);
    position: absolute;
    inset-block-start: 100%;
    inset-inline-end: 0;
    width: min(calc(100vh - 4rem), 15rem);
    padding: 0.25rem;
    border-radius: 0.25rem;
    display: flex;
    flex-direction: column;
    align-items: stretch;
  }
</style>

<div class="page-menu">
  <button class="btn ripple-container" aria-label="Menu" use:ripple on:click={() => { open = !open }}>
    <Icon d={mdiDotsVertical} />
  </button>
  {#if open}
    <div class="menu" transition:logical_fly={{ x: 5, y: -10, duration: 200 }} use:click_out={on_click_out}>
      <slot {close} />
    </div>
  {/if}
</div>