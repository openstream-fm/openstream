<script lang="ts">
	import Icon from "$share/Icon.svelte";

  export let menu_open = false;
  export let icon: string | null = null;

  import PageMenu from "./PageMenu.svelte";

  export const close_menu = () => {
    menu_open = false;
  }
</script>

<style>
  .page-top {
    display: flex;
    flex-direction: row;
    align-items: flex-start;
    justify-content: flex-start;
  }

  .titles {
    align-self: center;
  }

  p {
    color: #444;
    font-size: 0.9rem;
    margin-inline-start: 0.25rem;
  }



  .custom-icon {
    flex: none;
    display: flex;
    margin-inline-end: 1rem;
    align-self: center;
  }

  .icon {
    flex: none;
    display: flex;
    font-size: 3rem;
    margin-inline-end: 1rem;
    align-self: center;
    color: #555;
  }

  .page-menu {
    flex: none;
    display: flex;
    margin-inline-start: auto;
  }

  .action {
    flex: none;
    display: flex;
    margin-inline-start: auto;
  }
</style>

<div class="page-top">
  {#if $$slots.icon}
    <div class="custom-icon">
      <slot name="icon" />
    </div>
  {:else if icon != null}
    <div class="icon">
      <Icon d={icon} />
    </div>
  {/if}

  <div class="titles">
    <h1>
      <slot name="title" />
    </h1>
    {#if $$slots.subtitle}
      <p>
        <slot name="subtitle" />
      </p>
    {/if}
  </div>
  {#if $$slots.menu}
    <div class="page-menu">  
      <PageMenu bind:open={menu_open}>
        <slot name="menu" {close_menu} />
      </PageMenu>
    </div>  
  {:else if $$slots.action}
    <div class="action">
      <slot name="action" />
    </div>
  {/if}
</div>