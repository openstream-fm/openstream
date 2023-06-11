<script lang="ts">
  import Icon from "$share/Icon.svelte";
import { ripple } from "$share/ripple";
  import { mdiCheck, mdiCheckBold, mdiCheckboxMarkedCircle } from "@mdi/js";
  import { scale } from "svelte/transition";

  export let card: import("$server/defs/PublicPaymentMethod").PublicPaymentMethod;
  export let selected: boolean = false;
  export let on_click: () => void;
</script>

<style>
  .card {
    display: flex;
    flex-direction: row;
    align-items: center;
    text-align: start;
    cursor: pointer;
    border-radius: 0.5rem;
    border: #bbb 2px solid;
    padding: 0.75rem;
    transition: border-color 300ms ease, background-color 300ms ease, color 300ms ease;
  }

  .card.selected {
    border-color: rgba(0,0,0,0.5);
    background: green;
    color: #fff;
  }

  .data {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    flex: 1;
  }

  .brand {
    font-size: 1.05rem;
    font-weight: 600;
    margin-bottom: 0.25rem;
  }

  .expiration {
    margin-top: 0.125rem;
  }

  .selected-icon-out {
    width: 3rem;
    height: 3rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .selected-icon {
    display: flex;
    font-size: 1.75rem;
  }
</style>

<button
  class="card ripple-container"
  class:selected
  aria-selected={selected}
  use:ripple
  on:click|preventDefault={on_click}
>
  <div class="data">
    <div class="brand">{card.card_type}</div>
    <div class="number">Ending in <b>{card.last_4}</b></div>
    {#if card.expiration_month && card.expiration_year}
      <div class="expiration">
        Expires at
        <b>
          <span class="month">{card.expiration_month}</span>/<span class="year">{card.expiration_year}</span>
        </b>
      </div>  
    {/if}
  </div>
  <div class="selected-icon-out">
    {#if selected}
      <div class="selected-icon" transition:scale|local={{ duration: 300, delay: 100 }}>
        <Icon d={mdiCheckBold} />
      </div>
    {/if}
  </div>
</button>