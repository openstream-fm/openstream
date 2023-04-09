<script lang="ts">
  import { fly } from "svelte/transition";

  import Player from "$lib/components/Player/Player.svelte";
	import Top from "$lib/components/Dashboard/Top.svelte";
	import Drawer from "$lib/components/Dashboard/Drawer.svelte";
  
  let drawer_fixed_open = false;
  const open_drawer_fixed = () => drawer_fixed_open = true;
  const close_drawer_fixed = () => drawer_fixed_open = false;
</script>

<style>
  .dashboard {
    height: 100%;
    width: 100%;
    display: flex;
    flex-direction: column;
    background: var(--bg-gray);
  }

  .start {
    flex: 1;
    display: flex;
    flex-direction: row;
    align-items: stretch;
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
  <div class="start">
    <Drawer fixed_open={drawer_fixed_open} {close_drawer_fixed} {open_drawer_fixed} />
    <div class="content">
      <Top {drawer_fixed_open} {close_drawer_fixed} {open_drawer_fixed} />
      <div class="page">
        <slot />
      </div>
    </div>
  </div>
</div>