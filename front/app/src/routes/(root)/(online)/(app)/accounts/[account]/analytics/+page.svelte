<script lang="ts">
  export let data: import("./$types").PageData;
  import Page from "$lib/components/Page.svelte";
	import Analytics from "$share/analytics/Analytics.svelte";
  import { locale, lang } from "$lib/locale";
	import AnalyticsFilters from "$share/analytics/AnalyticsFilters.svelte";
	import type { OnSubmitEvent } from "$share/analytics/AnalyticsFilters.svelte";
  import { _get, action } from "$share/net.client";
	import { hash } from "$server/util/collections";

  $: account_stations = data.stations.items.filter(item => item.account_id === data.account._id);
  
  let analytics: import("$server/defs/analytics/Analytics").Analytics | null = null;
  let loading = false;

  const on_submit = action(async ({ qs }: OnSubmitEvent) => {
    if(loading) return;
    loading = true;
    try {
      const { analytics: data } = await _get<import("$server/defs/api/analytics/GET/Output").Output>(`/api/analytics?${qs}`);
      analytics = data;
      loading = false;
    } catch(e) {
      loading = false;
      throw e;
    }
  });
</script>

<style>
  h1 {
    font-weight: 600;
  }

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


<svelte:head>
  <title>Analytics</title>
</svelte:head>

<Page>
  <h1>Analytics</h1>

  <div class="boxes">
    <div class="filters">
      <AnalyticsFilters
        {loading}
        stations={account_stations}
        selected_stations="all"
        kind="last-24h"
        {on_submit}
        locale={$locale.analytics.filters}
      />
    </div>

    {#if analytics}
      {#key hash(analytics)}
        <div class="analytics" class:loading>
          <Analytics
            data={analytics}
            country_names={$locale.countries}
            lang={$lang}
            locale={$locale.analytics}
            stats_map_locale={$locale.stats_map}
          />
        </div>
      {/key}
    {/if}
  </div>
</Page>