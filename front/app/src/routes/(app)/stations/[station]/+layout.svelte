<script lang="ts">
  export let data: import("./$types").LayoutData;
  import Dashboard from "$lib/components/Dashboard/Dashboard.svelte";
</script>

<Dashboard stations={data.stations} station={data.station} user={data.user}>
  <slot />
</Dashboard>