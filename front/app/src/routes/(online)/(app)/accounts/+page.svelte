<script lang="ts">
	import TopUser from "$lib/components/Dashboard/TopUser.svelte";
	import SimpleLogo from "$lib/components/SimpleLogo.svelte";
	import { locale } from "$lib/locale";
	import { ripple } from "$share/ripple";
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

  .logo {
    font-size: min(6vw, 2rem);
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

  .or {
    margin-top: 1.5rem;
    color: #666;
  }

  .create {
    margin-top: 0.25rem;
    padding: 0.75rem;
    display: flex;
    text-align: center;
    text-decoration: none;
    color: var(--blue);
    font-size: 1.1rem;
    border-radius: 0.5rem;
  }



  .no-items {
    padding: 2rem;
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .no-items-message {
    font-size: 1.1rem;
    text-align: center;
    line-height: 1.5rem;
  }

  @media screen and (max-width: 460px) {
    .no-items-message {
      font-size: 1rem;
    }
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
  <title>{$locale.pages.accounts.head.title}</title>
</svelte:head>

<div class="layout" in:fly={{ x: -25, duration: 200 }}>
  <div class="top">
    <div class="logo">
      <SimpleLogo />
    </div>

    <div class="user-btn">
      <TopUser />
    </div>
  </div>
  <div class="page">
    <div class="page-title">{$locale.pages.accounts.title}</div>
    {#if data.accounts.items.length}
      <div class="list-box">
        {#each data.accounts.items as account (account._id)}
        <a href="/accounts/{account._id}" class="list-item na ripple-container" use:ripple>
          <span class="list-item-name">
            {account.name}
          </span>
        </a>
        {/each}
      </div>

      <div class="or">{$locale.pages.accounts.or}</div>

      <a class="create ripple-container" href="/accounts/create-account" use:ripple>
        {$locale.pages.accounts.create_new_account}
      </a> 
    {:else}
      <div class="no-items">
        <div class="no-items-message">
          {@html $locale.pages.accounts.no_items_message_html}
        </div>
        <a href="/accounts/create-account" class="na no-items-create ripple-container" use:ripple>
          {$locale.pages.accounts.no_items_create}
        </a>
      </div>
    {/if}
  </div>
</div>
