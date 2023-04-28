<script lang="ts">
  export let data: import("./$types").PageData;
  import Page from "$lib/components/Page.svelte";
	import type { Station } from "$server/defs/db/Station";
	import StatsMap from "$share/Map/StatsMap.svelte";
	import type { Stats } from "$share/Map/StatsMap.svelte";
	import { click_out } from "$share/actions";
	import { _get, action } from "$share/net.client";
	import { ripple } from "$share/ripple";
	import { fly } from "svelte/transition";

  let selector_state: { kind: "account" | "station", record_id: string, data: Stats, station: typeof data.stations.items[number] | null } = {
    kind: "account",
    record_id: data.account._id,
    data: data.stats,
    station: null,
  };

  import type { View } from "$share/Map/StatsMap.svelte";
  let view: View = "now";

  let _token = 0;

  const select = action(async (station: typeof data.stations.items[number] | null) => {
    selector_open = false;
    if(station?._id === selector_state.station?._id) return;
    const token = ++_token;
    if(station) {
      const { stats }: import("$server/defs/api/stations/[station]/stream-stats/GET/Output").Output =
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
      const { stats }: import("$server/defs/api/accounts/[account]/stream-stats/GET/Output").Output =
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

  $: account_stations = data.stations.items.filter(item => item.account_id === data.account._id);

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
</script>

<style>

  h1 {
    font-weight: 600;
  }

  .stats {
    margin-top: 2rem;
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
    margin-inline-end: 0.75rem;
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
    margin-inline-end: 1rem;
    margin-inline-start: -0.5rem;
  }

  .stats-selector-anchor {
    position: absolute;
    left: 0;
    bottom: 0;
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

<svelte:head>
  <title>{data.account.name}</title>
</svelte:head>

<Page>
  <h1>{data.account.name}</h1>

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
          <div class="stats-selector-btn-text">
            {#if selector_state.station}
              {selector_state.station.name}
            {:else}
              All stations
            {/if}
          </div>
          ▼
        </button>
        <div class="stats-selector-anchor">
          {#if selector_open}
            <div 
              class="stats-selector-menu"
              use:click_out={selector_menu_click_out}
              transition:fly|local={{ duration: 125, y: -10 }}
            >
              <button class="stats-selector-item" class:current={selector_state.station == null} on:click={() => select(null)}>
                <div class="stats-selector-name">
                  All stations
                </div>
              </button>
              {#each account_stations as station (station._id)}
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
    <StatsMap bind:view kind={selector_state.kind} record_id={selector_state.record_id} bind:data={selector_state.data} />
  </div>
</Page>