<script lang="ts">
  import { page } from "$app/stores";
  
  export let drawer_fixed_open: boolean;
  export let close_drawer_fixed: () => void;
  export let open_drawer_fixed: () => void;

  import Icon from "$share/Icon.svelte";
  import { ripple } from "$share/ripple";
	import { mdiMenu } from "@mdi/js";
  import TopUser from "./TopUser.svelte";

  const toggle_drawer = () => drawer_fixed_open ? close_drawer_fixed() : open_drawer_fixed();
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
    flex: 1;
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
    width: var(--top-h);
    flex: none;
    color: #333;
    transition: background-color 150ms ease;
    justify-self: flex-start;
  }

  .drawer-toggle:hover {
    background-color: rgba(0,0,0,0.05);
  }

  /* .station {
    display: flex;
    flex-direction: row;
    align-items: center;
    flex-shrink: 1;
  }

  .station-pic {
    width: 2.75rem;
    height: 2.75rem;
    flex: none;
    background-size: contain;
    background-position: center;
    background-repeat: no-repeat;
    border-radius: 0.25rem;
  }

  .station-name {
    flex-shrink: 1;
    margin-inline-start: 0.4rem;
    align-self: center;
    margin-inline-start: 1rem;
    display: flex;
  }

  .station-name-ellipsis {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    flex: 1;
    min-width: 0;
  } */

  @media screen and (max-width: 900px) {
    .drawer-toggle {
      display: flex;
    }
  } 

  /* @media screen and (max-width: 460px) {
    .station-name {
      display: none;
    }
  } */
</style>

<div class="top">
  <div class="box">
    <button class="drawer-toggle ripple-container" use:ripple aria-label="Toggle drawer" on:click={toggle_drawer}>
      <Icon d={mdiMenu} />
    </button>

    <!--
    {#if $page.data.station}
      <a class="na station" href="/accounts/{$page.data.station.account_id}/stations/{$page.data.station._id}">
        <div
          class="station-pic"
          style="background-image: url({$page.data.config.storage_public_url}/station-pictures/webp/128/{$page.data.station.picture_id}.webp)"
        />
        <span class="station-name">
          <span class="station-name-ellipsis">
            {$page.data.station.name}
          </span>  
        </span>
      </a>
    {/if}
    -->
    <TopUser />
  </div>
</div>