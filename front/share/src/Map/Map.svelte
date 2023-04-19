<script lang="ts">
  import dataset from "./countries.lite.geo.mjs";
  import { geoPath, geoMercator } from "d3";
  import { fade } from "svelte/transition";
  import { default_logger } from "$lib/logger";

  const logger = default_logger.scoped("map");

  const mouseenter = (item: typeof dataset.features[number]) => {
    logger.info(`hover start: `, item.properties.iso2, item.properties.name, item);
    dataset.features = [...dataset.features.filter(each => item !== each), item];
  }

  let w = 0;
  let h = 0;

  const tsignore = (src: any): any => src; 

  $: projection = geoMercator().center([0, -40]).fitExtent([[0, 0], [ 1000, 660 ]], dataset as any)
  $: path = geoPath(projection);
</script>

<style>
  .viewport {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    overflow: hidden;
  }

  svg {
    width: 100%;
    height: 100%;
  }
  
  path {
    fill: #f3f3f3;
    stroke: #aaa;
    stroke-width: 1.25;
    transition: filter 150ms ease;
  }

  path:hover {
    filter: drop-shadow(rgba(0,0,0,0.4) 0 0 2px);
  }
</style>

<div class="viewport" bind:clientWidth={w} bind:clientHeight={h} in:fade|local={{ duration: 350 }}>
  <svg viewBox="0 0 1000 660">
    {#each dataset.features as item (item.properties.iso2)}
      <!-- svelte-ignore a11y-mouse-events-have-key-events -->
      <!-- @ts-ignore -->
      <path
        d={path(tsignore(item))}
        on:mouseover={() => mouseenter(item)}
        />
    {/each}
  </svg>
</div>