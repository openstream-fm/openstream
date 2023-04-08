<script lang="ts">
	import Page from "$lib/components/Page.svelte";

  export let data: import("./$types").PageData;
</script>

<Page>
  <h1>{data.account.name}</h1>
</Page>