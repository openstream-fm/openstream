<script lang="ts">
  export let label: string;
  export let full: boolean;  
</script>

<style>
  .label {
    pointer-events: none;
    position: absolute;
    color: #ccc;
    transition: color 250ms ease, transform 150ms ease;
    margin: var(--spacing);
  }

  :global(.field-container:focus-within) > .label {
    color: var(--blue);
  }

  :global(.field-container:focus-within) > .label, .full {
    transform: translateY(-110%) scale(0.7);
    transform-origin: 0 0;
  }
</style>

<span class="label" class:full>
  {label}
</span>
