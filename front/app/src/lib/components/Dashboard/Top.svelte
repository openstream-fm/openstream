<script lang="ts">

  export let user: import("$server/defs/api/users/[user]/GET/Output").Output["user"];
  export let accounts: import("$server/defs/api/accounts/GET/Output").Output;
  export let account: import("$server/defs/api/accounts/[account]/GET/Output").Output["account"] | null;
  export let stations: import("$server/defs/api/stations/GET/Output").Output;
  export let station: import("$server/defs/api/stations/[station]/GET/Output").Output["station"] | null;

  export let drawer_fixed_open: boolean;
  export let close_drawer_fixed: () => void;
  export let open_drawer_fixed: () => void;

  import Icon from "$share/Icon.svelte";
  import { ripple } from "$share/ripple";
	import { mdiMenu } from "@mdi/js";

  import { page } from "$app/stores";
  $: path = $page.url.pathname;

  const toggle_drawer = () => drawer_fixed_open ? close_drawer_fixed() : open_drawer_fixed();

  import TopUser from "./TopUser.svelte";
	import { fly } from "svelte/transition";
</script>

<style>
  .top {
    position: sticky;
    top: 0;
    z-index: var(--z-top);
    display: flex;
    flex-direction: column;
    background: rgba(var(--bg-gray-rgb), 0.875);
    backdrop-filter: blur(2px);
  }

  .box {
    position: relative;
    flex: none;
    padding: 0.75rem 0;
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

  .station-out {
    padding: 1rem 1rem 0 1rem ;
    display: flex;
    align-items: stretch;
    justify-content: stretch;
    flex-direction: column;
  }

  .station-scroll {
    overflow-y: visible;
    overflow-x: auto;
    background: #fff;
    box-shadow: var(--some-shadow);
    border-radius: 0.5rem;
    display: flex;
  }

  .station {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-start;
    flex: none;
    gap: 1rem;
    padding: 1rem 1.5rem 1rem 1.5rem;
  }

  .station-name {
    max-width: 10rem;
    display: flex;
    flex-shrink: 1;
    min-width: 4rem;
  }

  .station-name, .station-action {
    font-size: 1rem;
  }

  .station-actions {
    display: flex;
    flex-direction: row;
    align-items: center;
    flex: none;
  }

  .station-action {
    flex: none;
    color: var(--blue);
    text-decoration: none;
    transition: background-color 200ms ease;
    padding: 0.5rem 0.75rem;
    border-radius: 0.25rem;
    flex: none;
  }

  .station-action.current {
    background: rgba(var(--red-rgb),  0.05);
  }

  .station-top-name {
    flex: 1;
    padding: 1rem;
    align-self: center;
    margin-inline-start: 1rem;
    display: flex;
  }

  .station-name-ellipsis, .station-top-name-ellipsis {
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
    {#if account != null && station != null}
      <a class="na station-top-name ripple-container" href="/accounts/{account._id}/stations/{station._id}" use:ripple in:fly={{ duration: 300, x: -25 }}>
        <div class="station-top-name-ellipsis">
          {station.name}
        </div>
      </a>
    {/if}
    <TopUser {user} {accounts} {account} {stations} {station} />
  </div>
</div>

{#if account != null && station != null}
  <div class="station-out" in:fly={{ duration: 300, y: -25 }}>
    <div class="station-scroll super-thin-scroll">
      <div class="station">
        <div class="station-name">
          <div class="station-name-ellipsis">
            {station.name}
          </div>
        </div>
        <div class="station-actions">
          <a
            href="/accounts/{account._id}/stations/{station._id}"
            class="station-action ripple-container"
            class:current={path === `/accounts/${account._id}/stations/${station._id}`}
            use:ripple
            >
            Dashboard
          </a>

          <a
            href="/accounts/{account._id}/stations/{station._id}/profile"
            class="station-action ripple-container"
            class:current={path === `/accounts/${account._id}/stations/${station._id}/profile`}
            use:ripple
            >
            Profile
          </a>

          <a
            href="/accounts/{account._id}/stations/{station._id}/playlist"
            class="station-action ripple-container"
            class:current={path === `/accounts/${account._id}/stations/${station._id}/playlist`}
            use:ripple
            >  
            Playlist
          </a>
        </div>
      </div>
    </div>
  </div>
{/if}