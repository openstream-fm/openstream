<script lang="ts">
	import { afterNavigate } from "$app/navigation";
  export let data: import("./$types").LayoutData;
  import { page } from "$app/stores";
	import { locale } from "$lib/locale";
	import { click_out } from "$share/actions";
	import { ripple } from "$share/ripple";
	import { logical_fly } from "$share/transition";
	import { crossfade, fade } from "svelte/transition";
  
  $: current_page = $page.data.current_page;
  $: account_stations = data.stations.items.filter(item => item.account_id === data.account._id);

  const selector_item_url = ({ _id, account_id }: { _id: string, account_id: string }) => {
    if(current_page == null || current_page === "dashboard") {
      return `/accounts/${account_id}/stations/${_id}`
    } else {
      return `/accounts/${account_id}/stations/${_id}/${current_page}`
    }
  }

  const scroll_into_view = (node: HTMLElement) => {
    // @ts-ignore
    if(node.scrollIntoViewIfNeeded) {
      // @ts-ignore
      node.scrollIntoViewIfNeeded(true);
    }
  }

  const [_enter, _leave] = crossfade({ duration: 300, fallback: (node) => fade(node, { duration: 200 }) });

  const current_enter = (node: HTMLElement, _params = {}) => {
    return _enter(node, { key: null })
  }

  const current_leave = (node: HTMLElement, _params = {}) => {
    return _leave(node, { key: null })
  }

  let selector_open = false;
  const selector_menu_click_out = (event: MouseEvent) => {
    setTimeout(() => {
      selector_open = false;
    }, 5)
  }

  const close_selector = () => {
    selector_open = false;
  }

  const open_selector = () => {
    selector_open = true;
  }

  const toggle_selector = () => {
    selector_open = !selector_open;
  }

  afterNavigate(() => {
    close_selector();
  })

  let scroll_y = 0;
</script>

