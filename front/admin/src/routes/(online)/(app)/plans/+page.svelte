<script lang="ts">
  export let data: import("./$types").PageData;
  import Page from "$lib/components/Page.svelte";
	import PageTop from "$lib/components/PageMenu/PageTop.svelte";
	import { ripple } from "$share/ripple";
	import { mdiCurrencyUsd } from "@mdi/js";

  const count_accounts = (accounts: typeof data.accounts, plan_id: string): number => {
    let count = 0;
    for(const account of accounts) {
      if(account.plan_id === plan_id) count += 1;
    }
    return count;
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
    font-weight: var(--font-bold);
  }

  .item-subtitle {
    margin-top: 0.25rem;
    color: #555;
  }

  .item-accounts {
    color: #555;
    font-size: 0.9rem;
    margin-top: 0.25rem;
  }

  .top-action {
    display: flex;
    flex: none;
    box-shadow: var(--some-shadow);
    color: #fff;
    padding: 0.75rem;
    background: var(--blue);
    border-radius: 0.25rem;
  }
  
  .empty {
    padding: 1rem;
  }  
</style>

<svelte:head>
  <title>Plans</title>
</svelte:head>

<Page>
  <PageTop icon={mdiCurrencyUsd}>
    <svelte:fragment slot="title">
      Plans
    </svelte:fragment>
    
    <svelte:fragment slot="subtitle">
      {data.plans.length} {data.plans.length === 1 ? "plan" : "plans"}
    </svelte:fragment>

    <a slot="action" href="/plans/create-plan" class="na top-action ripple-container" use:ripple>
      Create Plan
    </a>
  </PageTop>
  

  <div class="list">
    {#each data.plans as item (item._id)}
      {@const accounts = count_accounts(data.accounts, item._id)}
      <a href="/plans/{item._id}" class="na item ripple-container" use:ripple>
        <div class="item-title">{item.display_name}</div>
        <div class="item-subtitle">{item.identifier} - $ {item.price}</div>
        <div class="item-accounts">
          {accounts} 
          {#if accounts === 1}
            account
          {:else}
            accounts
          {/if}
        </div>
      </a>
    {:else}
      <div class="empty">
        There are no plans created yet
      </div>
    {/each}
  </div>
</Page>