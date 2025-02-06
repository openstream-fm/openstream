<script lang="ts">
  export let d: string;
  export let stroke: boolean = false;
  export let strokeWidth: number | "" = "";
  export let viewBox: string = "0 0 24 24";
</script>

<style>
  svg {
    width: var(--icon-size, 1em);
    height: var(--icon-size, 1em);
  }

  path {
    fill: var(--icon-color, currentColor);
  }

  path.stroke {
    fill: transparent;
    stroke: var(--icon-color, currentColor);
    stroke-width: var(--stroke-width);
  }
</style>

<svg {viewBox}>
  {#if stroke}
    <path d={d} class="stroke" style="--stroke-width:{strokeWidth}" />
  {:else}
    <path d={d} />
  {/if}
</svg>