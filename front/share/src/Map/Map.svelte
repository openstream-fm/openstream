<script lang="ts">
  import dataset from "./countries.lite.geo.mjs";
  import { geoPath, geoMercator } from "d3";
  import { fade } from "svelte/transition";
  import { onMount } from "svelte";
  import { add } from "$share/util.js";

  type Dataset = typeof dataset;
  type Item = Dataset["features"][number];
  type Stats = Partial<Record<string, StatsItem>>;  
  type StatsItem = { sessions: number, ips: number };
  
  const sample_stats: Stats = {
    "CA": { sessions: 5, ips: 4 },
    "ES": { sessions: 100, ips: 82 },
    "CO": { sessions: 500, ips: 436 },
    "AR": { sessions: 2550, ips: 2400 },
    "BR": { sessions: 5950, ips: 5342 },
    "US": { sessions: 6350, ips: 6141 },
    "FR": { sessions: 7800, ips: 7685 },
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

  let pointerX = 0;
  let pointerY = 0;
  let windowWidth = 0;

  $: tooltip_to_left = pointerX > windowWidth / 2;
  let tooltip_item: Item | null = null;
  
  $: tooltip_stats_item = get_tooltip_stats_item(stats, tooltip_item);
  const get_tooltip_stats_item = (...args: any[]): StatsItem | null => {
    if(tooltip_item == null) return null;
    return stats[tooltip_item.properties.iso2] || null;
  }

  const get_fill = (stats: Stats, item: Item) => {
    const max = Math.max(0, ...Object.values(stats).map(item => item?.sessions || 0))
    if(max === 0) return "var(--fill-none)";
    const sessions = stats[item.properties.iso2]?.sessions || 0;
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

<svelte:window bind:innerWidth={windowWidth} />

<div class="viewport">
  <svg viewBox="0 0 1000 660">
    {#each dataset.features as item (item.properties.iso2)}
      <path
        style:--fill={get_fill(stats, item)}
        d={path(as_any(item))}
        on:pointerenter={() => pointerenter(item)}
        on:mouseleave={() => pointerleave(item)}
      />
    {/each}
  </svg>
</div>

{#if tooltip_item != null}
  {@const sessions = tooltip_stats_item?.sessions || 0}
  {@const ips = tooltip_stats_item?.ips || 0}
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
      {sessions} sessions
    </div>
    <div class="map-tooltip-count">
      {ips} unique IPs
    </div>
  </div>
{/if}