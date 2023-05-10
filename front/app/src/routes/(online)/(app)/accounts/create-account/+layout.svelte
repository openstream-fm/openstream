<script lang="ts">
  import SimpleLogo from '../../../../../lib/components/SimpleLogo.svelte';
  import TopUser from "$lib/components/Dashboard/TopUser.svelte";
	import { fly } from "svelte/transition";
</script>

<style>
  .layout {
    flex: 1;
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
</style>

<div class="layout" in:fly|local={{ x: -25, duration: 200 }}>
  
  <div class="top">
    <div class="logo">
      <SimpleLogo />
    </div>

    <div class="user-btn">
      <TopUser />
    </div>
  </div>

  <slot />
</div>