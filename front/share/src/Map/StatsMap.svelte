<script lang="ts" context="module">
  export type Stats = import("$server/defs/stream-connection-stats/Stats").Stats;
  export type View = "now" | "last_24h" | "last_7d" | "last_30d";
</script>

<script lang="ts">
  import StatsMapInternal from "./StatsMapInternal.svelte";

  export let kind: "all" | "account" | "station";
  export let record_id: string;
  export let data: Stats | null = null;
  export let view: View = "now";
  export let locale: import("$server/locale/share/stats-map/stats-map.locale").StatsMapLocale;
  export let country_names: Record<string, string | undefined>;
</script>

{#key `${kind}-${record_id}`}
  <StatsMapInternal
    {kind}
    {record_id}
    {locale}
    {country_names}
    bind:data
    bind:view
  />
{/key}