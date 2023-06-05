<script lang="ts">
  export let menu_open = false;

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

  p {
    color: #444;
    font-size: 0.9rem;
    margin-inline-start: 0.25rem;
  }

  .page-menu {
    flex: none;
    display: flex;
    margin-inline-start: auto;
  }
</style>

<div class="page-top">
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
  {/if}
</div>