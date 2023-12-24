<script lang="ts">
  export let data: import("./$types").PageData;
  import Page from "$lib/components/Page.svelte";
	import Analytics from "$share/analytics/Analytics.svelte";
	import AnalyticsTypeSelector from "$share/analytics/AnalyticsTypeSelector.svelte";
  import { locale, lang } from "$lib/locale";

  import type { CountryCode } from "$server/defs/CountryCode";
	import type { QueryKind, StationItem } from "$share/analytics/AnalyticsFilters.svelte";
	import type { Data } from "$share/analytics/AnalyticsData.svelte";

  $: account_stations = data.stations.items.filter(item => item.account_id === data.account._id);

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
</script>

<svelte:head>
  <title>{$locale.pages["account/analytics"].head.title}</title>
</svelte:head>

<Page>
  <h1>
    {$locale.pages["account/analytics"].title}
  </h1>

  <Analytics
    stations={account_stations}
    bind:type
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