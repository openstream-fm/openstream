<script lang="ts">
  export let data: import("./$types").PageData;

  let country_code: CountryCode | null | undefined = undefined;
  let os: string | null | undefined = undefined;
  let browser: string | null | undefined = undefined;
  let kind: QueryKind = "last-30d";
  let selected_stations: StationItem[] | "all" = "all";
  let loading: boolean = false;
  let analytics_data: import("$server/defs/analytics/Analytics").Analytics | null = null;

  type Snapshot = {
    country_code: CountryCode | null | undefined,
    os: string | null | undefined,
    browser: string | null | undefined,
    kind: QueryKind,
    selected_stations: StationItem[] | "all",
    analytics_data: import("$server/defs/analytics/Analytics").Analytics | null,
  };

  export const snapshot = {
    capture: (): Snapshot => {
      return {
        analytics_data,
        country_code,
        os,
        browser,
        kind,
        selected_stations
      }
    },

    restore: (snapshot: Snapshot) => {
      ({ 
        analytics_data,
        kind,
        browser,
        country_code,
        os,
        selected_stations,
      } = snapshot);
    }
  }

  import Page from "$lib/components/Page.svelte";
	import PageTop from "$lib/components/PageMenu/PageTop.svelte";
	import Analytics from "$share/analytics/Analytics.svelte";

  import { locale, lang } from "$lib/locale";
	import type { CountryCode } from "$server/defs/CountryCode";
	import type { QueryKind, StationItem } from "$share/analytics/AnalyticsFilters.svelte";
</script>

<svelte:head>
  <title>Analytics</title>
</svelte:head>

<Page>
  <PageTop>
    <svelte:fragment slot="title">
      Analytics
    </svelte:fragment>
  </PageTop>
  
  <Analytics
    stations={data.stations.items}
    bind:data={analytics_data}
    bind:loading
    bind:selected_stations
    bind:kind
    bind:country_code
    bind:os
    bind:browser
    lang={$lang}
    locale={$locale.analytics}
    stats_map_locale={$locale.stats_map}
    country_names={$locale.countries}
  />
</Page>