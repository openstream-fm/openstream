<script lang="ts">
  export let data: import("./$types").PageData;

  import Page from "$lib/components/Page.svelte";
	import { ripple } from "$share/ripple";
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
    color: #555;
  }
</style>

<svelte:head>
  <title>Admins</title>
</svelte:head>

<Page>
  <h1>Admins</h1>
  <p>{data.admins.total} admins</p>

  <div class="list">
    {#each data.admins.items as item (item._id)}
      <a href="/admins/{item._id}" class="na item ripple-container" use:ripple>
        <div class="item-title">{item.first_name} {item.last_name}</div>
        <div class="item-subtitle">{item.email}</div>
      </a>
    {/each}
  </div>
</Page>