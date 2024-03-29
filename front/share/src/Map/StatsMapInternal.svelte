<script lang="ts" context="module">
  const logger = default_logger.scoped("stats-map");
  import type { View } from "./StatsMap.svelte";
</script>

<script lang="ts">
  export let data: Stats | null = null;
  export let kind: "all" | "account" | "station";
  // TODO: use generics
  export let record_id: string;
  export let view: View = "now"; 
  export let in_screen = true;
  export let locale: import("$server/locale/share/stats-map/stats-map.locale").StatsMapLocale;
  export let country_names: Record<string, string | undefined>;

  import { default_logger } from "$share/logger";
  import { _get } from "$share/net.client";
  import { sleep } from "$share/util";
  import { onMount } from "svelte";
  import Map from "./Map.svelte";
  import { ripple } from "$share/ripple";
  import { intersect } from "$share/actions";
  import { GET, unwrap } from "$share/client";

  let view_ids = ["now", "last_24h", "last_7d", "last_30d"] as const;
  let selector_titles = {
    "now": locale.now,
    "last_24h": locale["24_hours"],
    "last_7d": locale["7_days"],
    "last_30d": locale["30_days"],
  } as const;

  type Stats = import("$server/defs/stream-connection-stats/Stats").Stats;
  //type StatsItem = import("$server/defs/stream-connection-stats/StatsItem").StatsItem;

  onMount(() => {
    let mounted = true;
    let last = Date.now();
    let last_all = Date.now();

    const start_now_timer = async () => {
      let last = Date.now();
      let paused = false;
      while(true) {
        await sleep(250);
        if(!mounted) break;
        if(data == null) continue;
        if(document.visibilityState === "hidden" || in_screen === false) {
          if(!paused) {
            paused = true;
            logger.info(`pausing stream stats auto update for ${kind} ${record_id} (document: ${document.visibilityState}, element in screen: ${in_screen})`)
          }
          continue;
        } else {
          if(paused) {
            paused = false;
            logger.info(`(re)starting stream stats auto update for ${kind} ${record_id} (document: ${document.visibilityState}, element in screen ${in_screen})`)
          } 
        };
        if(Date.now() - last < 10_000) continue;
        if(Date.now() - last_all > 1000 * 60 * 60 * 30) { // 30min
          // update all stats every 30 min
          load()
          break;
        }

        try {
          const output = unwrap(await (
            kind === "all" ? GET("/stream-stats/now") : 
            kind === "account" ? GET("/accounts/{account}/stream-stats/now", { params: { path: { account: record_id } } }) : 
            GET("/stations/{station}/stream-stats/now", { params: { path: { station: record_id } } }) 
          ));
        
          last = Date.now();
        
          if(data) {
            data.now = output.stats;
          } 

          logger.info(`now stats updated for ${kind} ${record_id}`, output.stats)
        } catch(e: any) {
          logger.error(`error updating now stats for ${kind} ${record_id}`);
          logger.error(e);
          break;
        }
      }
    }

    const load = async () => {
      if(!mounted) return;
      if(data == null) {
        try {
          const output = unwrap(await (
            kind === "all" ? GET("/stream-stats") : 
            kind === "account" ? GET("/accounts/{account}/stream-stats", { params: { path: { account: record_id } } }) : 
            GET("/stations/{station}/stream-stats", { params: { path: { station: record_id } } })
          ));

          logger.info(`loaded stats map data for ${kind} ${record_id}`, output.stats);
          data = output.stats;
          last = Date.now();
          last_all = Date.now();
          start_now_timer();
        } catch(e: any) {
          logger.error(`failed to load stream stats for ${kind} ${record_id}, retrying in 1s`)
          logger.error(e);
          await sleep(1000);
          await load();
          return;
        }
      } else {
        last = Date.now();
        last_all = Date.now();
        start_now_timer();
      }
    }

    load();

    return () => {
      mounted = false;
    }

  })
</script>

<style>
  .stats-map {
    display: flex;
    flex-direction: column;
    container-type: inline-size;
    container-name: stats-map;
  }

  .stats-map-display {
    flex: 1;
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 1rem;
    --spacing: 1rem;
    --map-max-width: 800px;
  }

  .map-out {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }

  .view-selector {
    flex: none;
    display: flex;
    flex-direction: column;
    align-items: stretch;
    padding: var(--spacing) 0;
  }

  .view-btn {
    padding: var(--spacing) calc(var(--spacing) * 2) var(--spacing) var(--spacing);
    display: flex;
    flex-direction: column;
    text-align: start;
    align-items: flex-start;
    transition: background-color 300ms ease;
    background-color: transparent;
  }

  .view-btn.selected {
    background-color: rgba(0,0,0,0.075);
  }

  .view-title {
    font-size: 1.25rem;
    font-weight: var(--font-semi); 
  }

  .counter {
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
    font-size: 0.9rem;
  }

  @container stats-map (width < 770px) {
    .stats-map-display {
      flex-direction: column;
    }

    .map-out {
      width: 100%;
    }

    .view-selector {
      align-self: stretch;
      flex-direction: row;
      gap: 0.25rem;
      padding: 1rem;
    }

    .view-btn {
      flex: 1;
      border-radius: 0.5rem;
    }

    .counter {
      font-size: 0.8rem;
    }
  }

  @container stats-map (width < 600px) {
    .stats-map-display {
      flex-direction: column;
    }

    .view-selector {
      flex-wrap: wrap;
    }

    .view-btn {
      flex-grow: 1;
      flex-basis: 49%;
    }

    .counter {
      font-size: 0.8rem;
    }
  }
</style>

<div class="stats-map" use:intersect={{ enter: () => in_screen = true, leave: () => in_screen = false }}>
  <div class="stats-map-display">
    <div class="view-selector">
      {#if data != null}
        <!-- fix tscheck error -->
        {@const data_non_null = data}
        {#each view_ids as view_id}
          {@const stats = data_non_null[view_id]}
          {@const selected = view_id === view}
          {@const sessions = stats.sessions}
          {@const countries = Object.keys(stats.country_sessions).length}
          {@const count_names = view_id === "now" ? [locale.listener, locale.listeners] : [locale.session, locale.sessions]}
          <button class="view-btn ripple-container" class:selected use:ripple on:click={() => view = view_id}>
            <div class="view-title">
              {selector_titles[view_id]}
            </div>
            <div class="counters">
              <div class="counter">
                <span class="counter-num">{sessions}</span>
                <span class="counter-label">{sessions === 1 ? count_names[0] : count_names[1]}</span>
              </div>
              <div class="counter">
                <span class="counter-num">{countries}</span>
                <span class="counter-label">{countries === 1 ? locale.country : locale.countries}</span>
              </div>
            </div>
          </button>
        {/each}
      {/if}
    </div>
    <div class="map-out">
      {#if data != null}
        <Map is_now={view === "now"} stats={data[view]} {country_names} {locale} />
      {/if}
    </div>
  </div>
</div>