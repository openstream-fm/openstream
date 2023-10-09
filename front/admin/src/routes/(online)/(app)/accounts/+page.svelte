<script lang="ts">
  export let data: import("./$types").PageData;

  import Page from "$lib/components/Page.svelte";
  import PageTop from "$lib/components/PageMenu/PageTop.svelte";
	import { ripple } from "$share/ripple";
	import { mdiAccountOutline } from "@mdi/js";

  const get_stations_for_account = (_data: typeof data, account_id: string) => {
    return _data.stations.filter(item => item.account_id === account_id)
  }
</script>

<style>
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
    flex-direction: column;
    transition: background-color 200ms ease;
    padding: 1rem 1rem;
    border-radius: 0.5rem;
  }

  .item:hover {
    background-color: rgba(0,0,0,0.05);
  }

  .item-title {
    font-size: 1.25rem;
    font-weight: var(--font-bold);;
  }

  .item-subtitle {
    margin-top: 0.25rem;
    color: #555;
  }

  .item-listeners {
    color: #555;
    font-size: 0.9rem;
    margin-top: 0.25rem;
  }
</style>

<svelte:head>
  <title>Accounts</title>
</svelte:head>

<Page>
  <PageTop icon={mdiAccountOutline}>
    <svelte:fragment slot="title">
      Accounts
    </svelte:fragment>
    <svelte:fragment slot="subtitle">
      {data.accounts.length} {data.accounts.length === 1 ? "account" : "accounts"}
    </svelte:fragment>
  </PageTop>

  <div class="list">
    {#each data.accounts as item (item._id)}
      {@const stations = get_stations_for_account(data, item._id)}
      <a href="/accounts/{item._id}" class="na item ripple-container" use:ripple>
        <div class="item-title">{item.name}</div>
        <div class="item-subtitle">{stations.length} stations</div>
        <div class="item-listeners">
          {#if item.limits.listeners.used === 1}
            1 listener
          {:else}
            {item.limits.listeners.used} listeners
          {/if}
        </div>
      </a>
    {/each}
  </div>
</Page>