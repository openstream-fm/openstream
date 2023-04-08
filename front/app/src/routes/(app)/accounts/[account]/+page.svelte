<script lang="ts">
	import Page from "$lib/components/Page.svelte";

  export let data: import("./$types").PageData;
</script>

<svelte:head>
  <title>{data.account.name}</title>
</svelte:head>

<Page>
  <h1>{data.account.name}</h1>
</Page>