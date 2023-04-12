<script lang="ts">
  export let disabled: boolean = false;
  export let icon: string | null = null;
	import Icon from "$share/Icon.svelte";
	import TextField from "./TextField.svelte";
</script>

<style>
  .field-container {
    display: flex;
    align-items: center;
    border-bottom: 1px #bbb solid;    
    transition: border-bottom-color 250ms ease;
    --spacing-y: 0.75rem;
    --spacing-x: 0.5rem;
    --spacing: var(--spacing-y) var(--spacing-x);
  }

  .field-container:not(.disabled):focus-within {
    border-bottom-color: var(--blue); 
  }

  .disabled {
    background: #f3f3f3;
  }

  .icon {
    flex: none;
    color: #aaa;
    font-size: 1.25rem;
    margin-inline-start: 0.5rem;
    margin-block-start: 0.45rem;
    margin-inline-end: 0.15;
    transition: color 250ms ease;
  }

  .field-container:focus-within > .icon {
    color: var(--blue);
  }

    .field {
    flex: 1;
    display: flex;
  }

  .field-container:has(:global(*::-webkit-autofill)) {
    background: #000;
  }
</style>

<label class="field-container" class:disabled class:with-icon={icon != null}>
  {#if icon != null}
    <div class="icon">
      <Icon d={icon} />
    </div>
  {/if}
  <div class="field">
    <slot />
  </div>
</label>