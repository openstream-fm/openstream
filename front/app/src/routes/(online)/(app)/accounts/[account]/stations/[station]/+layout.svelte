<script lang="ts">
  export let data: import("./$types").LayoutData;
  import { page } from "$app/stores";
	import { station_stream_url } from "$lib/components/Player/player";
	import { crossfade, fade, fly } from "svelte/transition";
  
  const scroll_into_view = (node: HTMLElement) => {
    // @ts-ignore
    if(node.scrollIntoViewIfNeeded) {
      // @ts-ignore
      node.scrollIntoViewIfNeeded(true);
    }
  }

  const [_enter, _leave] = crossfade({ duration: 300, fallback: (node) => fade(node, { duration: 200 }) });

  const current_enter = (node: HTMLElement) => {
    return _enter(node, { key: null })
  }

  const current_leave = (node: HTMLElement) => {
    return _leave(node, { key: null })
  }
</script>

<style>

  .station-out {
    padding: 1rem 1rem 0 1rem ;
    display: flex;
    align-items: stretch;
    justify-content: stretch;
    flex-direction: column;
  }

  .station-scroll {
    overflow-y: visible;
    overflow-x: auto;
    background: #fff;
    box-shadow: var(--some-shadow);
    border-radius: 0.5rem;
    display: flex;
  }

  .station {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-start;
    flex: none;
    padding: 1rem 1.5rem 1rem 1.5rem;
  }

  .station-pic {
    width: 2rem;
    height: 2rem;
    flex: none;
    border-radius: 0.25rem;
    background-position: center;
    background-size: contain;
    background-repeat: no-repeat;
  }

  .station-name {
    max-width: 10rem;
    display: flex;
    flex-shrink: 1;
    min-width: 4rem;
    margin-inline-start: 0.75rem;
  }

  .station-name, .station-action {
    font-size: 1rem;
  }

  .station-actions {
    display: flex;
    flex-direction: row;
    align-items: center;
    flex: none;
    margin-inline-start: 1rem;
  }

  .station-action {
    display: flex;
    flex-direction: column;
    flex: none;
    color: var(--blue);
    text-decoration: none;
    margin: 0 0.1rem;
    position: relative;
  }

  .action-name {
    padding: 0.6rem 0.75rem;
  }

  /* .station-action.current {
    background: rgba(var(--red-rgb),  0.05);
  } */

  .station-name-ellipsis {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    flex: 1;
    min-width: 0;
  }

  .current-action-line {
    position: absolute;
    top: 0;
    bottom: 0;
    right: 0;
    left: 0;
    /* border-bottom: 1px solid var(--blue); */
    border-radius: 0.25rem 0.25rem;
    pointer-events: none;
    background: rgba(var(--blue-rgb), 0.1);
  }
</style>

{#key data.station._id}
  <div class="station-out" in:fade={{ duration: 200 }}>
    <div class="station-scroll super-thin-scroll">
      <div class="station">
        <div class="station-pic" style="background-image: url({ data.config.storagePublicURL }/station-pictures/webp/64/{data.station.picture_id}.webp)" />
        <div class="station-name">
          <div class="station-name-ellipsis">
            {data.station.name}
          </div>
        </div>
        <div class="station-actions">
          <a
            href="/accounts/{data.station.account_id}/stations/{data.station._id}"
            class="station-action"
            class:current={$page.url.pathname === `/accounts/${data.station.account_id}/stations/${data.station._id}`}
            on:click={event => scroll_into_view(event.currentTarget)}
          >
            <span class="action-name ripple-container">
              Dashboard
            </span>
            {#if $page.url.pathname === `/accounts/${data.station.account_id}/stations/${data.station._id}`}
              <div class="current-action-line" in:current_enter|loal out:current_leave|local />
            {/if}
          </a>

          <a
            href="/accounts/{data.station.account_id}/stations/{data.station._id}/profile"
            class="station-action"
            class:current={$page.url.pathname === `/accounts/${data.station.account_id}/stations/${data.station._id}/profile`}
            on:click={event => scroll_into_view(event.currentTarget)}
          >
            <span class="action-name ripple-container">
              Profile
            </span>
            {#if $page.url.pathname === `/accounts/${data.station.account_id}/stations/${data.station._id}/profile`}
              <div class="current-action-line" in:current_enter|local out:current_leave|local />
            {/if}
          </a>

          <a
            href="/accounts/{data.station.account_id}/stations/{data.station._id}/playlist"
            class="station-action"
            class:current={$page.url.pathname === `/accounts/${data.station.account_id}/stations/${data.station._id}/playlist`}
            on:click={event => scroll_into_view(event.currentTarget)}
          >
            <span class="action-name ripple-container">
              Playlist
            </span>
            {#if $page.url.pathname === `/accounts/${data.station.account_id}/stations/${data.station._id}/playlist`}
              <div class="current-action-line" in:current_enter|local out:current_leave|local />
            {/if}
          </a>
        </div>
      </div>
    </div>
  </div>

  <slot />

{/key}