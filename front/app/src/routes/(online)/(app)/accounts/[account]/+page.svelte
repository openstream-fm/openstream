<script lang="ts">
  export let data: import("./$types").PageData;
  import Page from "$lib/components/Page.svelte";
  import Map from "$share/Map/Map.svelte";
</script>

<style>

  .map-box {
    max-width: 45rem;
    background: #fff;
    border-radius: 0.5rem;
    box-shadow: 0 20px 25px -5px #0000001a, 0 10px 10px -5px #0000000a;
  }

  .map-out {
    padding: min(7.5%, max(4%, 1.5rem));
  }

  .map-viewport {
    position: relative;
    padding-top: 66%;
  }
</style>

<svelte:head>
  <title>{data.account.name}</title>
</svelte:head>

<Page>
  <h1>{data.account.name}</h1>

  <div class="map-box">
    <div class="map-out">
      <div class="map-viewport">
        <Map />
      </div>
    </div>
  </div>
</Page>