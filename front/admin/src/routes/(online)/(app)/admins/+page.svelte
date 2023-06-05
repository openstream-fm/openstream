<script lang="ts">
  export let data: import("./$types").PageData;

  import Page from "$lib/components/Page.svelte";
	import PageTop from "$lib/components/PageMenu/PageTop.svelte";
	import { ripple } from "$share/ripple";
	import { mdiShieldAccountOutline } from "@mdi/js";
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
    font-weight: 600;
  }

  .item-subtitle {
    color: #555;
  }
</style>

<svelte:head>
  <title>Admins</title>
</svelte:head>

<Page>
  <PageTop icon={mdiShieldAccountOutline}>
    <svelte:fragment slot="title">
      Admins
    </svelte:fragment>
    <svelte:fragment slot="subtitle">
      {data.admins.total} admins
    </svelte:fragment>
  </PageTop>
  <div class="list">
    {#each data.admins.items as item (item._id)}
      <a href="/admins/{item._id}" class="na item ripple-container" use:ripple>
        <div class="item-title">{item.first_name} {item.last_name}</div>
        <div class="item-subtitle">{item.email}</div>
      </a>
    {/each}
  </div>
</Page>