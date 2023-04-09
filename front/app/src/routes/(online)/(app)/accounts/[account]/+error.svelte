<script lang="ts">
  import { page } from "$app/stores";
	import Page from "$lib/components/Page.svelte";
	import { ripple } from "$lib/ripple";

  $: error = $page.error!;
</script>

<style>

  .page {
    padding: 0 1rem;
  }

  h1 {
    text-align: left;
    font-size: 7rem;
    filter: drop-shadow(var(--red) 0.025em 0.025em 0);
  }

  h2 {
    text-align: left;
    margin-top: 1rem;
    font-size: 2.5rem;
  }

  .code {
    margin-top: 2rem;
  }

  .btns {
    margin-top: 1rem;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
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
  }
</style>

<svelte:head>
  <title>{error?.status || 500} {error?.message || "Error"}</title>
</svelte:head>

<Page>
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
</Page>