<script lang="ts">
  export let data: import("./$types").LayoutData;
  import Dashboard from "$lib/components/Dashboard/Dashboard.svelte";
</script>

<Dashboard accounts={data.accounts} account={data.account} user={data.user}>
  <slot />
</Dashboard>