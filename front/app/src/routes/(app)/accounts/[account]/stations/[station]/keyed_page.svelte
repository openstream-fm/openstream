<script lang="ts">
  import CircularMeter from "$lib/components/CircularMeter/CircularMeter.svelte";
  import Page from "$lib/components/Page.svelte";
	import { pause, player_state, play_station } from "$lib/components/Player/player";
	import { default_logger } from "$lib/logger";
	import { get_now_playing_store } from "$lib/now-playing";
	import type { StationLimits } from "$server/defs/StationLimits";
	import CircularProgress from "$share/CircularProgress.svelte";
	import Icon from "$share/Icon.svelte";
	import { _get } from "$share/net.client";
	import { ripple } from "$share/ripple";
	import { mdiMicrophoneOutline, mdiPause, mdiPlay } from "@mdi/js";
	import { onMount } from "svelte";
	import { derived } from "svelte/store";

  export let data: import("./$types").PageData;

  const logger = default_logger.scoped("dashboard");

  const now_playing = get_now_playing_store(data.station._id, data.now_playing);
  $: if($now_playing) data.now_playing = $now_playing.info;

  $: on_air = $now_playing!.info.kind === "none" ? $now_playing!.info.start_on_connect : true;
  
  const station_preview_state = derived(player_state, (state): "loading" | "paused" | "playing" => {
    if(state.type === "station") {
      if(data.station?._id && data.station._id === state.station._id) return state.audio_state;
      else return "paused";
    } else {
      return "paused";
    }
  })

  const toggle_play = () => {
    if($station_preview_state === "playing" || $station_preview_state === "loading") pause();
    else play_station({ _id: data.station._id, name: data.station.name })
  }

  const stats_num = (v: number): string => {
    if(v < 1_000) return `${v}`;
    if(v < 1_000_000) return `${(v / 1_000).toFixed(1)} K`;
    if(v < 1_000_000_000) return `${(v / 1_000_000).toFixed(1)} M`;
    return `${(v / 1_000_000_000).toFixed(1)} B`
  }

  const units = [ "B", "KB", "MB", "GB", "TB" ];
  
  const to_fixed_2 = (v: number): number => Math.round(v * 100) / 100; 

  const preety_bytes = (_v: number): string => {
    
    let v = _v;

    for(const unit of units) {
      if(v < 1000) {
        return `${to_fixed_2(v)} ${unit}`;
      } 
      v = v / 1000;
    }

    return `${to_fixed_2(v)} PB`;
  }

  const UPDATE_INTERVAL = 5_000;

  onMount(() => {

    const update = async () => {
      
      const token = timer;

      const skip = document.hidden === true;

      if(skip) {
        logger.info("skipping update tick because of document.hidden");
      } else {
        try {
          const limits: StationLimits = await _get(`/api/stations/${data.station._id}/limits`);
          logger.info(`station limits updated`);
          data.station.limits = limits;
        } catch(e) {
          logger.warn(`error updating station limits: ${e}`);
        }
      }

      if(token === timer) {
        timer = setTimeout(update, skip ? 1000 : UPDATE_INTERVAL);
      }
    }

    let timer = setTimeout(update, UPDATE_INTERVAL);
    
    return () => clearTimeout(timer);
  })
</script>

<style>

  .meters {
    display: flex;
    flex-direction: row;
    gap: 1rem;
    align-items: stretch;
  }

  .meter {
    background: #fff;
    flex: 1;
    padding: 2rem 1rem;
    border-radius: 0.5rem;
    text-align: center;
    box-shadow: 0 20px 25px -5px rgba(0,0,0,.1),0 10px 10px -5px rgba(0,0,0,.04);
  }

  .meter-title {
    font-weight: 600;
    font-size: 2em;
  }

  .meter-text {
    color: #333;
    font-size: 1.5em;
  }

  .used, .avail {
    font-weight: 600;
  }

  .used {
    color: var(--red);
  }

  .avail {
    color: #333;
  }

  .of {
    color: #999;
    font-size: 0.8em;
  }

  .meter-graph {
    max-width: 15rem;
    margin: 0 auto;
  }

  @media screen and (max-width: 1160px) {
    .meter {
      font-size: 0.8rem;
    }
  }

  .top-boxes {
    margin-bottom: 1.5rem;
    display: flex;
    flex-direction: row;
    gap: 2rem;
  }

  .top-box {
    padding: 2rem;
    border-radius: 0.5rem;
    box-shadow: 0 20px 25px -5px rgba(0,0,0,.1),0 10px 10px -5px rgba(0,0,0,.04);
    background: #fff;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    flex: 1;
  }

  .air-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    background: #EAF2E0;
    padding: 1rem;
    border-radius: 50%;
    color: var(--green);
    font-size: 3rem;
  }

  .air-title {
    font-weight: 700;
    font-size: 2rem;
    margin-top: 1rem;
    white-space: nowrap;
  }

  .air-subtitle {
    color: #444;
    margin-top: 1rem;
  }

  .top-box-air.on .air-icon {
    color: var(--green);
  }
  .top-box-air.on .air-title {
    color: var(--green);
  }
  

  .top-box-air.off .air-icon {
    color: var(--red);
  }

  .top-box-air.off .air-title {
    margin-top: 1.25rem;
    color: var(--red);
  }

  .preview-btn {
    display: flex;
    flex: none;
    align-items: center;
    justify-content: center;
    width: 6rem;
    height: 6rem;
    font-size: 3.75rem;
    border-radius: 50%;
    background-color: #EAF2E0;
    color: var(--green);
    cursor: pointer;
  }

  .preview-btn[data-state="loading"] {
    font-size: 3rem;
  }

  .preview-title {
    display: flex;
    font-size: 1.25rem;
    flex: 1;
    align-items: center;
    justify-content: center;
    text-align: center;
    margin-top: 1.5rem;
  }

  .top-box-stats {
    align-items: stretch;
  }

  .stats-title {
    font-size: 1.4rem;
    font-weight: 600;
  }

  .stats-label {
    color: #666;
    font-size: 0.8rem;
    text-align: left;
  }

  .stats-items {
    margin-top: 0.5rem;
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: stretch;
    justify-content: space-around;
  }

  .stats-item {
    display: block;
    margin-top: 0.5rem;
  }

  .stats-value {
    font-size: 1.2rem;
    color: var(--green);
    font-weight: 600;
  }

  .top-boxes[data-air="off"] > .top-box-preview {
    visibility: hidden;
    order: 3;
  }

  @media screen and (max-width: 700px) {
    .top-boxes {
      flex-direction: column;
      gap: 1rem; 
    }

    .top-boxes[data-air="off"] > .top-box-preview {
      display: none;
    }

    .meters {
      flex-direction: column;
    }

    .meter-graph {
      max-width: 10rem;
    }
  } 
