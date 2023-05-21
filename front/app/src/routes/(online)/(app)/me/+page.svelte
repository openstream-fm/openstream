<script lang="ts">
	import Page from "$lib/components/Page.svelte";
	import Me from "./me.svelte";
  export let data: import("./$types").PageData;
  $: key = JSON.stringify([
    data.user._id,
    data.user.email,
    data.user.first_name,
    data.user.last_name,
    data.user.phone,
    data.user.language
  ]);
</script>

<Page compact>
  {#key key}
    <Me bind:data />
  {/key}
</Page>
