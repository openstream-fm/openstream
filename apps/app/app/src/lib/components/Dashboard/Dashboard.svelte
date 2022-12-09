<script lang="ts">
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

  .page {
    height: 1600px;
  }
</style>

<div class="dashboard">
  <Drawer />
  <div class="content">
    <Top />
    <div class="page">
      <slot />
    </div>
  </div>
</div>