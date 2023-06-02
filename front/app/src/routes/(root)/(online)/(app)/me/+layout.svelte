<script lang="ts">
	import TopUser from "$lib/components/Dashboard/TopUser.svelte";
	import SimpleLogo from "$lib/components/SimpleLogo.svelte";
	import { logical_fly } from "$share/transition";
</script>
<style>
  .layout {
    flex: 1;
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    background: var(--bg-gray);
  }

  .top {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    padding: 1.5rem;
  }

  .logo {
    font-size: min(6vw, 2rem);
  }

  .user-btn {
    margin-inline-end: -1rem;
  }

  .page {
    display: flex;
    flex-direction: column;
  }

</style>

<div class="layout" in:logical_fly|local={{ x: -25, duration: 200 }}>
  <div class="top">
    <div class="logo">
      <SimpleLogo />
    </div>

    <div class="user-btn">
      <TopUser />
    </div>
  </div>

  <div class="page">
    <slot />
  </div>
</div>