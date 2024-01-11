<script lang="ts">
  import Page from "$lib/components/Page.svelte";
	import { pause, player_state, play_station } from "$lib/components/Player/player";
	import { get_now_playing_store } from "$lib/now-playing";
	import CircularProgress from "$share/CircularProgress.svelte";
	import Icon from "$share/Icon.svelte";
	import { _get } from "$share/net.client";
	import { ripple } from "$share/ripple";
  import { mdiMicrophoneOutline, mdiPause, mdiPlay } from "@mdi/js";
	import { derived } from "svelte/store";
  import StatsMap from "$share/Map/StatsMap.svelte";
	import { locale } from "$lib/locale";

  export let data: import("./$types").PageData;

  const now_playing = get_now_playing_store(data.station._id, data.now_playing);
  $: if($now_playing) data.now_playing = $now_playing.info;

  $: on_air = is_on_air($now_playing);
	const is_on_air = (now_playing: typeof $now_playing) => {
		const info = now_playing?.info;
    if (info == null) return null;
		if (info.kind === "external-relay") return true;
		if (info.kind === "live") return true;
		if (info.kind === "playlist") return true;
		if (info.kind === "none") {
			if (info.start_on_connect && info.external_relay_error == null) return true;
			else return false;
		};
	}

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
    else play_station({ _id: data.station._id, picture_id: data.station.picture_id, name: data.station.name })
  }
</script>

<style>

  .page {
    display: flex;
    flex-direction: column;
    --spacing: 1.5rem;
    gap: 1.5rem;
  }

  .top {
    display: flex;
    flex-direction: row;
    gap: var(--spacing);
  }

  .top-boxes {
    display: flex;
    flex-direction: row;
    flex: 1;
    gap: var(--spacing);
  }

  .top-box {
    border-radius: 0.5rem;
    box-shadow: 0 20px 25px -5px rgba(0,0,0,.1),0 10px 10px -5px rgba(0,0,0,.04);
    background: #fff;
    display: flex;
    flex-direction: column;
    flex: 1;
  }

  .stats {
    background: #fff;
    border-radius: 0.5rem;
    box-shadow: 0 20px 25px -5px rgba(0,0,0,.1),0 10px 10px -5px rgba(0,0,0,.04);
  }

  .top-box-air {
    flex: 1;
    padding: 2rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
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

  .top-box-preview {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 2rem; 
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
    font-size: 1rem;
    flex: 1;
    align-items: center;
    justify-content: center;
    text-align: center;
    margin-top: 1.5rem;
    color: #444;
  }

  .top-boxes[data-air="off"] > .preview-out {
    visibility: hidden;
    order: 3;
  }

  .external-relay-error {
    color: var(--red);
  }

  @media screen and (max-width: 700px) {
    .top-boxes {
      flex-direction: column;
    }

    .top-box {
      order: 2;
    }

    .top-box-broadcast {
      order: 1;
    }

    .top-boxes[data-air="off"] > .preview-out {
      display: none;
    }
  } 

  .top-box-broadcast {
    padding: 0;
    display: flex;
  }
  
  .broadcast-btn {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    flex: 1;
    align-self: stretch;
    justify-self: stretch;
    text-align: center;
    color: #fff;
    background: #fff;
    border: var(--blue) 1px solid;
    color: var(--blue);
    font-size: 1.1rem;
    padding: 1rem 0;
    border-radius: 0.5rem;   
    text-align: center;
  } 
</style>

<svelte:head>
  <title>{data.station.name}</title>
</svelte:head>

<Page>

  <div class="page">
    <div class="top">
      <div class="top-boxes" data-air={on_air ? "on" : "off"}>
        <div class="top-box">
          <div class="top-box-air" class:on={on_air} class:off={!on_air}>
            <div class="air-icon">
              <Icon d={mdiMicrophoneOutline} />
            </div>
            <div class="air-title">
              {#if on_air}
                <span class="on-air">{$locale.pages["station/dashboard"].on_air}</span>
              {:else}
                <span class="off-air">{$locale.pages["station/dashboard"].off_air}</span>
              {/if}
            </div>
            {#if on_air}
              <div class="air-subtitle">
                {#if data.now_playing.kind === "playlist"}
                  {$locale.pages["station/dashboard"].playlist}
                {:else if data.now_playing.kind === "external-relay"}
                  {$locale.misc.Relay}
                {:else if data.now_playing.kind === "live"}
                  {$locale.pages["station/dashboard"].live}
                {:else if data.now_playing.kind === "none"}
                  {#if data.now_playing.external_relay_error != null}
                    <span class="external-relay-error">
                      {$locale.misc.External_relay_error}
                    </span>
                  {:else if data.now_playing.external_relay_url != null}
                    {$locale.misc.Relay}
                  {:else}
                    {$locale.pages["station/dashboard"].playlist}
                  {/if}
                {/if}
              </div>
            {/if}
          </div>
        </div>

        <div class="top-box preview-out">
          <div class="top-box-preview">
            <button
              use:ripple class="preview-btn ripple-container"
              data-state={$station_preview_state}
              on:click={toggle_play}
              aria-label={$station_preview_state === "playing" ? $locale.pages["station/dashboard"].aria_pause : $locale.pages["station/dashboard"].aria_play}
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
              {$locale.pages["station/dashboard"].preview}
            </div>
          </div>
        </div>

        <div class="top-box top-box-broadcast">
          <a class="na broadcast-btn ripple-container" href="/accounts/{data.account._id}/stations/{data.station._id}/broadcast" use:ripple>
            {$locale.pages["station/dashboard"].broadcast}
          </a>
        </div>

      </div>
    </div>

    <div class="stats">
      <StatsMap
        kind="station"
        record_id={data.station._id}
        locale={$locale.stats_map}
        country_names={$locale.countries}
        bind:data={data.stats}
      />
    </div>
  </div>
</Page>