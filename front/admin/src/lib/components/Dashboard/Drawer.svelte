<script lang="ts">
  export let fixed_open: boolean;
  export let static_open: boolean;
  export let close: () => void;
  // svelte-ignore unused-export-let
  export let open: () => void;
  // svelte-ignore unused-export-let
  export let toggle: () => void;

  import { page } from "$app/stores";

  const HTML_OPEN_FIXED_CLASSNAME = "station-drawer-fixed-open";
  
  import DrawerItem from "./DrawerItem.svelte";
  import { 
    mdiViewDashboardOutline,
    mdiRadioTower,
  	mdiMenu,
		mdiAccountMultipleOutline,
		mdiPoll,
		mdiAccountOutline,
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
    document.documentElement.classList[fixed_open ? "add" : "remove"](HTML_OPEN_FIXED_CLASSNAME);
  }

  onMount(() => {
    
    let media = window.matchMedia("screen and (max-width: 900px)");
    
    media.onchange = () => {
      if(!media.matches) fixed_open = false;
    }

    return () => {
      media.onchange = null;
      document.documentElement.classList.remove(HTML_OPEN_FIXED_CLASSNAME);
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
    font-weight: var(--font-bold);
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
    width: 2rem;
    height: 2rem;
    margin-inline-end: 0.75rem;
  }

  .logo-text {
    font-size: 1.5rem;
    font-weight: var(--font-bold);
  }

  .inner {
    position: sticky;
    top: var(--top-h);
    height: calc(100vh - var(--top-h));
    /*transition: height 350ms cubic-bezier(0.85, 0, 0.15, 1); /* expoInOut: same as player */
    display: flex;
    flex-direction: column;
  }

  .player-open .inner {
    height: calc(100vh - var(--top-h) - var(--player-h));
  }

  .links {
    overflow-y: auto;
    overflow-x: hidden;
    flex: 1;
  }

  .drawer-overlay {
    display: none;
    position: fixed;
    inset: 0;
    background: rgba(0,0,0,0.5);
    z-index: calc(var(--z-drawer-fixed) - 1);
  }
  
  /* .account-switch {
    display: flex;
    flex-direction: column;
    align-items: stretch;
    padding: 0 0.5rem 0.5rem 0.5rem;
  }

  .account-switch-inner {
    display: flex;
    flex-direction: column;
    align-items: stretch;
    position: relative;
  }

  .account-switch-anchor {
    position: absolute;
    inset-inline-start: 0;
    inset-block-end: 0;
    width: 0;
    height: 0;
  }

  .account-switch-btn {
    flex: 1;
    padding: 1rem;
    display: flex;
    flex-direction: row;
    align-items: center;
    border-radius: 0.25rem;
    transition: background-color 200ms ease;
  }

  .account-switch.open .account-switch-btn, .account-switch-btn:hover {
    background: rgba(0,0,0,0.05);
  }

  .account-switch-btn-name {
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    margin-inline-end: 0.5rem;
    text-align: start;
    font-weight: var(--font-bold);
    font-size: 1.05rem;
  }

  .account-switch-btn-icon {
    display: flex;
    flex: none;
    font-size: 1rem;
  }

  .account-switch-menu {
    min-width: calc(var(--drawer-width) - 1rem);
    max-width: calc(100vw - 2rem);
    display: flex;
    flex-direction: column;
    overflow-y: auto;
    overflow-x: hidden;
    max-height: calc(100vh - 9rem);
    padding: 0.25rem;
    border-radius: 0.25rem;
    box-shadow: var(--some-shadow);
    background: #fff;
    position: relative;
    z-index: 1;
  }

  .account-switch-menu-item {
    padding: 1rem;
    border-radius: 0.25rem;
    transition: background-color 200ms ease;
    flex: none;
  }

  .account-switch-menu-item-text {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .account-switch-menu-item:not(.see-all) {
    font-weight: var(--font-bold);
  }

  .account-switch-menu-item.see-all {
    color: #333;
  }

  .account-switch-menu-item:hover {
    background: rgba(0,0,0,0.05);
  }

  .account-switch-menu-item.current {
    background: rgba(var(--blue-rgb), 0.1);
  }

  .account-switch-menu-sep {
    height: 2px;
    background: #ddd;
    margin: 0.25rem 0.5rem;
  } */

  .links {
    display: flex;
    flex-direction: column;
    overflow-y: auto;
    overflow-x: hidden;
  }

  @media screen and (max-width: 900px) {

    .drawer {
      position: fixed;
      top: 0;
      left: 0;
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
      margin-inline-end: 0.6rem;
    }

    .logo-text {
      font-size: 1.35rem;
    }

    .drawer:not(.fixed-open) {
      margin-inline-start: calc(var(--drawer-width) * -1);
      box-shadow: none;
    }
  }

  @media not screen and (max-width: 900px) {
    /* .player-open .account-switch-menu {
      max-height: calc(100vh - 14.5rem);
    }

    .account-switch {
      margin-top: 1rem;
    } */

    .links {
      padding-top: 1rem;
    }
  
    .drawer:not(.static-open) {
      margin-inline-start: calc(var(--drawer-width) * -1);
      box-shadow: none;
    }

    .top {
      display: none;
    }
  }

</style>

{#if fixed_open}
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div class="drawer-overlay" transition:fade|local={{ duration: 250 }} on:click={close} />
{/if}

<div class="drawer" class:player-open={false}  class:fixed-open={fixed_open} class:static-open={static_open}>
  <div class="inner">
    <div class="top">
      <button class="toggle ripple-container" use:ripple aria-label="Toogle drawer" on:click={close}>
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

    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div class="links super-thin-scroll" on:click={() => fixed_open = false}>
      <DrawerItem href="/" label="Dashboard" icon={mdiViewDashboardOutline} />
      <DrawerItem href="/admins" label="Admins" icon={mdiShieldAccountOutline} />
      <DrawerItem href="/users" label="Users" icon={mdiAccountMultipleOutline} />
      <DrawerItem href="/accounts" label="Accounts" icon={mdiAccountOutline} />
      <DrawerItem href="/stations" label="Stations" icon={mdiRadioTower} />
      <DrawerItem href="/plans" label="Plans" icon={mdiCurrencyUsd} />
      <DrawerItem href="/listeners" label="Listeners" icon={mdiConnection} />
      <DrawerItem href="/analytics" label="Analytics" icon={mdiPoll} />
    </div>
  </div>
</div>