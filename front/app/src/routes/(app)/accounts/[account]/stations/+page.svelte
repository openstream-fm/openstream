<script lang="ts">
	import TopUser from "$lib/components/Dashboard/TopUser.svelte";
	import Page from "$lib/components/Page.svelte";
	import Player from "$lib/components/Player/Player.svelte";
	import { ripple } from "$lib/ripple";
	import { fly } from "svelte/transition";
  export let data: import("./$types").PageData;
</script>

<style>
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

  .no-items {
    padding: 2rem;
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .no-items-message {
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

<Page>
  <div class="page">
    <div class="page-title">Select a station</div>
    <div class="list-box">
      {#each data.stations.items as station (station._id)}
        <a href="/accounts/{data.account._id}/stations/{station._id}" class="list-item na ripple-container" use:ripple>
          <span class="list-item-name">
            {station.name}
          </span>
        </a>
      {:else}
        <div class="no-items">
          <div class="no-items-message">
            This account doesn't have stations yet
          </div>
          <a href="/accounts/{data.account._id}/stations/create-station" class="na no-items-create ripple-container" use:ripple>
            Create a station
          </a>
        </div>
      {/each}
    </div>
  </div>
</Page>
