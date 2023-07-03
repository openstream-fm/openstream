<script lang="ts" context="module">
  export type AllKind =
    | { all_kind: "all" }
    | { all_kind: "account", account_id: string }

  export type Selection = { kind: "all" | "account" | "station", record_id: string, station: Station | null };

  export type Station = { _id: string, name: string, picture_id: string };

  export type Data = 
    & { 
      stats: Stats | null,
      stations: Station[]
      storage_public_url: string,
    } 
    & AllKind
    & Selection;
</script>

<script lang="ts">
  export let data: Data;

  $: console.log({ data });

	import type { Stats } from "$share/Map/StatsMap.svelte";
	import { click_out } from "$share/actions";
	import { _get, _patch, action } from "$share/net.client";
	import { ripple } from "$share/ripple";
  
	import { logical_fly } from "$share/transition";

  let _token = 0;

  let selector_open = false;

  const select = action(async (station: Station | null) => {
    selector_open = false;
    if(station && data.kind === "station" && data.record_id === station._id) return;
    if(station == null && (data.kind !== "station")) return;
    const token = ++_token;
    if(station) {
      const { stats }: import("$api/stations/[station]/stream-stats/GET/Output").Output =
        await _get(`/api/stations/${station._id}/stream-stats`);
      if(token === _token) {
        data = { ...data, kind: "station", record_id: station._id, station, stats };
      }
    } else if(data.all_kind === "account") {
      const { stats }: import("$api/accounts/[account]/stream-stats/GET/Output").Output =
        await _get(`/api/accounts/${data.account_id}/stream-stats`);
      if(token === _token) {
        data = { ...data, kind: "account", record_id: data.account_id, station: null, stats };
      }
    } else if(data.all_kind === "all") {
      const { stats }: import("$api/stream-stats/GET/Output").Output =
        await _get(`/api/stream-stats`);
     if(token === _token) {
        data = { ...data, kind: "all", record_id: "", station: null, stats }; 
      }
    }
  })

  
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
  .selector-out {
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

<div class="selector-out">
  <div class="stats-selector">
    <button class="stats-selector-btn ripple-container" class:open={selector_open} use:ripple aria-label={"Select one station or all"} on:click={toggle_selector}>
      {#if data.station != null}
        <div
          class="stats-selector-btn-icon"
          style:background-image="url({data.storage_public_url}/station-pictures/webp/64/{data.station.picture_id}.webp)"
        />
      {/if}
      <span class="stats-selector-btn-text">
        {#if data.station != null}
          {data.station.name}
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
          <button class="stats-selector-item" class:current={data.station == null} on:click={() => select(null)}>
            <div class="stats-selector-name">
              All stations
            </div>
          </button>
          {#each data.stations as station (station._id)}
            <button class="stats-selector-item" class:current={data.station?._id === station._id} on:click={() => select(station)}>
              <div class="stats-selector-icon" style:background-image="url({data.storage_public_url}/station-pictures/webp/64/{station.picture_id}.webp)" />
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
