

<script lang="ts">
  export let start: number; // 0 to 1
  export let end: number; // 0 to 1
  export let strokeWidth = "var(--stroke-width, 5)";
  export let stroke = "transparent";
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

  $: d = describeArc(50, 50, 35, start * 360 + 90, end * 360 + 90);
</script>

<path {d} style="stroke: {stroke}; fill: {fill}; stroke-width: {strokeWidth}; stroke-linecap: round;"></path>