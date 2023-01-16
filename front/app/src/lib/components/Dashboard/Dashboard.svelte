<script lang="ts">
  export let accounts: import("$server/defs/api/accounts/GET/Output").Output;
  export let account: import("$server/defs/api/accounts/[account]/GET/Output").Output["account"];
  export let user: import("$server/defs/api/users/[user]/GET/Output").Output["user"];

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

<div class="dashboard">
  <Drawer {account} {accounts} {user} fixed_open={drawer_fixed_open} {close_drawer_fixed} {open_drawer_fixed} />
  <div class="content">
    <Top {account} {accounts} {user} drawer_fixed_open={drawer_fixed_open} {close_drawer_fixed} {open_drawer_fixed} />
    <div class="page">
      <slot />
    </div>
    <Player />
  </div>
</div>