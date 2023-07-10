<script lang="ts">
  export let stations: StationItem[];
  export let selected_stations: "all" | StationItem[];
  export let kind: QueryKind;
  //export let custom_since: Date | null = null;
  //export let custom_until: Date | null = null;
  export let loading: boolean = false;
  export let browser: string | null | undefined;
  export let os: string | null | undefined;
  export let country_code: CountryCode | null | undefined;
  export let domain: string | null | undefined;

  export let locale: import("$server/locale/share/analytics/analytics.locale").AnalyticsLocale;
  export let country_names: import("$server/locale/share/countries/countries.locale").CountriesLocale;
  export let stats_map_locale: import("$server/locale/share/stats-map/stats-map.locale").StatsMapLocale;
  
  export let data: import("$server/defs/analytics/Analytics").Analytics | null;
  export let lang: string;

  import type { CountryCode } from "$server/defs/CountryCode";
  import type { StationItem, QueryKind,  } from "./AnalyticsFilters.svelte";
  export type { StationItem, QueryKind };

  import AnalyticsFilters from "$share/analytics/AnalyticsFilters.svelte";
	import { _get, action } from "$share/net.client";
	import { hash } from "$server/util/collections";
  import AnalyticsData from "./AnalyticsData.svelte";

  let filters: AnalyticsFilters;

  export const submit = async (qs = filters.get_resolved_qs()) => {
    if(loading) return;
    loading = true;
    try {
      const { analytics } = await _get<import("$server/defs/api/analytics/GET/Output").Output>(`/api/analytics?${qs}`);
      data = analytics;
      loading = false;
    } catch(e) {
      loading = false;
      throw e;
    }
  }

  const on_submit = action(() => {
    submit()  
  });

  import type { ClickEvent } from "./AnalyticsData.svelte";
  import { assert_never } from "$share/assert-never";
  import { tick } from "svelte";
  const on_data_click = async (event: ClickEvent) => {
    if(event.kind === "country_code") {
      if(event.value === country_code) {
        country_code = undefined;
      } else {
        country_code = event.value;
      }
    } else if(event.kind === "os") {
      if(event.value === os) {
        os = undefined;
      } else {
        os = event.value;
      }
    } else if(event.kind === "browser") {
      if(event.value === browser) {
        browser = undefined;
      } else {
        browser = event.value;
      }
    } else if (event.kind === "domain") {
      if(event.value === domain) {
        domain = undefined;
      } else {
        domain = event.value;
      }
    } else if(event.kind === "station") {
      const item = stations.find(station => station._id === event.value);
      if(item == undefined) selected_stations = "all";
      else {
        const is_only_selected = Array.isArray(selected_stations) && selected_stations.length === 1 && selected_stations[0]._id === item._id;
        if(is_only_selected) {
          selected_stations = "all"
        } else {
          selected_stations = [item]
        }
      }
    } else {
      return  assert_never(event, "AnalyticsData.ClickEvent.kind")
    }

    await tick();
    submit();
  }
</script>

<style>
  .boxes {
    display: flex;
    flex-direction: column;
    align-items: stretch;
    gap: 2rem;
  }
  
  .filters {
    background: #fff;
    display: flex;
    flex-direction: column;
    border-radius: 0.5rem;
    box-shadow: var(--some-shadow);
    padding: 1rem;
    margin-block-start: 2rem;
  }
  
  .analytics {
    margin-top: 2rem;
    transition: opacity 300ms ease;
  }

  .analytics.loading {
    opacity: 0.2;
  }
</style>


<div class="boxes">
  <div class="filters">
    <AnalyticsFilters
      bind:this={filters}
      {loading}
      stations={stations}
      bind:selected_stations={selected_stations}
      bind:kind={kind}
      {on_submit}
      locale={locale}
      country_names={country_names}
      bind:country_code={country_code}
      bind:os={os}
      bind:browser={browser}
      bind:domain={domain}
    />
  </div>

  {#if data}
    <div class="analytics" class:loading>
      {#key hash(data)}
        <AnalyticsData
          data={data}
          selected_stations={selected_stations}
          country_code={country_code}
          os={os}
          browser={browser}
          domain={domain}
          country_names={country_names}
          lang={lang}
          locale={locale}
          stats_map_locale={stats_map_locale}
          on_click={on_data_click}
        />
      {/key}
    </div>
  {/if}
</div>