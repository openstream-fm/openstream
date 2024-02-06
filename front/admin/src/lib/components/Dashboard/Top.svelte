<script lang="ts">
  export let toggle_drawer: () => void;
  export let with_drawer: boolean;

  import Icon from "$share/Icon.svelte";
  import { ripple } from "$share/ripple";
	import { mdiMenu } from "@mdi/js";

  import TopUser from "./TopUser.svelte";

  // @ts-ignore
  import logo from "$share/img/logo-trans-128.png?w=40&format=webp";
  import { locale } from "$lib/locale";
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
    flex: none;
    align-items: center;
    justify-content: center;
    font-size: 1.75rem;
    width: var(--top-h);
    color: #333;
    transition: background-color 150ms ease;
    justify-self: flex-start;
  }

  .drawer-toggle:hover {
    background-color: rgba(0,0,0,0.05);
  }

  .logo {
    margin-inline-start: 1.5rem;
    display: flex;
    flex-direction: row;
    align-items: center;
  }

  .with-drawer .drawer-toggle {
    display: flex;
  }

  .with-drawer .logo {
    margin-inline-start: 0;
  }

  @media screen and (max-width: 540px) {
    .logo {
      display: none;
    }
  }

  .logo-icon {
    background-size: contain;
    background-position: center;
    background-repeat: no-repeat;
    width: 2rem;
    height: 2rem;
    margin-inline-end: 0.75rem;
  }

  .logo-text {
    font-size: 1.5rem;
    font-weight: var(--font-bold);
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

  /* @media screen and (max-width: 460px) {
    .station-name {
      display: none;
    }
  } */
</style>
<div class="top" class:with-drawer={with_drawer}>
  <div class="box">
    <button class="drawer-toggle ripple-container" use:ripple aria-label="Toggle drawer" on:click={toggle_drawer}>
      <Icon d={mdiMenu} />
    </button>

    <div class="logo">
      <div class="logo-icon" style="background-image: url({logo})">
      </div>
      <div class="logo-text">
        {$locale.logo_text}
      </div>
    </div>

    <!--
    {#if $page.data.station}
      <a class="na station" href="/accounts/{$page.data.station.account_id}/stations/{$page.data.station._id}">
        <div
          class="station-pic"
          style="background-image: url({$page.data.config.storage_public_url}/station-pictures/webp/128/{$page.data.station.picture_id}.webp?v={STATION_PICTURES_VERSION})"
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