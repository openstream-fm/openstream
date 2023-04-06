<script lang="ts">
	import TopUser from "$lib/components/Dashboard/TopUser.svelte";
	// import { ripple } from "$lib/ripple";
	import { fly } from "svelte/transition";
  export let data: import("./$types").PageData;
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
    flex-direction: column;
    align-items: center;
  }

  .page-title {
    margin-top: 2rem;
    font-size: 2rem;
    font-weight: 600;
  }

  .stations-box {
    margin-top: 3.5rem;
    width: min(80%, 500px);
    background: #fff;
    box-shadow: 0 0 15px 0 rgb(0 0 0 / 10%);
    border-radius: 0.5rem;
    padding: 1rem 0;
    display: flex;
    flex-direction: column;
    align-items: stretch;
    min-width: 0;
  }
</style>

<svelte:head>
  <title>Select station</title>
</svelte:head>

<div class="layout" in:fly|local={{ x: -25, duration: 200 }}>
  <div class="top">
    <div class="title">
      openstream
    </div>

    <div class="user-btn">
      <TopUser station={null} user={data.user} stations={data.stations} />
    </div>
  </div>
  <div class="page">
    <div class="page-title">Create a station</div>
    <div class="stations-box">

    </div>
  </div>
</div>