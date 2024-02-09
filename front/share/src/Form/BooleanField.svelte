<script lang="ts">
  export let value: boolean;
  export let label: string;

	import Icon from "$share/Icon.svelte";
  import { ripple } from "$share/ripple";
	import { mdiCheckBold, mdiCheckboxBlankOutline } from "@mdi/js";
	import { scale } from "svelte/transition";
</script>

<style>
  input {
    display: none;
  }

  .boolean-field {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
  }

  .button {
    display: flex;
    flex-direction: row;
    align-items: center;
    user-select: none;
    cursor: pointer;
    padding: 0.75rem;
    margin: 0 -0.5rem;
    transition: background-color 200ms ease;
  }

  .button:hover {
    background-color: rgba(0,0,0,0.05);
  }

  .icon, .icon-out {
    width: 1.5rem;
    height: 1.5rem;
  }

  .icon-out {
    margin-inline-end: 1rem;
    flex: none;
    position: relative;
  }

  .icon {
    position: absolute;
    inset-block-start: 0;
    inset-inline-start: 0;
    font-size: 1.5rem;
    display: flex;
    transition: color 200ms ease;
    color: #666;
  }

  .checked .icon {
    color: var(--green);
  }
</style>

<div class="boolean-field" class:checked={value}>
  <label class="button ripple-container" use:ripple>
    <input type="checkbox" bind:checked={value} />
    <div class="icon-out">
      {#if value}
        <div class="icon" transition:scale={{ duration: 200 }}>
          <Icon d={mdiCheckBold} />
        </div>
      {:else}
        <div class="icon" transition:scale={{ duration: 200 }}>
          <Icon d={mdiCheckboxBlankOutline} />
        </div>
      {/if}
    </div>
    <div class="label">
      {label}
    </div>
  </label>
</div>