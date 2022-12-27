<script lang="ts">
  import { page } from "$app/stores";
	import Page from "$lib/components/Page.svelte";

  $: error = $page.error!;
</script>

<Page>
  <h1>{error.status ?? 500} Error</h1>

  <p>{error.message ?? "error message"}</p>

  <pre>
    <code>{error.code ?? "INTERNAL_NO_CODE"}</code>
  </pre>
</Page>