<script lang="ts">
	import TopUser from "$lib/components/Dashboard/TopUser.svelte";
	import Player from "$lib/components/Player/Player.svelte";
	import { ripple } from "$lib/ripple";
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
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    background: var(--bg-gray);
  }

  .page {
    display: flex;
    flex-grow: 1;
    flex-direction: column;
    align-items: center;
    padding-bottom: 3rem;
  }

  .page-title {
    margin-top: 2rem;
    font-size: 2rem;
    font-weight: 600;
  }

  .list-box {
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

  .list-item {
    padding: 1rem 2rem;
    cursor: pointer;
    user-select: none;
    font-size: 1.1rem;
    transition: background-color 150ms ease;
  }

  .list-item:hover {
    background: #eee;
  }

  .list-item-name {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .no-stations {
    padding: 2rem;
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .no-stations-message {
    font-size: 1.1rem;
  }

  .no-items-create {
    background: var(--blue);
    color: #fff;
    cursor: pointer;
    user-select: none;
    padding: 0.75rem 1rem;
    border-radius: 0.25rem;
    margin-top: 2rem;
    box-shadow: 0 4px 8px 0 rgb(0 0 0 / 12%), 0 2px 4px 0 rgb(0 0 0 / 8%);
    font-weight: 500;
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
      <TopUser user={data.user} accounts={data.accounts} account={null} stations={null} station={null} />
    </div>
  </div>
  <div class="page">
    <div class="page-title">Select an account</div>
    <div class="list-box">
      {#each data.accounts.items as account (account._id)}
        <a href="/accounts/{account._id}" class="list-item na ripple-container" use:ripple>
          <span class="list-item-name">
            {account.name}
          </span>
        </a>
      {:else}
        <div class="no-stations">
          <div class="no-stations-message">
            You don't have an account yet
          </div>
          <a href="/accounts/create-account" class="na no-items-create ripple-container" use:ripple>
            Create an account
          </a>
        </div>
      {/each}
    </div>
  </div>
  
  <Player />
</div>
