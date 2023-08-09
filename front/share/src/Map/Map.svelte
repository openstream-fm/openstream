<script lang="ts">
  export let stats: Stats;
  export let country_names: Record<string, string | undefined>;
  export let locale: import("$server/locale/share/stats-map/stats-map.locale").StatsMapLocale;
  
  type Stats = {
    sessions: number,
    country_sessions: Record<string, number | undefined>,
    ips?: number,
    country_ips?: Record<string, number | undefined>,
    country_avg_listening_ms?: Record<string, number | undefined>,
  } 

  type Dataset = typeof dataset;
  type Item = Dataset["features"][number];

  // import("$server/defs/stream-connection-stats/StatsItem").StatsItem & { country_ips: Stats["country_sessions"] };  
  
  import dataset from "./countries.lite.geo";
  import { geoPath, geoMercator } from "d3";
  import { fade } from "svelte/transition";
  import { onMount } from "svelte";
  import { add } from "$share/util";
  import { click_out } from "$share/actions";
  import Icon from "$share/Icon.svelte";
  import { mdiDownload, mdiMenu } from "@mdi/js";
  import { logical_fly } from "$share/transition";
  import { ripple } from "$share/ripple";
 
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

  $: tooltip_avg_listening_ms = get_tooltip_avg_listening_ms(stats, tooltip_item);
  const get_tooltip_avg_listening_ms = (...args: any[]): number | null => {
    if(tooltip_item == null) return 0;
    return stats.country_avg_listening_ms?.[tooltip_item.properties.iso2] || null;
  }

  const SEC = 1000;
  const MIN = SEC * 60;
  const pad = (n: number) => String(n).padStart(2, "0");
  const format_avg_listening_ms = (ms: number) => {
    const total_secs = Math.round(ms / SEC);
    const mins = Math.floor(total_secs / 60);
    const secs = total_secs % 60;
    return `${pad(mins)}:${pad(secs)}`;
  }

  // $: tooltip_ips = get_tooltip_ips(stats, tooltip_item);
  // const get_tooltip_ips = (...args: any[]): number => {
  //   if(!show_ips) return 0;
  //   if(tooltip_item == null) return 0;
  //   return stats.country_ips?.[tooltip_item.properties.iso2] || 0;
  // }

  const get_fill = (stats: Stats, item: Item) => {
    const max = Math.max(0, ...Object.values(stats.country_sessions).map(Number));
    if(max === 0) return "var(--fill-none)";
    const sessions = stats.country_sessions[item.properties.iso2] || 0;
    if(sessions === 0) return "var(--fill-none)";
    const opacity =  Math.log(Math.max(2, sessions)) / Math.log(Math.max(2, max));
    return `rgba(var(--fill-full-rgb),${opacity})`
  }

  const fill_none = "#f3f3f3";

  const blue_rgb = { r: 0, g: 116, b: 217 };
  //const blue_rgb = { r: 0, g: 56, b: 118 };

  const get_fill_for_export = (stats: Stats, item: Item) => {
    const max = Math.max(0, ...Object.values(stats.country_sessions).map(Number));
    if(max === 0) return fill_none;
    const sessions = stats.country_sessions[item.properties.iso2] || 0;
    if(sessions === 0) return fill_none;
    const opacity =  Math.log(Math.max(2, sessions)) / Math.log(Math.max(2, max));
    return `rgba(${blue_rgb.r},${blue_rgb.g},${blue_rgb.b},${opacity})`;
  }

  const tooltip_mount = (node: HTMLElement) => {
    document.documentElement.appendChild(node);
  }

  onMount(() => {
    const set = (event: Event) => {
      const e = event as PointerEvent;
      pointerX = e.pageX;
      pointerY = e.pageY;
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

  export const get_svg_source = (width = 1000) => {
    const height = width * 0.66;
    let source = `<svg xmlns="http://www.w3.org/2000/svg" width="${width}" height="${height}" viewBox="0 0 1000 660">`;
    source += `\n  <rect x="0" y="0" width="${width}" height="${height}" fill="#ffffff" />`
    for(const item of dataset.features) {
      source += `\n  <path strokeWidth="1.25" stroke="#aaaaaa" fill="${get_fill_for_export(stats, item)}" d="${path(item as any)}" />`;
    }
    source += "\n</svg>"
    return source;
  }

  const export_svg = () => {
    const source = get_svg_source();
    const url = `data:image/svg+xml;charset=utf-8,${encodeURIComponent(source)}`;
    const a = document.createElement("a");
    a.download = "map.svg";
    a.href = url;
    a.click();
  }

  const export_png = () => {
    const canvas = document.createElement('canvas');
    canvas.width = 1000;
    canvas.height = 660;
    const ctx = canvas.getContext('2d')!;
    
    const img = new Image();
    var svg_blob = new Blob([get_svg_source()], { type: "image/svg+xml;charset=utf-8" });
    var url = URL.createObjectURL(svg_blob);

    img.onload = function () {
      ctx.fillStyle = "#ffffff";
      ctx.fillRect(0, 0, canvas.width, canvas.height);
      ctx.drawImage(img, 0, 0);
      URL.revokeObjectURL(url);

      canvas.toBlob(blob => {
        if(!blob) return; 
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.download = "map.png";
        a.href = url
        a.click();
      }, "image/png", 1);
    };

    img.src = url;
  };

  let menu_open = false;

  const menu_click_out = () => {
    setTimeout(() => {
      menu_open = false;
    }, 2)
  }
</script>

<style>
  .map {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    width: 100%;
    position: relative;
  }

  .viewport {
    width: 100%;
    max-width: var(--map-max-width, none);
    padding: 2rem;
    aspect-ratio: 100 / 66;    
  }

  svg {
    width: 100%;
    height: 100%;
  }
  
  path {
    --fill-none: #f3f3f3;
    fill: var(--fill);
    stroke: var(--stroke);
    stroke-width: 1.25;
    transition: filter 150ms ease, fill 200ms ease;
  }

  path:hover {
    filter: drop-shadow(rgba(0,0,0,0.4) 0 0 2px);
  }

  .map-tooltip {
    position: absolute;
    white-space: nowrap;
    padding: 0.5rem 0.75rem;
    background: #fff;
    box-shadow: rgba(0,0,0,0.2) 0 1px 3px 2px;
    color: #000;
    border-radius: 0.25rem;
    top: var(--pointer-y);  
    left: var(--pointer-x);
    z-index: var(--z-map-tooltip);
    pointer-events: none;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    text-align: start;
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

  .menu-out {
    position: absolute;
    inset-block-start: 1rem;
    inset-inline-end: 1rem;
  }

  .menu-position {
    position: relative;
  }

  .menu-btn {
    display: flex;
    color: #333;
    padding: 0.75rem;
    font-size: 1.25rem;
    border-radius: 0.25rem;
    transition: background-color 200ms ease;
  }

  .menu-btn:hover, .menu-btn.active {
    background: rgba(0,0,0,0.05);
  }

  .menu {
    position: absolute;
    inset-block-start: 100%;
    inset-inline-end: 0;
    padding: 0.5rem;
    border-radius: 0.25rem;
    background: #fff;
    box-shadow: var(--some-shadow);
  }


  .menu-item {
    display: flex;
    flex-direction: row;
    align-items: center; 
    padding: 0.5rem;
    border-radius: 0.25rem;
    transition: background-color 200ms ease;
    font-size: 0.8rem;
    white-space: nowrap;
  }

  .menu-item:hover {
    background-color: rgba(0,0,0,0.05);
  }

  .menu-item-icon {
    display: flex;
    font-size: 1rem;
    margin-inline-end: 0.5rem;
  }
</style>

<svelte:window bind:innerWidth={windowWidth} on:pointerdown={pointerout} />

<div class="map" style:--fill-full-rgb="{blue_rgb.r}, {blue_rgb.g}, {blue_rgb.b}">
  <div class="viewport">
    <svg viewBox="0 0 1000 660" use:click_out={() => tooltip_item = null}>
      {#each dataset.features as item (item.properties.iso2)}
        <path
          style:--fill={get_fill(stats, item)}
          style:--stroke={"#aaa"}
          d={path(as_any(item))}
          on:pointerenter|stopPropagation={() => pointerenter(item)}
          on:pointerdown|stopPropagation={() => {}}
          on:mouseleave={() => pointerleave(item)}
        />
      {/each}
    </svg>
  </div>

  <div class="menu-out">
    <div class="menu-position">
      <button class="menu-btn ripple-container" class:active={menu_open} use:ripple on:click={() => menu_open = !menu_open}>
        <Icon d={mdiMenu} />
      </button>
      {#if menu_open}
        <div class="menu" transition:logical_fly|local={{ y: -15, x: 15, duration: 200 }} use:click_out={menu_click_out}>
          <button class="menu-item menu-item-svg ripple-container" use:ripple on:click={export_svg}>
            <div class="menu-item-icon">
              <Icon d={mdiDownload} />
            </div>
            {locale.download_as_svg}
          </button>
          <button class="menu-item menu-item-svg ripple-container" use:ripple on:click={export_png}>
            <div class="menu-item-icon">
              <Icon d={mdiDownload} />
            </div>
            {locale.download_as_png}
          </button>
        </div>
      {/if}
    </div>
  </div>

  {#if tooltip_item != null}
    {@const name = country_names[tooltip_item.properties.iso2] || tooltip_item.properties.name}
    <div
      class="map-tooltip"
      class:to-left={tooltip_to_left}
      in:fade|local={{ duration: 200 }}
      style:--pointer-x="{pointerX}px"
      style:--pointer-y="{pointerY}px"
      use:tooltip_mount
    >
      <slot name="tooltip" country_code={tooltip_item.properties.iso2} country_name={name}>
        <div class="map-tooltip-name">
          {name}
        </div>
        
        <div class="map-tooltip-count">
          {tooltip_sessions} {tooltip_sessions === 1 ? locale.listener : locale.listeners}
        </div>
      </slot>
    </div>
  {/if}
</div>