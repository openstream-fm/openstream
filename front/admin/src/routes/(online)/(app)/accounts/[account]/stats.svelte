<script lang="ts">
  export let data: import("./$types").PageData;
  import StatsMap from "$share/Map/StatsMap.svelte";
	import type { Stats } from "$share/Map/StatsMap.svelte";
	import { click_out } from "$share/actions";
	import { _get, _patch, action } from "$share/net.client";
	import { ripple } from "$share/ripple";
  
  let selector_state: { kind: "account" | "station", record_id: string, data: Stats, station: typeof data.stations.items[number] | null } = {
    kind: "account",
    record_id: data.account._id,
    data: data.stats,
    station: null,
  };

  import type { View } from "$share/Map/StatsMap.svelte";
	import { locale } from "$lib/locale";
	import { logical_fly } from "$share/transition";

  let view: View = "now";

  let _token = 0;

  const select = action(async (station: typeof data.stations.items[number] | null) => {
    selector_open = false;
    if(station?._id === selector_state.station?._id) return;
    const token = ++_token;
    if(station) {
      const { stats }: import("$api/stations/[station]/stream-stats/GET/Output").Output =
        await _get(`/api/stations/${station._id}/stream-stats`);
      if(token === _token) {
        selector_state = {
          kind: "station",
          record_id: station._id,
          data: stats,
          station,
        }
      }
    } else {
      const { stats }: import("$api/accounts/[account]/stream-stats/GET/Output").Output =
        await _get(`/api/accounts/${data.account._id}/stream-stats`);
      if(token === _token) {
        selector_state = {
          kind: "account",
          record_id: data.account._id,
          data: stats,
          station: null,
        }
      }
    }
  })

  let selector_open = false;
  
  const close_selector = () => {
    selector_open = false
  }

  const toggle_selector = () => {
    selector_open = !selector_open
  }

  const selector_menu_click_out = () => {
    setTimeout(close_selector, 2);  
  }

  const units = [ "B", "KB", "MB", "GB", "TB" ];
</script>

<style>
  .stats {
    background: #fff;
    border-radius: 0.5rem;
    box-shadow: 0 20px 25px -5px rgba(0,0,0,.1),0 10px 10px -5px rgba(0,0,0,.04);
  }

  .stats-selector-out {
    padding: 0.5rem;
    margin-bottom: -1rem;
  }

  .stats-selector {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
  }

  .stats-selector-btn, .stats-selector-item {
    display: flex;
    flex-direction: row;
    align-items: center;
    padding: 0 1rem;
    height: 3rem;
    border-radius: 0.25rem;
    transition: background-color 200ms ease;
  }

  .stats-selector-btn:hover, .stats-selector-btn.open, .stats-selector-item:hover {
    background: rgba(0,0,0,0.025);
  }

  .stats-selector-btn-text {
    margin-inline-end: 0.35rem;
  }

  .stats-selector-btn-chevron {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .stats-selector-item.current {
    background: rgba(var(--blue-rgb), 0.1);
  }

  .stats-selector-menu {
    min-width: min(80vw, 20rem);
  }
  
  .stats-selector-btn-icon, .stats-selector-icon {
    width: 1.75rem;
    height: 1.75rem;
    border-radius: 0.25rem;
    background-position: center;
    background-size: contain;
    background-repeat: no-repeat;
    flex: none;
    margin-inline-start: -0.5rem;
    margin-inline-end: 0.75rem;
  }

  .stats-selector-anchor {
    position: absolute;
    inset-block-end: 0;
    inset-inline-start: 0;
    width: 0;
    height: 0;
    z-index: 1;
  }

  .stats-selector-menu {
    display: flex;
    flex-direction: column;
    box-shadow: 0 5px 25px 0 rgb(0 0 0 / 10%);
    background: #fff;
    padding: 0.5rem;
    border-radius: 0.5rem;
  }

  .stats-selector-item {
    display: flex;
    flex-direction: row;
    align-items: center;
  }
</style>

  <div class="stats">
    <div class="stats-selector-out">
      <div class="stats-selector">
        <button class="stats-selector-btn ripple-container" class:open={selector_open} use:ripple aria-label={"Select one station or all"} on:click={toggle_selector}>
          {#if selector_state.station != null}
            <div
              class="stats-selector-btn-icon"
              style:background-image="url({data.config.storage_public_url}/station-pictures/webp/64/{selector_state.station.picture_id}.webp)"
            />
          {/if}
          <span class="stats-selector-btn-text">
            {#if selector_state.station}
              {selector_state.station.name}
            {:else}
              All stations
            {/if}
          </span>
          <span class="stats-selector-btn-chevron">
            ▼
            <!-- <Icon d={mdiPlay} /> -->
          </span>
        </button>
        <div class="stats-selector-anchor">
          {#if selector_open}
            <div 
              class="stats-selector-menu"
              use:click_out={selector_menu_click_out}
              transition:logical_fly|local={{ duration: 125, y: -10 }}
            >
              <button class="stats-selector-item" class:current={selector_state.station == null} on:click={() => select(null)}>
                <div class="stats-selector-name">
                  All stations
                </div>
              </button>
              {#each data.account_stations as station (station._id)}
                <button class="stats-selector-item" class:current={selector_state.station?._id === station._id} on:click={() => select(station)}>
                  <div class="stats-selector-icon" style:background-image="url({data.config.storage_public_url}/station-pictures/webp/64/{station.picture_id}.webp)" />
                  <div class="stats-selector-name">
                    {station.name}
                  </div>
                </button>
              {/each}  
            </div>
          {/if}
        </div>
      </div>
    </div>
    
    <StatsMap
      bind:view
      kind={selector_state.kind}
      record_id={selector_state.record_id}
      locale={$locale.stats_map}
      country_names={$locale.countries}
      bind:data={selector_state.data}
    />
  </div>

