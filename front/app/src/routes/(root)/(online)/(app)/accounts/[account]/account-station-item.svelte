<script lang="ts">
	import { page } from "$app/stores";
	import { locale } from "$lib/locale";
	import { get_now_playing_store } from "$lib/now-playing";
	import { intersect } from "$share/actions";
	import { ripple } from "$share/ripple";
	import { onMount } from "svelte";

  export let station: import("$server/defs/PublicStation").PublicStation;
  export let now_playing: import("$api/stations/[station]/now-playing/GET/Output").Output | undefined = undefined;

  $: on_air = now_playing && !(now_playing.kind === "none" && !now_playing.start_on_connect);

  let store: ReturnType<typeof get_now_playing_store> | null;
  let unsub: (() => void) | null = null;

  onMount(() => {
    store = get_now_playing_store(station._id, now_playing || null);
    unsub = store.subscribe(v => {
      now_playing = v?.info || undefined;
    })

    return () => {
      if(unsub) unsub();
    }
  })

  const enter = () => {
    if(store != null) return;
    store = get_now_playing_store(station._id, now_playing || null);
    unsub = store.subscribe(v => {
      now_playing = v?.info || undefined;
    })
  }

  const leave = () => {
    if(unsub == null) return;
    unsub();
    unsub = null;
    store = null;
  }
</script>

<style>
  .station {
    border-top: 1px var(--red) solid;
    display: flex;
    flex-direction: row;
    align-items: center;
    background: #fff;
    box-shadow: var(--some-shadow);
  }

  .pic {
    width: min(30%, 8rem);
    aspect-ratio: 1;
    margin: 1rem 2rem 1rem 1rem;
    border-radius:  0.5rem;;
    background-size: contain;
    background-position: center;
    background-repeat: no-repeat;
  }
  
  .name {
    font-size: 1.1rem;
  }

  .now-playing {
    margin-top: 0.4rem;
    font-size: 0.8rem;
  }

  .now-playing-state {
    font-weight: 900;
  }

  .on-air .now-playing-state {
    color: var(--green);
  } 

  .off-air .now-playing-state {
    color: var(--red);
  }

  .now-playing-sub {
    margin-top: 0.25rem;
  }
</style>

<a 
  href="/accounts/{station.account_id}/stations/{station._id}"
  class="na station ripple-container"
  use:ripple
  use:intersect={{ enter, leave }}
  class:on-air={on_air}
  class:off-air={!on_air}
>
  <div class="pic" style:background-image="url({$page.data.config.storage_public_url}/station-pictures/webp/128/{station.picture_id}.webp)">
  </div>
  <div class="data">
    <div class="name">{station.name}</div>
    <div class="now-playing">
      {#if now_playing}
        <div class="now-playing-state">
          {#if on_air}
            {$locale.pages["account/dashboard"].station_item.on_air}
          {:else}
            {$locale.pages["account/dashboard"].station_item.off_air}
          {/if}
        </div>
        {#if now_playing.kind === "none"}
          {#if now_playing.start_on_connect}
            <div class="now-playing-sub">
              {#if now_playing.external_relay_url != null}
                <!-- TODO: locale -->
                Relay
              {:else}
                {$locale.pages["account/dashboard"].station_item.playlist}
              {/if}
            </div>
          {/if}
        {:else}
          <div class="now-playing-sub">
            {#if now_playing.kind === "live"}
              {$locale.pages["account/dashboard"].station_item.live}
            {:else if now_playing.kind === "playlist"}
              {$locale.pages["account/dashboard"].station_item.playlist}
            {:else if now_playing.kind === "external-relay"}
              <!-- TODO: locale -->
              Relay
            {/if}
          </div>
        {/if}
      {/if}
  </div>
</a>