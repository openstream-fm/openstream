<script lang="ts">
  export let card: import("$server/defs/PublicPaymentMethod").PublicPaymentMethod;
  export let selected: boolean = false;
  export let on_click: () => void;
  export let locale: import("$server/locale/share/payments/payments.locale").PaymentsLocale;

  import Icon from "$share/Icon.svelte";
  import { ripple } from "$share/ripple";
  import { mdiCheckBold } from "@mdi/js";
  import { scale } from "svelte/transition";

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
    font-weight: var(--font-bold);
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
    <div class="number">
      {@html
        locale.Ending_in_XXXX_html
          .replace("@XXXX", card.last_4)
      }
    </div>
    {#if card.expiration_month && card.expiration_year}
      <div class="expiration">
        {@html
          locale.Expires_at_MM_YYYY_html
            .replace("@YYYY", card.expiration_year)
            .replace("@MM", card.expiration_month)
        }
      </div>  
    {/if}
  </div>
  <div class="selected-icon-out">
    {#if selected}
      <div class="selected-icon" transition:scale={{ duration: 300, delay: 100 }}>
        <Icon d={mdiCheckBold} />
      </div>
    {/if}
  </div>
</button>