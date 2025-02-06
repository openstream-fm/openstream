<script lang="ts">
  let {
    data
  }: {
    data: import("./$types").PageData
  } = $props();
  
  import "$lib/font/fira-sans/font.css";
  import { mdiPause, mdiPlay } from "@mdi/js";
  import Loading from "./loading.svelte";
  import Icon from "./icon.svelte";
  import { onMount } from "svelte";
  import SILENCE from "./1-second-of-silence.mp3"

  let state = $state<"paused" | "loading" | "playing">("paused")
  const subtitle = $derived(data.station.now_playing ?? data.station.slogan);

  let audio!: HTMLAudioElement;

  const src = `https://stream.openstream.fm/stream/${data.station.id}`;

  const update = async () => {
    const res = await fetch(`/station/${data.station.id}/data.json`);
    const json = await res.json();
    if(res.ok) {
      data = { ...data, station: json };
    }
  }

  $effect(() => {
    if(window.MediaSession != null && navigator.mediaSession) {
      navigator.mediaSession.playbackState = 
        state === "playing" ? "playing" :
        state === "loading" ? "playing" :
        state === "paused" ? "paused" :
        "none";
    }
  })

  $effect(() => {
    if(window.MediaSession != null && navigator.mediaSession) {
      navigator.mediaSession.metadata = new MediaMetadata({
        title: data.station.name,
        artist: data.station.now_playing ?? data.station.slogan ?? undefined,
        artwork: [
          {
            src: `https://storage.openstream.fm/station-pictures/png/512/${data.station.picture_id}.png`,
            sizes: "512x512",
            type: "image/png"
          }
        ]
      });
    }
  })  

  onMount(() => {
    if(window.MediaSession != null && navigator.mediaSession) {
      navigator.mediaSession.setActionHandler("play", play);
      navigator.mediaSession.setActionHandler("pause", stop);
      navigator.mediaSession.setActionHandler("stop", stop);
      navigator.mediaSession.setActionHandler("seekbackward", null);
      navigator.mediaSession.setActionHandler("seekforward", null);
      navigator.mediaSession.setActionHandler("previoustrack", null);
      navigator.mediaSession.setActionHandler("nexttrack", null);
      navigator.mediaSession.setActionHandler("seekto", null);
    }
  })

  onMount(() => {
    audio = new Audio();
    audio.onpause = () => state = "paused";
    audio.onplaying = () => state = "playing";
    audio.onplay = () => state = "loading";
    audio.onstalled = () => state = "loading";
  })

  onMount(() => {
    let timer: any;
    const fn = () => {
      update();
      timer = setTimeout(fn, 10_000);
    }
    timer = setTimeout(fn, 10_000)
    return () => clearTimeout(timer)
  })

  const stop = () => {
    state = "paused";
    audio.pause();
    audio.currentTime = 0;
    audio.src = SILENCE;
  }

  const play = () => {
    audio.src = src;
    audio.load();
    audio.play();  
  }

  const toggle = () => {
    if (state === "paused") {
      play();
    } else if(state === "loading" || state === "playing") {
      stop();
    }
  }
</script>

<style>
  :global {
    * {
      box-sizing: border-box;
      min-width: 0;
    }

    button {
      appearance: none;
      border: none;
      background: none;
      padding: 0;
      margin: 0;
    }

    html {
      font-family: "Fira Sans", sans-serif;
    }

    html, body {
      margin: 0;
      padding: 0;
      width: 100%;
      height: 100%;
      display: flex;
      flex: 1;
      overflow: hidden;
    }
  }

  .page {
    flex: 1;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    background: #eee;
    justify-content: space-evenly;
    overflow: hidden;

    @media screen and (orientation: portrait) {
      flex-direction: column;
    }
  }

  .sep {
    min-width: 1.5rem;
    flex: 3;
    
    &.between {
      min-width: 1rem;
      flex: 1;
    }
  }


  .img {
    --size: min(75vmin, 30vmax);
    height: var(--size);
    width: var(--size);
    display: flex;
  }

  img {
    flex: 1;
    width: 100%;
    height: 100%;
    object-fit: contain;
    background: #fff;
    border-radius: 7.5%;
  } 

  .toggle {
    --size: max(2rem, min(40vmin, 15vmax));
    --icon-size: 65%;
    flex: none;
    width: var(--size);
    height: var(--size);
    background: #1C203C;
    color: #fff;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: rgba(0,0,0,0.25) 0 0 0.5rem 0.1rem; 
    
    &.loading {
      --icon-size: 55%;
    }
  }

  .texts {
    display: flex;
    flex-direction: column;
    @media screen and (orientation: portrait) {
      text-align: center;
      align-items: center;
      gap: 0.5rem;
    }
  }

  .texts {
    font-size: max(0.9rem, min(15vmin, 5vmax));
  }

  .title {
    color: #1C203C;
    font-size: 1em;
    font-weight: 500;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .subtitle {
    font-size: 0.8em;
    color: #1C203C;
    opacity: 0.75;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  } 
</style>

<div class="page">
  <div class="sep"></div>

  <div class="img">
    <img src="https://storage.openstream.fm/station-pictures/webp/512/{data.station.picture_id}.webp" alt="{data.station.name}" />
  </div>

  <div class="sep between"></div>

  <button class="toggle" class:loading={state === "loading"} onclick={toggle}>
    {#if state === "loading"}
      <Loading />
    {:else if state === "paused"}
      <Icon d={mdiPlay} />
    {:else}
      <Icon d={mdiPause} />
    {/if}
  </button>
  
  <div class="sep between"></div>

  <div class="texts">
    <div class="title">
      {data.station.name}
    </div>
    {#if subtitle}
      <div class="subtitle">
        {subtitle}
      </div>
    {/if}
  </div>

  <div class="sep"></div>

</div>