<script lang="ts">
  export let stations: import("$server/defs/api/stations/GET/Output").Output;
  export let station: import("$server/defs/api/stations/[station]/GET/Output").Output["station"];
  export let user: import("$server/defs/api/users/[user]/GET/Output").Output["user"];
  export let fixed_open: boolean;

  export let close_drawer_fixed: () => void;
  export let open_drawer_fixed: () => void;

  const HTML_OPEN_CLASSNAME = "station-drawer-fixed-open";

  const toggle = () => fixed_open ? close_drawer_fixed() : open_drawer_fixed();

  import DrawerItem from "./DrawerItem.svelte";
  import { 
    mdiViewDashboardOutline,
    mdiMultimedia,
    mdiRadioTower,
    mdiPoll,
    mdiAccountOutline,
    mdiShieldAccountOutline,
  	mdiMenu,
  } from "@mdi/js";
	import { onMount } from "svelte";
	import Icon from "$share/Icon.svelte";
	import { ripple } from "$share/ripple";
	import { fade } from "svelte/transition";
	import { browser } from "$app/environment";

  $: if(browser) {
    document.documentElement.classList[fixed_open ? "add" : "remove"](HTML_OPEN_CLASSNAME);
  }

  onMount(() => {
    
    let media = window.matchMedia("screen and (max-width: 900px)");
    
    media.onchange = () => {
      if(!media.matches) close_drawer_fixed();
    }

    return () => {
      media.onchange = null;
      document.documentElement.classList.remove(HTML_OPEN_CLASSNAME);
    }
  })
</script>

<style>
  .drawer {
    position: relative;
    width: var(--drawer-width);
    flex: none;
    background: #fff;
    box-shadow: 0 0 15px 0 rgb(0 0 0 / 10%);
    z-index: var(--z-drawer-wide);
    transition: margin 300ms ease;
  }

  .top {
    height: var(--top-height);
    font-weight: 600;
    font-size: 1.5rem;
    display: flex;
    align-items: center;
    justify-content: flex-start;
  }

  .logo {
    margin-inline-start: 1.5rem;
  }

  .inner {
    position: sticky;
    top: 0;
    height: 100vh;
  }

  .toggle {
    display: none;
    align-items: center;
    justify-content: center;
    font-size: 1.75rem;
    width: var(--top-height);
    height: var(--top-height);
    cursor: pointer;
    user-select: none;
    transition: background-color 150ms ease;
    margin-inline-end: -1.25rem;
  }

  .toggle:hover {
    background: rgba(0,0,0,0.05);
  }

  .drawer-overlay {
    display: none;
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0,0,0,0.5);
    z-index: calc(var(--z-drawer-fixed) - 1);
  }

  @media screen and (max-width: 900px) {

    .drawer {
      position: fixed;
      z-index: var(--z-drawer-fixed);
    }

    .drawer-overlay {
      display: block;
    }

    .toggle {
      display: flex;
    }

    .drawer:not(.fixed-open) {
      margin-inline-start: calc(var(--drawer-width) * -1);
      box-shadow: none;
    }
  }
</style>

{#if fixed_open}
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div class="drawer-overlay" transition:fade|local={{ duration: 250 }} on:click={close_drawer_fixed} />
{/if}

<div class="drawer" class:fixed-open={fixed_open}>
  <div class="inner">
    <div class="top">
      <button class="toggle ripple-container" use:ripple aria-label="Toggle drawer" on:click={toggle}>
        <Icon d={mdiMenu} />
      </button>
      <div class="logo">
        openstream
      </div>
    </div>

    <div class="links">
      <DrawerItem href="/stations/{station._id}" label="Dashboard" icon={mdiViewDashboardOutline} on_click={close_drawer_fixed} />
      <DrawerItem href="/stations/{station._id}/playlist" label="Playlist" icon={mdiMultimedia} on_click={close_drawer_fixed} />
      <!--
        <DrawerItem href="/stations/{station._id}/profile" label="Profile" icon={mdiAccountOutline} />
        <DrawerItem href="/station" label="Account" icon={mdiShieldAccountOutline} />
        <DrawerItem href="/stations" label="Stations" icon={mdiRadioTower} />
        <DrawerItem href="/analytics" label="Analytics" icon={mdiPoll} />
      -->
    </div>
  </div>
</div>