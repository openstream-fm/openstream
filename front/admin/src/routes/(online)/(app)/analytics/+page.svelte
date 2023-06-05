<script lang="ts">
  export let data: import("./$types").PageData;
  import Page from "$lib/components/Page.svelte";
	import Analytics from "$share/analytics/Analytics.svelte";
  import { locale, lang } from "$lib/locale";
	import AnalyticsFilters from "$share/analytics/AnalyticsFilters.svelte";
	import type { OnSubmitEvent } from "$share/analytics/AnalyticsFilters.svelte";
  import { _get, action } from "$share/net.client";
	import { hash } from "$server/util/collections";
	import PageTop from "$lib/components/PageMenu/PageTop.svelte";

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
  <PageTop>
    <svelte:fragment slot="title">
      Analytics
    </svelte:fragment>
  </PageTop>
  
  <div class="boxes">
    <div class="filters">
      <AnalyticsFilters
        {loading}
        stations={data.stations.items}
        selected_stations="all"
        kind="last-30d"
        {on_submit}
        locale={$locale.analytics.filters}
      />
    </div>

    {#if analytics}
      <div class="analytics" class:loading>
        {#key hash(analytics)}
          <Analytics
            data={analytics}
            country_names={$locale.countries}
            lang={$lang}
            locale={$locale.analytics}
            stats_map_locale={$locale.stats_map}
          />
        {/key}
      </div>
    {/if}
  </div>
</Page>