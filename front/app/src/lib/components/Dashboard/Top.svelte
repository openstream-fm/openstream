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

  const toggle_drawer = () => drawer_fixed_open ? close_drawer_fixed() : open_drawer_fixed();

  import TopUser from "./TopUser.svelte";
</script>

<style>
  .top {
    position: sticky;
    top: 0;
    z-index: var(--z-top);
    height: var(--top-height);
    display: flex;
    flex-direction: row;
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
    flex: none;
    width: var(--top-height);
    color: #333;
    transition: background-color 150ms ease;
    justify-self: flex-start;
  }

  .drawer-toggle:hover {
    background-color: rgba(0,0,0,0.05);
  }

  @media screen and (max-width: 900px) {
    .drawer-toggle {
      display: flex;
    }
  } 
</style>

<div class="top">
  <div class="box">
    <button class="drawer-toggle ripple-container" use:ripple aria-label="Toggle drawer" on:click={toggle_drawer}>
      <Icon d={mdiMenu} />
    </button>
    <TopUser {user} {accounts} {account} {stations} {station} />
  </div>
</div>