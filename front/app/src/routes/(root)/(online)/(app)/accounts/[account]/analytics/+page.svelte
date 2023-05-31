<script lang="ts">
  export let data: import("./$types").PageData;
  import Page from "$lib/components/Page.svelte";
	import Analytics from "$share/analytics/Analytics.svelte";
  import { locale } from "$lib/locale";
</script>

<style>
  h1 {
    font-weight: 600;
  }

  /* p {
    font-size: 1.2rem;
    margin-top: 1.5rem;
  } */

  .analytics {
    margin-top: 2rem;
  }
</style>


<svelte:head>
  <title>Analytics</title>
</svelte:head>

<Page>
  <h1>Analytics</h1>

  <div class="analytics">
    <Analytics data={data.analytics} country_names={$locale.countries} />
  </div>
</Page>