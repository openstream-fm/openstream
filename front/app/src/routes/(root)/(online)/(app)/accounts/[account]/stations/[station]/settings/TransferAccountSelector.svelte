<script lang="ts">
	import Icon from "$share/Icon.svelte";

  type Item = typeof data.accounts.items[number];

  export let data: import("./$types").PageData;
  export let selected: Item | null = null;
  export let open = false;

  import { click_out } from "$share/actions";
	import Validator from "$share/formy/Validator.svelte";
	import { _string } from "$share/formy/validate";
	import { ripple } from "$share/ripple";
	import { mdiChevronDown } from "@mdi/js";
	import { fly } from "svelte/transition";

  let show_accounts = data.accounts.items.filter(account => account._id !== data.station.account_id).sort((a, b) => a.name.localeCompare(b.name));

  const toggle = () => {
    open = !open
  }

  const menu_click_out = () => {
    setTimeout(() => open = false, 3);
  }

  const item_click = (item: Item) => {
    selected = item;
    open = false;
  }

</script>

<style>
  .account-selector {
    display: flex;
    flex-direction: column;
    position: relative;
  }

  .switch {
    display: flex;
  }

  .btn {
    padding: 0.75rem 0.5em;
    flex: 1;
    text-align: start;
    display: flex;
    flex-direction: row;
    align-items: center;
    border-bottom: #bbb 1px solid; 
    transition: border-color 200ms ease;
    outline: 0;
  }

  .switch:focus-within > .btn {
    border-bottom-color: var(--blue); 
  }

  .btn-name {
    flex: 1;
    transition: color 200ms ease;
  }

  .btn.empty > .btn-name {
    color: rgba(0,0,0,0.35);
  }

  .btn-chevron {
    flex: none;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.25rem;
    margin-inline-start: 0.75rem;
  }

  .menu {
    position: absolute;
    top: 100%;
    left: 0;
    width: 100%;
    display: flex;
    flex-direction: column;
    padding: 0.5rem 0;
    border-radius: 0.25rem;
    box-shadow: var(--some-shadow);
    background: #fff;
    z-index: 1;
  }


  .menu-item {
    display: flex;
    align-self: stretch;
    overflow: hidden;
    text-overflow: ellipsis;
    padding: 0.75rem;
    border-radius: 0.25rem;
    transition: background-color 200ms ease;
  }

  .menu-item:hover {
    background-color: rgba(0,0,0,0.05);
  }

  .menu-item.selected {
    background-color: rgba(var(--blue-rgb), 0.1);
  }
</style>

<div class="account-selector">
  <div class="switch">
    <button class="btn" class:empty={!selected} on:click|preventDefault={toggle}>
      <div class="btn-name">
        {#if selected}
          {selected.name}
        {:else}
          <!-- TODO: locale -->
          Select a target account
        {/if}
      </div>
      <div class="btn-chevron">
        <Icon d={mdiChevronDown} />
      </div>
    </button>

    {#if open}
      <div class="menu"
        transition:fly|local={{ y: -20, duration: 200 }}
        use:click_out={menu_click_out}
      >
        {#each show_accounts as account (account._id)}
          {@const is_selected = account._id === selected?._id}
          <button
            class="menu-item ripple-container"
            class:selected={is_selected}
            use:ripple
            on:click|preventDefault={() => item_click(account)}
          >
            {account.name}
          </button>
        {/each}
      </div>
    {/if}
  </div>
  <Validator value={selected?._id} fn={_string({ required: true })} />
</div>