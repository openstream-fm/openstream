<script lang="ts">
	export let data: import("./$types").PageData;
  
  import Page from "$lib/components/Page.svelte";
	import { locale } from "$lib/locale";
	import { ripple } from "$share/ripple";
  import { STATION_PICTURES_VERSION } from "$server/defs/constants";
  
  $: current_account_stations = data.stations.items.filter(item => item.account_id === data.account._id);
</script>

<style>
  .page {
    display: flex;
    flex-grow: 1;
    flex-direction: column;
    align-items: center;
    padding-bottom: 3rem;
  }

  .page-title {
    margin-top: 2rem;
    font-size: 2rem;
    font-weight: 600;
  }

  .list-box {
    margin-top: 3.5rem;
    width: min(90%, 500px);
    background: #fff;
    box-shadow: 0 0 15px 0 rgb(0 0 0 / 10%);
    border-radius: 0.5rem;
    padding: 1rem 0;
    display: flex;
    flex-direction: column;
    align-items: stretch;
    min-width: 0;
  }

  .list-item {
    padding: 1rem 2rem;
    cursor: pointer;
    user-select: none;
    font-size: 1.1rem;
    transition: background-color 150ms ease;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-start;
  }


  .list-item:hover {
    background: #eee;
  }

  .list-item-pic {
    border-radius: 0.25rem;
    flex: none;
    overflow: hidden;
    background-size: contain;
    background-position: center;
    background-repeat: no-repeat;
    width: 2rem;
    height: 2rem;
    margin-inline-end: 1rem;
  }

  .list-item-name {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    flex: 1;
  }

  .no-items {
    padding: 2rem;
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .no-items-message {
    font-size: 1.1rem;
    text-align: center;
    line-height: 1.5rem;
  }

  @media screen and (max-width: 460px) {
    .no-items-message {
      font-size: 1rem;
    }
  }

  .no-items-create {
    background: var(--blue);
    color: #fff;
    cursor: pointer;
    user-select: none;
    padding: 0.75rem 1rem;
    border-radius: 0.25rem;
    margin-top: 2rem;
    box-shadow: 0 4px 8px 0 rgb(0 0 0 / 12%), 0 2px 4px 0 rgb(0 0 0 / 8%);
    font-weight: 500;
  }

  .or {
    margin-top: 1.5rem;
    color: #666;
  }

  .create {
    margin-top: 0.25rem;
    padding: 0.75rem;
    display: flex;
    text-align: center;
    text-decoration: none;
    color: var(--blue);
    font-size: 1.1rem;
    border-radius: 0.5rem;
  }
</style>

<svelte:head>
  <title>{$locale.pages["stations"].head.title}</title>
</svelte:head>

<Page>
  <div class="page">
    <div class="page-title">{$locale.pages["stations"].title}</div>
    {#if current_account_stations.length}
      <div class="list-box">
        {#each current_account_stations as station (station._id)}
          <a href="/accounts/{station.account_id}/stations/{station._id}" class="list-item na ripple-container" use:ripple>
            <div
              class="list-item-pic"
              style="background-image: url({data.config.storage_public_url}/url(station-pictures/webp/64/{station.picture_id}.webp?v={STATION_PICTURES_VERSION})"
            />
            <span class="list-item-name">
              {station.name}
            </span>
          </a>
        {/each}
      </div>


      <div class="or">{$locale.pages.stations.or}</div>

      <a class="create ripple-container" href="/accounts/{data.account._id}/stations/create-station" use:ripple>
        {$locale.pages.stations.create_new_station}
      </a> 
      
    {:else}
      <div class="no-items">
        <div class="no-items-message">
          {@html $locale.pages.stations.no_items_message_html}
        </div>
        <a href="/accounts/{data.account._id}/stations/create-station" class="na no-items-create ripple-container" use:ripple>
          {$locale.pages.stations.no_items_create}
        </a>
      </div>
    {/if}
  </div>
</Page>
