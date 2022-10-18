<script lang="ts">
  export let d: string;
</script>

<style>
  .icon {
    width: 1em;
    height: 1em;
    display: flex;
  }

  path {
    fill: var(--icon-color, currentColor);
  }
</style>

<svg class="icon" viewBox="0 0 24 24">
  <path {d} />
</svg>