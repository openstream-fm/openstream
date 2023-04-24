<script lang="ts">
  export let data: import("./$types").PageData;
  import Page from "$lib/components/Page.svelte";
	import StatsMap from "$share/Map/StatsMap.svelte";
</script>

<style>

  .h1 {
    font-weight: 600;
  }

  .stats {
    margin-top: 2rem;
    background: #fff;
    border-radius: 0.5rem;
    box-shadow: 0 20px 25px -5px rgba(0,0,0,.1),0 10px 10px -5px rgba(0,0,0,.04);
  }
</style>

<svelte:head>
  <title>{data.account.name}</title>
</svelte:head>

<Page>
  <h1>{data.account.name}</h1>

  <div class="stats">
    <StatsMap kind="account" record_id={data.account._id} bind:data={data.stats} />
  </div>
</Page>