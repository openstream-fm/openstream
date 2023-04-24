<script lang="ts">
  import StatsMapInternal from "./StatsMapInternal.svelte";

  export let kind: "account" | "station";
  export let record_id: string;
  export let data: import("$server/defs/stream-connection-stats/Stats").Stats | null = null;
</script>

{#key `${kind}-${record_id}`}
  <StatsMapInternal {kind} {record_id} bind:data />
{/key}