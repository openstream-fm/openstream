<script lang="ts">
	import Page from "$lib/components/Page.svelte";
	import Me from "./me.svelte";
  export let data: import("./$types").PageData;
  $: key = JSON.stringify([
    data.admin._id,
    data.admin.email,
    data.admin.first_name,
    data.admin.last_name,
    // data.user.phone,
    // data.user.language
  ]);
</script>

<Page compact>
  {#key key}
    <Me bind:data />
  {/key}
</Page>
