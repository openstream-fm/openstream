<script lang="ts">
  import { page } from "$app/stores";
	import { ripple } from "$lib/ripple";
	import { fly } from "svelte/transition";

  $: error = $page.error!;
</script>

<style>
  h1 {
    text-align: center;
    font-size: 7rem;
    filter: drop-shadow(var(--red) 0.025em 0.025em 0);
  }

  h2 {
    text-align: center;
    margin-top: 1rem;
    font-size: 2.5rem;
  }

  .code {
    margin-top: 2rem;
    padding: 0.5rem;
    border: rgba(0,0,0,0.1) 1px solid;
    border-radius: 0.25rem;
    background: rgba(0,0,0,0.05);
  }

  .btns {
    margin-top: 1rem;
    display: flex;
    flex-direction: row;
    gap: 2rem;
    align-items: center;
    justify-content: center;
  }

  .btn {
    display: block;
    margin: 1rem 0;
    padding: 0.75rem 1rem;
    color: #fff;
    background-color: var(--blue);
    border: 0;
    appearance: none;
    text-decoration: none;
    font-weight: 500;
    box-shadow: 0 4px 8px 0 rgb(0 0 0 / 12%), 0 2px 4px 0 rgb(0 0 0 / 8%);
    border-radius: 0.25rem;
  }

  .top {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    padding: 1.5rem;
  }

  .title {
    color: var(--red);
    font-size: min(6vw, 2rem);
    font-weight: 600;
  }

  .layout {
    flex: 1;
    display: flex;
    flex-direction: column;
    background: var(--bg-gray);
  }

  .page {
    display: flex;
    flex-direction: column;
    align-items: center;
  }
</style>

<svelte:head>
  <title>{error?.status || 500} {error?.message || "Error"}</title>
</svelte:head>

<div class="layout" in:fly|local={{ x: -25, duration: 200 }}>
  <div class="top">
    <div class="title">
      openstream
    </div>
  </div>
  <div class="page">
    <h1>{error.status ?? 500}</h1>

    <h2>{error.message ?? "An error ocurred"}</h2>

    <div class="code">
      {error.code ?? "CLIENT_PAGE_MISSING_CODE"}
    </div>

    <div class="btns">
      {#if error.status !== 404}
        <button on:click={() => location.reload()} use:ripple class="ripple-container btn retry">
          Retry
        </button>
      {/if}
      <a href="/" use:ripple class="ripple-container btn home">
        Take me to home
      </a>
    </div>
  </div>
</div>
