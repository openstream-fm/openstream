<script lang="ts">
  export let data: import("./$types").LayoutData;
  import Dashboard from "$lib/components/Dashboard/Dashboard.svelte";
</script>

<Dashboard user={data.user} accounts={data.accounts} account={data.account} stations={data.stations} station={data.station}>
  <slot />
</Dashboard>