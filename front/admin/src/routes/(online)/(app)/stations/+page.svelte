<script lang="ts">
  export let data: import("./$types").PageData;

  import Page from "$lib/components/Page.svelte";
	import { ripple } from "$share/ripple";

  const get_account = (_data: typeof data, account_id: string) => {
    return _data.accounts.items.find(item => item._id === account_id)
  }
</script>

<style>
  p {
    color: #444;
    font-size: 0.9rem;
    margin-inline-start: 0.25rem;
  }

  .list {
    box-shadow: var(--some-shadow);
    display: flex;
    flex-direction: column;
    align-items: stretch;
    background: #fff;
    padding: 0.5rem;
    border-radius: 0.5rem;
    margin-top: 1rem;
  }

  .item {
    display: flex;
    flex-direction: row;
    align-items: center;
    transition: background-color 200ms ease;
    padding: 0.75rem 0.75rem;
    border-radius: 0.5rem;
  }

  .item-pic {
    flex: none;
    width: 3rem;
    height: 3rem;
    background-size: contain;
    background-position: center;
    background-repeat: no-repeat;
    border-radius: 0.5rem;
    margin-inline-end: 0.75rem;
  }

  .item-info {
    display: flex;
    flex-direction: column;
  }

  .item:hover {
    background-color: rgba(0,0,0,0.05);
  }

  .item-title {
    font-size: 1.25rem;
    font-weight: 600;
  }

  .item-subtitle {
    margin-top: 0.25rem;
    color: #555;
  }
</style>


<svelte:head>
  <title>Stations</title>
</svelte:head>

<Page>
  <h1>Stations</h1>
  <p>{data.stations.total} stations</p>

  <div class="list">
    {#each data.stations.items as item (item._id)}
      {@const account = get_account(data, item.account_id)}
      <a href="/stations/{item._id}" class="na item ripple-container" use:ripple>
        <div class="item-pic" style:background-image="url({data.config.storage_public_url}/station-pictures/webp/64/{item.picture_id}.webp)" />
        <div class="item-info">
          <div class="item-title">{item.name}</div>
          <div class="item-subtitle">
            {#if account}
              {account.name}
            {:else}
              #{item.account_id}
            {/if}
          </div>
        </div>
      </a>
    {/each}
  </div>
</Page>