<script lang="ts">
	import Page from "$lib/components/Page.svelte";

  export let data: import("./$types").PageData;
</script>

<Page>
  <pre>
    <code>
      {JSON.stringify(data, null, 2)}
    </code>
  </pre>
</Page>