</style>

<svelte:head>
  <title>{data.station.name}</title>
</svelte:head>

<Page>

  <div class="top-boxes" data-air={on_air ? "on" : "off"}>
    <div class="top-box top-box-air" class:on={on_air} class:off={!on_air}>
      <div class="air-icon">
        <Icon d={mdiMicrophoneOutline} />
      </div>
      <div class="air-title">
        {#if on_air}
          <span class="on-air">ON AIR</span>
        {:else}
          <span class="off-air">OFF AIR</span>
        {/if}
      </div>
      {#if on_air}
        <div class="air-subtitle">
          {#if data.now_playing.kind === "playlist" || data.now_playing.kind === "none"}
            Playlist
          {:else if data.now_playing.kind === "live"}
            Live Streaming
          {/if}
        </div>
      {/if}
    </div>

    <div class="top-box top-box-preview">
      <button
        use:ripple class="preview-btn ripple-container"
        data-state={$station_preview_state}
        on:click={toggle_play}
        aria-label={$station_preview_state === "playing" ? "Pause" : "Play"}
      >
        {#if $station_preview_state === "playing"}
          <Icon d={mdiPause} />
        {:else if $station_preview_state === "paused"}
          <Icon d={mdiPlay} />
        {:else}
          <!-- "loading" -->
          <CircularProgress />
        {/if}
      </button>

      <div class="preview-title">
        Preview
      </div>
    </div>

    <div class="top-box top-box-stats">
      <div class="stats-title">
        Sessions
      </div>
      <div class="stats-items">
        <div class="stats-item">
          <div class="stats-label">24 hours</div>
          <div class="stats-value">{stats_num(data.dashboard_stats.listeners_last_24h)}</div>
        </div>
        <div class="stats-item">
          <div class="stats-label">7 days</div>
          <div class="stats-value">{stats_num(data.dashboard_stats.listeners_last_7d)}</div>
        </div>
        <div class="stats-item">
          <div class="stats-label">30 days</div>
          <div class="stats-value">{stats_num(data.dashboard_stats.listeners_last_30d)}</div>
        </div>
      </div>
    </div>
  </div>

  <div class="meters">
    <div class="meter">
      <div class="meter-title">
        Listeners
      </div>
      <div class="meter-graph">
        <CircularMeter used={data.station.limits.listeners.used / data.station.limits.listeners.total} />
      </div>
      <div class="meter-text">
        <span class="used">{data.station.limits.listeners.used}</span>
        <span class="of">of</span>
        <span class="avail">{data.station.limits.listeners.total}</span>
      </div>
    </div>
    <div class="meter">
      <div class="meter-title">
        Transfer
      </div>
      <div class="meter-graph">
        <CircularMeter used={data.station.limits.transfer.used / data.station.limits.transfer.total} />
      </div>
      <div class="meter-text">
        <span class="used">{preety_bytes(data.station.limits.transfer.used)}</span>
        <span class="of">of</span>
        <span class="avail">{preety_bytes(data.station.limits.transfer.total)}</span>
      </div>
    </div>
    <div class="meter">
      <div class="meter-title">
        Storage
      </div>
      <div class="meter-graph">
        <CircularMeter used={data.station.limits.storage.used / data.station.limits.storage.total} />
      </div>
      <div class="meter-text">
        <span class="used">{preety_bytes(data.station.limits.storage.used)}</span>
        <span class="of">of</span>
        <span class="avail">{preety_bytes(data.station.limits.storage.total)}</span>
      </div>
    </div>
  </div>
</Page>