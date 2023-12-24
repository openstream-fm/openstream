<script lang="ts">
  export let data: import("./$types").PageData;

  let type: "stream" | "app" = "stream";
  let country_code: CountryCode | null | undefined = undefined;
  let os: string | null | undefined = undefined;
  let browser: string | null | undefined = undefined;
  let domain: string | null | undefined = undefined;
  let kind: QueryKind = "last-30d";
  let selected_stations: StationItem[] | "all" = "all";
  let loading: boolean = false;
  let app_kind: string | null | undefined = undefined;
  let app_version: number | null | undefined = undefined;
  let analytics_data: Data | null = null;

  type Snapshot = {
    type: "stream" | "app",
    kind: QueryKind,
    country_code: CountryCode | null | undefined,
    os: string | null | undefined,
    browser: string | null | undefined,
    domain: string | null | undefined,
    app_kind: string | null | undefined,
    app_version: number | null | undefined,
    selected_stations: StationItem[] | "all",
    analytics_data: Data | null,
  };

  export const snapshot = {
    capture: (): Snapshot => {
      return {
        type,
        analytics_data,
        country_code,
        os,
        browser,
        kind,
        domain,
        app_kind,
        app_version,
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
        domain,
        app_kind,
        app_version,
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
	import type { Data } from "$share/analytics/AnalyticsData.svelte";
	import { ripple } from "$share/ripple";
</script>

<svelte:head>
  <title>Analytics</title>
</svelte:head>

<Page>
  <PageTop>
    <svelte:fragment slot="title">Analytics</svelte:fragment>
  </PageTop>
  
  <Analytics
    stations={data.stations}
    bind:data={analytics_data}
    bind:loading
    bind:selected_stations
    bind:kind
    bind:country_code
    bind:os
    bind:browser
    bind:domain
    bind:app_kind
    bind:app_version
    lang={$lang}
    locale={$locale.analytics}
    stats_map_locale={$locale.stats_map}
    country_names={$locale.countries}
  />
</Page>