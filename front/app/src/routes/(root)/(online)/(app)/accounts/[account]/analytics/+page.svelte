<script lang="ts">
  export let data: import("./$types").PageData;
  import Page from "$lib/components/Page.svelte";
	import Analytics from "$share/analytics/Analytics.svelte";
  import { locale, lang } from "$lib/locale";

  import type { CountryCode } from "$server/defs/CountryCode";
	import type { QueryKind, StationItem } from "$share/analytics/AnalyticsFilters.svelte";

  $: account_stations = data.stations.items.filter(item => item.account_id === data.account._id);

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
</script>

<svelte:head>
  <title>{$locale.pages["account/analytics"].head.title}</title>
</svelte:head>

<Page>
  <h1>{$locale.pages["account/analytics"].title}</h1>

  <Analytics
    stations={account_stations}
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