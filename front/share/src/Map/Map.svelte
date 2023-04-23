<script lang="ts">
  import dataset from "./countries.lite.geo.mjs";
  import { geoPath, geoMercator } from "d3";
  import { fade } from "svelte/transition";
  import { onMount } from "svelte";
  import { add } from "$share/util.js";

  type Dataset = typeof dataset;
  type Item = Dataset["features"][number];
  type Stats = import("$server/defs/stream-connection-stats/StatsItem").StatsItem;  
  
  const sample_stats: Stats = {
    sessions: 16736,
    // ips: 15850,
    country_sessions: {
      "RU": 5,
      "ES": 100,
      "CO": 500,
      "AR": 2550,
      "BR": 5950,
      "US": 6350,
      "FR": 7800,
    },
    // country_ips: {
    //   "RU": 4,
    //   "ES": 82,
    //   "CO": 436,
    //   "AR": 2400,
    //   "BR": 5342,
    //   "US": 6141,
    //   "FR": 7685,
    // }
  };

  export let stats: Stats = sample_stats;

  const pointerenter = (item: typeof dataset.features[number]) => {
    //logger.info(`hover start: `, item.properties.iso2, item.properties.name, item);
    dataset.features = [...dataset.features.filter(each => item !== each), item];
    tooltip_item = item;
  }

  const pointerleave = (item: typeof dataset.features[number]) => {
    //logger.info(`hover end: `, item.properties.iso2, item.properties.name, item);
    if(tooltip_item === item) tooltip_item = null;
  }

  const pointerout = () => {
    tooltip_item = null;
  }

  let pointerX = 0;
  let pointerY = 0;
  let windowWidth = 0;

  $: tooltip_to_left = pointerX > windowWidth / 2;
  let tooltip_item: Item | null = null;
  
  $: tooltip_sessions = get_tooltip_sessions(stats, tooltip_item);
  const get_tooltip_sessions = (...args: any[]): number => {
    if(tooltip_item == null) return 0;
    return stats.country_sessions[tooltip_item.properties.iso2] || 0;
  }

  // $: tooltip_ips = get_tooltip_ips(stats, tooltip_item);
  // const get_tooltip_ips = (...args: any[]): number => {
  //   if(tooltip_item == null) return 0;
  //   return stats.country_ips[tooltip_item.properties.iso2] || 0;
  // }

  const get_fill = (stats: Stats, item: Item) => {
    const max = Math.max(0, ...Object.values(stats.country_sessions).map(Number));
    if(max === 0) return "var(--fill-none)";
    const sessions = stats.country_sessions[item.properties.iso2] || 0;
    if(sessions === 0) return "var(--fill-none)";
    const opacity =  0.15 + (sessions / max) * 0.85;
    return `rgba(var(--blue-rgb), ${opacity})`
  }

  onMount(() => {
    const set = (event: Event) => {
      const e = event as PointerEvent;
      pointerX = e.x;
      pointerY = e.y;
    };

    const off = [
      add(window, "pointermove", set, { capture: true }),
      add(window, "pointerdown", set, { capture: true }),
      add(window, "pointerup", set, { capture: true }),
    ]

    return () => {
      for(const fn of off) fn();
    }
  })

  const as_any = (src: any): any => src; 
  const projection = geoMercator().center([0, -40]).fitExtent([[0, 0], [ 1000, 660 ]], dataset as any)
  const path = geoPath(projection);
</script>

<style>
  .viewport {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
  }

  svg {
    width: 100%;
    height: 100%;
  }
  
  path {
    --fill-none: #f3f3f3;
    fill: var(--fill);
    stroke: #aaa;
    stroke-width: 1.25;
    transition: filter 150ms ease;
  }

  path:hover {
    filter: drop-shadow(rgba(0,0,0,0.4) 0 0 2px);
  }

  .map-tooltip {
    white-space: nowrap;
    padding: 0.5rem 0.75rem;
    background: #fff;
    box-shadow: rgba(0,0,0,0.2) 0 1px 3px 2px;
    color: #000;
    border-radius: 0.25rem;
    position: fixed;
    top: var(--pointer-y);  
    left: var(--pointer-x);
    z-index: var(--z-map-tooltip);
    pointer-events: none;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    text-align: left;
    gap: 0.1rem;
  }

  .map-tooltip.to-left {
    transform: translateX(-100%);
  }

  .map-tooltip-name {
    font-size: 0.9rem;
  }

  .map-tooltip-count {
    font-size: 0.8rem;
  }
</style>

<svelte:window bind:innerWidth={windowWidth} on:pointerdown={pointerout} />

<div class="viewport">
  <svg viewBox="0 0 1000 660">
    {#each dataset.features as item (item.properties.iso2)}
      <path
        style:--fill={get_fill(stats, item)}
        d={path(as_any(item))}
        on:pointerenter|stopPropagation={() => pointerenter(item)}
        on:pointerdown|stopPropagation={() => {}}
        on:mouseleave={() => pointerleave(item)}
      />
    {/each}
  </svg>
</div>

{#if tooltip_item != null}
  {@const name = tooltip_item.properties.name}
  <div
    class="map-tooltip"
    class:to-left={tooltip_to_left}
    in:fade|local={{ duration: 200 }}
    style:--pointer-x="{pointerX}px"
    style:--pointer-y="{pointerY}px"
  >
    <div class="map-tooltip-name">
      {name}
    </div>
    <div class="map-tooltip-count">
      {tooltip_sessions} sessions
    </div>
    <!-- <div class="map-tooltip-count">
      {tooltip_ips} unique IPs
    </div> -->
  </div>
{/if}