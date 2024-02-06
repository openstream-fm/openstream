<script lang="ts">
  export let data: import("./$types").LayoutData;

  import Top from "$lib/components/Dashboard/Top.svelte";
	import Drawer from "$lib/components/Dashboard/Drawer.svelte";
	import { logical_fly } from "$share/transition";
  
  let drawer_fixed_open = false;
  let drawer_static_open = true;

  const open = () => {
    if(window.innerWidth > 900) {
      drawer_static_open = true;
    } else {
      drawer_fixed_open = true;
    }
  }

  const close = () => {
    if(window.innerWidth > 900) {
      drawer_static_open = false;
    } else {
      drawer_fixed_open = false;
    }
  }

  const toggle = () => {
    if(window.innerWidth > 900) {
      drawer_static_open  = !drawer_static_open; 
    } else {
      drawer_fixed_open = !drawer_fixed_open;
    }
  }

</script>

<style>
  .dashboard {
    flex: 1;
    display: flex;
    flex-direction: column;
    background: var(--bg-gray);
  }

  .bottom {
    flex: 1;
    display: flex;
    flex-direction: row;
    align-items: stretch;
  }

  .content {
    flex: 1;
    display: flex;
    flex-direction: column;
  }

  .page {
    flex-grow: 1;
  }
</style>

<div class="dashboard" in:logical_fly={{ duration: 300, x: -25 }}>
  <Top toggle_drawer={toggle} with_drawer={true} />
  <div class="bottom">
    <Drawer bind:fixed_open={drawer_fixed_open} bind:static_open={drawer_static_open} {open} {close} {toggle}  />
    <div class="content">
      {#key data.account._id}
        <div class="page">
          <slot />
        </div>
      {/key}
    </div>
  </div>    
</div>