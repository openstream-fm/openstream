<script lang="ts">
  export let fixed_open: boolean;
  export let close_drawer_fixed: () => void;
  export let open_drawer_fixed: () => void;

  import { page } from "$app/stores";

  const HTML_OPEN_CLASSNAME = "station-drawer-fixed-open";

  const toggle = () => fixed_open ? close_drawer_fixed() : open_drawer_fixed();

  import DrawerItem from "./DrawerItem.svelte";
  import { 
    mdiViewDashboardOutline,
    mdiRadioTower,
  	mdiMenu,
		mdiAccountMultipleOutline,
		mdiPoll,
		mdiClose,
		mdiAccountOutline,
		mdiAccount,
		mdiCurrencyUsd,
		mdiShieldAccountOutline,
		mdiConnection,
  } from "@mdi/js";
	import { onMount } from "svelte";
	import Icon from "$share/Icon.svelte";
	import { ripple } from "$share/ripple";
	import { fade } from "svelte/transition";
	import { browser } from "$app/environment";
  // @ts-ignore
  import logo from "$share/img/logo-trans-128.png?w=64&format=webp";

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
    height: var(--top-h);
    font-weight: var(--font-bold);;
    font-size: 1.5rem;
    display: flex;
    align-items: center;
    justify-content: flex-start;
  }

  .toggle {
    flex: none;
    display: none;
    width: 4rem;
    height: 100%;
    font-size: 1.5rem;
    align-items: center;
    justify-content: center;
    margin-inline-end: -1.5rem;
    transition: background-color 200ms ease;
  }
  
  .toggle:hover {
    background: rgba(0,0,0,0.05);
  }

  .logo {
    margin-inline-start: 1rem;
    display: flex;
    flex-direction: row;
    align-items: center;
  }

  .logo-icon {
    background-size: contain;
    background-position: center;
    background-repeat: no-repeat;
    font-size: 1.5rem;
    width: 2.5rem;
    height: 2.5rem;
    margin-inline-end: 0.5rem;
  }

  .logo-text {
    font-size: 1.5rem;
    font-weight: 700;
  }

  .inner {
    position: sticky;
    top: 0;
    height: 100vh;
    /*transition: height 350ms cubic-bezier(0.85, 0, 0.15, 1); /* expoInOut: same as player */
    display: flex;
    flex-direction: column;
  }

  .player-open .inner {
    height: calc(100vh - var(--player-h));
  }

  .drawer-overlay {
    display: none;
    position: fixed;
    inset: 0;
    background: rgba(0,0,0,0.5);
    z-index: calc(var(--z-drawer-fixed) - 1);
  }

  @media screen and (max-width: 900px) {

    .drawer {
      position: fixed;
      z-index: var(--z-drawer-fixed);
    }

    .inner {
      height: 100vh !important;
    }

    .drawer-overlay {
      display: block;
    }

    .toggle {
      display: flex;
    }

    .logo-icon {
      width: 2rem;
      height: 2rem;
      font-size: 1.35rem;
    }

    .logo-text {
      font-size: 1.35rem;
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
      <button class="toggle ripple-container" use:ripple aria-label="Toogle drawer" on:click={close_drawer_fixed}>
        <Icon d={mdiMenu} />
      </button>
      <div class="logo">
        <div class="logo-icon" style="background-image: url({logo})">
        </div>
        <div class="logo-text">
          openstream
        </div>
      </div>
    </div>

    <div class="links">
      <DrawerItem href="/" label="Dashboard" icon={mdiViewDashboardOutline} on_click={close_drawer_fixed} />
      <DrawerItem href="/admins" label="Admins" icon={mdiShieldAccountOutline} on_click={close_drawer_fixed} />
      <DrawerItem href="/users" label="Users" icon={mdiAccountMultipleOutline} on_click={close_drawer_fixed} />
      <DrawerItem href="/accounts" label="Accounts" icon={mdiAccountOutline} on_click={close_drawer_fixed} />
      <DrawerItem href="/stations" label="Stations" icon={mdiRadioTower} on_click={close_drawer_fixed} />
      <DrawerItem href="/plans" label="Plans" icon={mdiCurrencyUsd} on_click={close_drawer_fixed} />
      <DrawerItem href="/listeners" label="Listeners" icon={mdiConnection} on_click={close_drawer_fixed} />
      <DrawerItem href="/analytics" label="Analytics" icon={mdiPoll} on_click={close_drawer_fixed} />
    </div>
  </div>
</div>