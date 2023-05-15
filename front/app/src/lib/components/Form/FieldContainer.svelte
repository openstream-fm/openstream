<script lang="ts">
  export let disabled: boolean = false;
  export let readonly: boolean = false;
  export let icon: string | null = null;
  export let btn: { icon: string, action: () => void, tabindex?: number | undefined } | null = null;
	import Icon from "$share/Icon.svelte";
	import { ripple } from "$share/ripple";

  // let btn_click_token = false;

  // const btn_pointerdown = () => {
  //   btn_click_token = true;
  //   btn?.action();
  // }

  // // handle enter key
  // const btn_click = () => {
  //   let token = btn_click_token;
  //   btn_click_token = false;
  //   if (!token) {
  //     btn?.action();
  //   }
  // }
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
    background: var(--field-container-bg, #fff);
  }

  .field-container:not(.disabled):not(.readonly):focus-within {
    border-bottom-color: var(--blue); 
  }

  .disabled {
    background: var(--field-container-disabled-bg, #f3f3f3);
  }

  .readonly {
    background: var(--field-container-readonly-bg, #f3f3f3);
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

  .field-container:not(.disabled):not(.readonly):focus-within > .icon {
    color: var(--blue);
  }

  .field {
    flex: 1;
    display: flex;
  }

  .btn-out {
    display: flex;
    flex-direction: row;
    align-items: stretch;
    flex: none;
  }

  .btn {
    user-select: none;
    cursor: pointer;
    flex: none;
    appearance: none;
    background: transparent;
    padding: 0;
    border: 0;
    margin: 0;
    width: 2rem;
    height: 2rem;
    font-size: 1.25em;
    border-radius: 0.25em;
    align-self: stretch;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #bbb;
    transition: background-color 200ms ease, color 200ms ease;
  }

  .btn:hover {
    color: #888;
  }
</style>

<label class="field-container" class:disabled class:readonly class:with-icon={icon != null}>
  {#if icon != null}
    <div class="icon">
      <Icon d={icon} />
    </div>
  {/if}
  <div class="field">
    <slot />
  </div>
  {#if btn != null}
    <div class="btn-out">
      <!-- <button class="btn" on:pointerdown|capture|preventDefault={btn_pointerdown} on:click|preventDefault={btn_click}>
        <Icon d={btn.icon} />
      </button> -->
      <button class="btn ripple-container" use:ripple on:click|preventDefault={() => btn?.action()} tabindex={btn.tabindex}>
        <Icon d={btn.icon} />
      </button>
    </div>
  {/if}
</label>