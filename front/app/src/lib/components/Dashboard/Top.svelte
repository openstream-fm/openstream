<script lang="ts">
  import { page } from "$app/stores";
  
  export let drawer_fixed_open: boolean;
  export let close_drawer_fixed: () => void;
  export let open_drawer_fixed: () => void;

  import Icon from "$share/Icon.svelte";
  import { ripple } from "$share/ripple";
	import { mdiMenu } from "@mdi/js";

  const toggle_drawer = () => drawer_fixed_open ? close_drawer_fixed() : open_drawer_fixed();

  import TopUser from "./TopUser.svelte";
	import { fly } from "svelte/transition";
</script>

<style>
  .top {
    position: sticky;
    top: 0;
    height: var(--top-h);
    z-index: var(--z-top);
    display: flex;
    flex-direction: column;
    background: rgba(var(--bg-gray-rgb), 0.875);
    backdrop-filter: blur(2px);
  }

  .box {
    position: relative;
    flex: none;
    box-shadow: 0 4px 20px 0 rgb(0 0 0 / 5%);
    z-index: calc(var(--z-top) + 1);
    background: #fff;
    display: flex;
    flex-direction: row;
  }

  .drawer-toggle {
    display: none;
    align-items: center;
    justify-content: center;
    font-size: 1.75rem;
    flex: none;
    width: var(--top-height);
    color: #333;
    transition: background-color 150ms ease;
    justify-self: flex-start;
  }

  .drawer-toggle:hover {
    background-color: rgba(0,0,0,0.05);
  }

  .station-top-name {
    flex: 1;
    padding: 1rem;
    align-self: center;
    margin-inline-start: 1rem;
    display: flex;
  }

  .station-top-name-ellipsis {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    flex: 1;
    min-width: 0;
  }

  @media screen and (max-width: 900px) {
    .drawer-toggle {
      display: flex;
    }

    .station-top-name {
      margin-inline-start: -1rem;
    }
  } 
</style>

<div class="top">
  <div class="box">
    <button class="drawer-toggle ripple-container" use:ripple aria-label="Toggle drawer" on:click={toggle_drawer}>
      <Icon d={mdiMenu} />
    </button>
    {#if $page.data.station}
      <a class="na station-top-name ripple-container" href="/accounts/{$page.data.station.account_id}/stations/{$page.data.station._id}" use:ripple in:fly={{ duration: 300, x: -25 }}>
        <div class="station-top-name-ellipsis">
          {$page.data.station.name}
        </div>
      </a>
    {/if}
    <TopUser />
  </div>
</div>