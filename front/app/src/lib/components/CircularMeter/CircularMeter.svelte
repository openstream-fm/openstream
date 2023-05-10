<script lang="ts">
  export let used: number; // 0 to 1
  export let width: number | null = null;
  export let height: number | null = null;
  export let strokeWidth = "var(--stroke-width, 5)";
  export let stroke = "var(--red)";
  export let fill = "transparent";

  const polarToCartesian = (cX: number, cY: number, radius: number, degrees: number) => {
      const radians = (degrees - 180) * Math.PI / 180;
      return {
          x: cX + (radius * Math.cos(radians)),
          y: cY + (radius * Math.sin(radians))
      }
  }

  const describeArc = (x: number, y: number, radius: number, startA: number, endA: number) => {
      const start = polarToCartesian(x, y, radius, endA)
      const end = polarToCartesian(x, y, radius, startA)

      const largeArcFlag = endA - startA <= 180 ? "0" : "1";

      return `
          M ${start.x}, ${start.y}
          A ${radius} ${radius} 0 ${largeArcFlag} 0 ${end.x} ${end.y}
      `;
  }
  
  const clamp = (min: number, v: number, max: number): number => Math.max(min, Math.min(v, max));

  $: d = describeArc(50, 50, 35, 90, clamp(0.001, used, 0.999) * 360 + 90);
</script>

<svg {height} {width} viewBox="0 0 100 100" preserveAspectRatio="xMidYMid meet">
  <circle 
    style="
      stroke: rgba(0,0,0,0.075);
      stroke-width: {strokeWidth};
      fill: transparent"
    cx={50}
    cy={50}
    r={35}  
  />
  {#if used !== 0}
    <path {d} style="stroke: {stroke}; fill: {fill}; stroke-width: {strokeWidth};"></path>
  {/if}
</svg>