<script lang="ts">
	import { fly, scale } from "svelte/transition";

  export let user: import("$server/defs/api/users/[user]/GET/Output").Output["user"];
  export let accounts: import("$server/defs/api/accounts/GET/Output").Output;
  export let account: import("$server/defs/api/accounts/[account]/GET/Output").Output["account"];
  export let stations: import("$server/defs/api/stations/GET/Output").Output;
  //export let station: import("$server/defs/api/stations/[station]/GET/Output").Output["station"] | null;

  type Station = import("$server/defs/api/stations/[station]/GET/Output").Output["station"];

  import { page } from "$app/stores";
  // @ts-ignore
  $: station = ($page.data.station as Station) || null;

  let drawer_fixed_open = false;
  const open_drawer_fixed = () => drawer_fixed_open = true;
  const close_drawer_fixed = () => drawer_fixed_open = false;

	import Player from "../Player/Player.svelte";
  import Drawer from "./Drawer.svelte";
  import Top from "./Top.svelte";
</script>

<style>
  .dashboard {
    height: 100%;
    width: 100%;
    display: flex;
    flex-direction: row;
    background: var(--bg-gray);
  }

  .content {
    display: flex;
    flex-direction: column;
    flex: 1;
  }

  .page {
    flex-grow: 1;
  }
</style>

<div class="dashboard" in:fly={{ duration: 300, x: -25 }}>
  <Drawer {user} {accounts} {account} {stations} {station} fixed_open={drawer_fixed_open} {close_drawer_fixed} {open_drawer_fixed} />
  <div class="content">
    <Top {user} {accounts} {account} {stations} {station} drawer_fixed_open={drawer_fixed_open} {close_drawer_fixed} {open_drawer_fixed} />
    <div class="page">
      <slot />
    </div>
    <Player />
  </div>
</div>