<script lang="ts">
  export let accounts: import("$server/defs/api/accounts/GET/Output").Output;
  export let account: import("$server/defs/api/accounts/[account]/GET/Output").Output["account"];
  export let user: import("$server/defs/api/users/[user]/GET/Output").Output["user"];

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
    flex: 1;
  }
</style>

<div class="dashboard">
  <Drawer {account} {accounts} {user} />
  <div class="content">
    <Top {account} {accounts} {user} />
    <div class="page">
      <slot />
    </div>
  </div>
</div>