<script lang="ts">
	import CircularProgress from "$share/CircularProgress.svelte";
  import Icon from "$share/Icon.svelte";
	import { ripple } from "$share/ripple";
	import { mdiClose, mdiPause, mdiPlay } from "@mdi/js";
	import { slide } from "svelte/transition";
	import { player_state, player_title, player_subtitle, player_audio_state, pause, resume, close, player_picture_id } from "./player";
	import { expoInOut } from "svelte/easing";

  import { page } from "$app/stores";
  
  $: state = $player_state;
  $: title = $player_title;
  $: subtitle = $player_subtitle;
  $: audio_state = $player_audio_state;

  $: toggle_aria_label = audio_state === "loading" ? "Loading" : audio_state === "paused" ? "Play" : "Pause";

  const toggle = () => {
    if(audio_state === "playing") pause();
    else if(audio_state === "loading") pause();
    else if(audio_state === "paused") resume();
  }

  const transition = (node: HTMLElement) => {
    return () => {
      const h = node.clientHeight;
      return {
        css: (t: number, u: number) => `opacity: ${t}; margin-block-end: -${u * h}px`,
        easing: expoInOut, 
        duration: 350,
      }
    }
  }
</script>

<style>
  .player-holder {
    display: flex;
    position: sticky;
    bottom: 0;
    z-index: var(--z-player);
    padding-top: 1rem;
    margin-top: -1rem;
    pointer-events: none;
    overflow: hidden;
  }

  .player {
    display: flex;
    flex: 1;
    pointer-events: all;
    flex-direction: row;
    align-items: center;
    box-shadow: rgba(0,0,0,0.25) 0 0 6px;
    background: #fff;
    height: var(--player-h);
  }

  .info {
    flex: 1;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-start;
  }

  .pic {
    width: 2.25rem;
    height: 2.25rem;
    margin-inline-end: 1rem;
    background-size: contain;
    background-position: center;
    background-repeat: no-repeat;
    border-radius: 0.25rem;
  }

  .titles {
    flex: 1;
  }

  .title > span, .subtitle > span {
    display: block;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .title {
    font-size: 1rem;
    font-weight: 600;
  }

  .subtitle {
    font-size: 0.9rem;
    color: #555;
    margin-top: 0.25rem;
  }

  .toggle-out {
    flex: none;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0 0.5rem;
  }

  .toggle {
    flex: none;
    width: 3.5rem;
    height: 3.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 2rem;
    color: #333;
    border-radius: 0.25rem;
    transition: background-color 150ms ease;
  }

  .toggle[data-audio-state="loading"] {
    font-size: 1.5rem;
  }

  .toggle:hover {
    background-color: rgba(0,0,0,0.05);
  }


  .btns {
    flex: none;
    display: flex;
    flex-direction: row;
    align-items: center;
    padding: 0 1rem;
  }

  .btn {
    width: 3rem;
    height: 3rem;
    flex: none;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #333;
    font-size: 1.5rem;
    border-radius: 0.25rem;
    transition: background-color 150ms ease;
  }

  .btn:hover {
    background-color: rgba(0,0,0,0.05);
  }

</style>

{#if state.type !== "closed"}
  <div class="player-holder">
    <div class="player" aria-label="Player" transition:transition|local>

      <div class="toggle-out">
        <button use:ripple class="toggle ripple-container" aria-label={toggle_aria_label} data-audio-state={audio_state} on:click={toggle}>
          {#if audio_state === "paused"}
            <Icon d={mdiPlay} />
          {:else if audio_state === "playing"}
            <Icon d={mdiPause} />
          {:else}
            <!-- audio_state: loading -->
            <CircularProgress />
          {/if}
        </button>
      </div>

      <div class="info">
        <div class="pic" style="background-image: url({$page.data.config.storage_public_url}/station-pictures/webp/64/{$player_picture_id}.webp)" /> 
        <div class="titles">
          <div class="title">
            <span>
              {title}
            </span>
          </div>
          {#if subtitle}
            <div class="subtitle" transition:slide|local={{ duration: 200 }}>
              <span>{subtitle}</span>
            </div>
          {/if}
        </div>
      </div>

      <div class="btns">
        <button use:ripple class="btn close ripple-container" aria-label="Close player" on:click={close}>
          <Icon d={mdiClose} />
        </button>
      </div>
    </div>
  </div>
{/if}