<script lang="ts">
  export let data: import("./$types").PageData;
  import Page from "$lib/components/Page.svelte";
	import { ripple } from "$share/ripple";
	import PlanForm from "./PlanForm.svelte";

  $: active_plans = data.plans.items.filter(item => item.deleted_at == null);

  const count_accounts = (accounts: typeof data.accounts, plan_id: string): number => {
    let count = 0;
    for(const account of accounts.items) {
      if(account.plan_id === plan_id) count += 1;
    }
    return count;
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
    font-weight: 600;
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

  .top {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
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
  <div class="top">
    <div class="title">
      <h1>Plans</h1>
      <p>
        {active_plans.length} 
        {#if active_plans.length === 1}
          plan
        {:else}
          plans
        {/if}
      </p>
    </div>
    <a href="/plans/create-plan" class="na top-action ripple-container" use:ripple>
      Create Plan
    </a>
  </div>
  

  <div class="list">
    {#each active_plans as item (item._id)}
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