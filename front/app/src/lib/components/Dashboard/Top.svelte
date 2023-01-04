<script lang="ts">
  export let accounts: import("$server/defs/api/accounts/GET/Output").Output;
  export let account: import("$server/defs/api/accounts/[account]/GET/Output").Output["account"];
  export let user: import("$server/defs/api/users/[user]/GET/Output").Output["user"];

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
</style>

<div class="top">
  <div class="box">
    <TopUser {user} {account} {accounts} />
  </div>
</div>