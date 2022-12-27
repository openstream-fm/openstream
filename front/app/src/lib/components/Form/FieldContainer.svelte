<script>
	import TextField from "./TextField.svelte";
</script>

<style>
  .field-container {
    display: flex;
    border-bottom: 1px #bbb solid;    
    transition: border-bottom-color 250ms ease;
    --spacing-y: 0.75rem;
    --spacing-x: 0.5rem;
    --spacing: var(--spacing-y) var(--spacing-x);
  }

  .field-container:focus-within {
    border-bottom-color: var(--blue); 
  }
</style>

<label class="field-container">
  <slot />
</label>