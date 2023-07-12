<script lang="ts">
  export let data: import("./$types").PageData;
  import StatsMap from "$share/Map/StatsMap.svelte";
	import { _get, _patch } from "$share/net.client";
  
  import type { View } from "$share/Map/StatsMap.svelte";
	import { locale } from "$lib/locale";
	
  import type { Data } from "$share/Map/StationSelector.svelte";
	import StationSelector from "$share/Map/StationSelector.svelte";

  let selection_data: Data = {
    all_kind: "account",
    account_id: data.account._id,
    station: null,
    kind: "account",
    record_id: data.account._id,
    stations: data.stations,
    stats: data.stats,
    storage_public_url: data.config.storage_public_url,
  }

  let view: View = "now";
</script>

<style>
  .stats {
    background: #fff;
    border-radius: 0.5rem;
    box-shadow: 0 20px 25px -5px rgba(0,0,0,.1),0 10px 10px -5px rgba(0,0,0,.04);
  }
</style>

<div class="stats">

  <div class="selector">
    <StationSelector bind:data={selection_data} />
  </div>
      
  <StatsMap
    bind:view
    kind={selection_data.kind}
    record_id={selection_data.record_id}
    locale={$locale.stats_map}
    country_names={$locale.countries}
    bind:data={selection_data.stats}
  />
</div>

