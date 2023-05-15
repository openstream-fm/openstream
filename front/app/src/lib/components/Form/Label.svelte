<script lang="ts">
  export let label: string;
  export let full: boolean;  
</script>

<style>
  .label {
    pointer-events: none;
    position: absolute;
    color: #aaa;
    transition: color 250ms ease, transform 150ms ease;
    margin: var(--spacing);
  }

  :global(.field-container:not(.disabled):not(.readonly):focus-within) .label {
    color: var(--blue);
  }

  :global(.field-container:not(.with-icon):focus-within) .label,
  :global(.field-container:not(.with-icon)) .full {
    transform: translateY(-110%) scale(0.7);
    transform-origin: 0 0;
  }

  :global(.field-container.with-icon:focus-within) .label,
  :global(.field-container.with-icon) .full {
    transform: translateX(-1.75rem) translateY(-110%) scale(0.7);
    transform-origin: 0 0;
  }
</style>

<span class="label" class:full>
  {label}
</span>
