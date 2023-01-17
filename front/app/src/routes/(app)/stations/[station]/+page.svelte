<script lang="ts">
	import KeyedPage from "./keyed_page.svelte";
  export let data: import("./$types").PageData;
</script>

{#key data.station._id}
  <KeyedPage {data} />
{/key}