<script lang="ts">
	import Page from "$lib/components/Page.svelte";
	// import { ripple } from "$lib/ripple";
	// import { fly } from "svelte/transition";
  // export let data: import("./$types").PageData;
</script>

<style>
  .page {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .page-title {
    margin-top: 2rem;
    font-size: 2rem;
    font-weight: 600;
  }

  .stations-box {
    margin-top: 3.5rem;
    width: min(80%, 500px);
    background: #fff;
    box-shadow: 0 0 15px 0 rgb(0 0 0 / 10%);
    border-radius: 0.5rem;
    padding: 1rem 0;
    display: flex;
    flex-direction: column;
    align-items: stretch;
    min-width: 0;
  }
</style>

<svelte:head>
  <title>Select station</title>
</svelte:head>

<Page>
  <div class="page">
    <div class="page-title">Create a station</div>
    <div class="stations-box"></div>
  </div>
</Page>