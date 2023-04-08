<script lang="ts">
	import TopUser from "$lib/components/Dashboard/TopUser.svelte";
	import Player from "$lib/components/Player/Player.svelte";
	// import { ripple } from "$lib/ripple";
	import { fly } from "svelte/transition";
</script>

<style>

  .top {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    padding: 1.5rem;
  }

  .title {
    color: var(--red);
    font-size: min(6vw, 2rem);
    font-weight: 600;
  }

  .user-btn {
    margin-inline-end: -1rem;
  }

  .layout {
    flex: 1;
    display: flex;
    flex-direction: column;
    background: var(--bg-gray);
  }

  .page {
    display: flex;
    flex-grow: 1;
    flex-direction: column;
    align-items: center;
  }

  .page-title {
    margin-top: 2rem;
    font-size: 2rem;
    font-weight: 600;
  }
</style>

<svelte:head>
  <title>Create new account</title>
</svelte:head>

<div class="layout" in:fly|local={{ x: -25, duration: 200 }}>
  <div class="top">
    <div class="title">
      openstream
    </div>

    <div class="user-btn">
      <TopUser />
    </div>
  </div>
  <div class="page">
    <div class="page-title">Create account</div>
    <div class="box">

    </div>
  </div>
</div>