<style>

  .station-out {
    padding: 1rem 1rem 0 1rem ;
    display: flex;
    align-items: stretch;
    justify-content: stretch;
    flex-direction: column;
    position: relative;
    z-index: var(--z-station-top);
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
    padding: 0.5rem 1.5rem 0.5rem 0.5rem;
  }

  .station-btn-out {
    position: relative;
    display: flex;  
  }

  .station-selector-menu {
    position: fixed;
    transform: translateY(calc(var(--scroll-y) * -1));
    padding: 0.5rem;
    margin: 0 0 0 -0.5rem;
    border-radius: 0.25rem;
    z-index: var(--z-station-menu);
    min-width: min(80%, 20rem);
  }

  .station-btn, .station-selector-item {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-start;
    transition: background-color 200ms ease;
  }

  .station-selector-item {
    padding: 0.5rem;
    border-radius: 0.25rem;
  }

  .station-btn {
    padding: 0.5rem;
    border-radius: 0.5rem;
  }

  .station-btn:hover, .station-selector-item:hover, .station-btn.station-selector-open {
    background: rgba(0,0,0,0.025);
  }

  .station-selector-item.current {
    background: rgba(var(--blue-rgb), 0.1);
  }

  .station-selector-anchor {
    position: absolute;
    inset-block-end: 0;
    inset-inline-start: 0;
    width: 0;
    height: 0;
    overflow: visible;
  }

  .station-selector-menu {
    background: #fff;
    box-shadow: 0 5px 25px 0 rgb(0 0 0 / 10%);
  }

  .station-selector-item {
    display: flex;
    flex-direction: row;
    align-items: center;
  }

  .station-pic, .station-selector-pic {
    width: 2rem;
    height: 2rem;
    flex: none;
    border-radius: 0.25rem;
    background-position: center;
    background-size: contain;
    background-repeat: no-repeat;
  }

  .station-name, .station-action {
    font-size: 1rem;
  }

  .station-name, .station-selector-name {
    max-width: 10rem;
    display: flex;
    flex-shrink: 1;
    margin-inline-start: 0.75rem;
  }

  .station-selector-name {
    min-width: 4rem;
  }

  .station-name-ellipsis, .station-selector-ellipsis {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    flex: 1;
    min-width: 0;
  }

  .station-actions {
    display: flex;
    flex-direction: row;
    align-items: center;
    flex: none;
    padding: 0.5rem 0;
    margin-inline-start: 0.25rem;
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

  .current-action-line {
    position: absolute;
    inset: 0;
    /* border-bottom: 1px solid var(--blue); */
    border-radius: 0.25rem;
    pointer-events: none;
    background: rgba(var(--blue-rgb), 0.1);
  }

  @media screen and (max-width: 500px) {
    .station-name {
      display: none;
    }
  }
</style>

<svelte:window bind:scrollY={scroll_y} />

<div class="station-out" in:fade={{ duration: 200 }}>
  <div class="station-scroll super-thin-scroll" on:scroll={close_selector}>
    <div class="station">
      <div class="station-btn-out">
        <button class="station-btn" class:station-selector-open={selector_open} on:click={toggle_selector}>
          <div class="station-pic" style="background-image: url({ data.config.storage_public_url }/station-pictures/webp/64/{data.station.picture_id}.webp)" />
          <div class="station-name">
            <div class="station-name-ellipsis">
              {data.station.name}
            </div>
          </div>
        </button>
        <div class="station-selector-anchor">
          {#if selector_open}
            <div
              class="station-selector-menu"
              style:--scroll-y="{scroll_y}px"
              use:click_out={selector_menu_click_out}
              transition:logical_fly|local={{ duration: 200, y: -10 }}
            >
              {#each account_stations as station (station._id)}
                <a
                  href={selector_item_url(station)}
                  class="na station-selector-item ripple-container"
                  class:current={station._id === data.station._id}
                  on:click={close_selector}
                  use:ripple
                >
                  <div class="station-selector-pic" style="background-image: url({ data.config.storage_public_url }/station-pictures/webp/64/{station.picture_id}.webp)" />
                  <div class="station-selector-name">
                    <div class="station-selector-ellipsis">
                      {station.name}
                    </div>
                  </div>
                </a>
              {/each}
            </div>
          {/if}
        </div>
      </div>
      <div class="station-actions">
        <a
          href="/accounts/{data.station.account_id}/stations/{data.station._id}"
          class="station-action"
          class:current={current_page === "dashboard"}
          on:click={event => scroll_into_view(event.currentTarget)}
        >
          <span class="action-name ripple-container">
            {$locale.station_nav.dashboard}
          </span>
          {#if current_page === "dashboard"}
            <div class="current-action-line" in:current_enter|loal out:current_leave|local />
          {/if}
        </a>

        <a
          href="/accounts/{data.station.account_id}/stations/{data.station._id}/profile"
          class="station-action"
          class:current={current_page === "profile"}
          on:click={event => scroll_into_view(event.currentTarget)}
        >
          <span class="action-name ripple-container">
            {$locale.station_nav.profile}
          </span>
          {#if current_page === "profile"}
            <div class="current-action-line" in:current_enter|local out:current_leave|local />
          {/if}
        </a>

        <a
          href="/accounts/{data.station.account_id}/stations/{data.station._id}/playlist"
          class="station-action"
          class:current={current_page === "playlist"}
          on:click={event => scroll_into_view(event.currentTarget)}
        >
          <span class="action-name ripple-container">
            {$locale.station_nav.playlist}
          </span>
          {#if current_page === "playlist"}
            <div class="current-action-line" in:current_enter|local out:current_leave|local />
          {/if}
        </a>

        <a
          href="/accounts/{data.station.account_id}/stations/{data.station._id}/broadcast"
          class="station-action"
          class:current={current_page === "broadcast"}
          on:click={event => scroll_into_view(event.currentTarget)}
        >
          <span class="action-name ripple-container">
            {$locale.station_nav.broadcast}
          </span>
          {#if current_page === "broadcast"}
            <div class="current-action-line" in:current_enter|local out:current_leave|local />
          {/if}
        </a>

        <a
          href="/accounts/{data.station.account_id}/stations/{data.station._id}/settings"
          class="station-action"
          class:current={current_page === "settings"}
          on:click={event => scroll_into_view(event.currentTarget)}
        >
          <span class="action-name ripple-container">
            {$locale.station_nav.settings}
          </span>
          {#if current_page === "settings"}
            <div class="current-action-line" in:current_enter|local out:current_leave|local />
          {/if}
        </a>
      </div>
    </div>
  </div>
</div>

{#key data.station._id}
  <slot />
{/key}