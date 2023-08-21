<script lang="ts">
  export let data: import("./$types").PageData;
  
  import Page from "$lib/components/Page.svelte";
	import PageTop from "$lib/components/PageMenu/PageTop.svelte";
	import { lang } from "$lib/locale";
	import { locale } from "$lib/locale";
	import { ripple } from "$share/ripple";
  import StatsMap from "$share/Map/StatsMap.svelte";
	import PageMenuItem from "$lib/components/PageMenu/PageMenuItem.svelte";
	import { mdiTrashCanOutline } from "@mdi/js";
	import { _delete, action } from "$share/net.client";
	import { _message } from "$share/notify";
	import { goto } from "$app/navigation";
	import { invalidate_siblings } from "$lib/invalidate";
	import Dialog from "$share/Dialog.svelte";
  import { STATION_PICTURES_VERSION } from "$defs/constants";
	

  const date = (d: string | Date) => {
    const date = new Date(d);
    return date.toLocaleString($lang, {
      year: "numeric",
      month: "long"       ,
      day: "numeric",
      weekday: "long",
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
    })
  }

  let delete_open = false;
  let deleting = false;
  const del = action(async () => {
    if(deleting) return;
    deleting = true;
    try {
      await _delete(`/api/stations/${data.station._id}`);
      delete_open = false;
      _message("Station deleted");
      await goto("/stations", { invalidateAll: true });
      invalidate_siblings();
      deleting = false;
    } catch(e) {
      deleting = false;
      throw e;
    }
  })
</script>

<style>
  .title-pic { 
    width: 4rem;
    height: 4rem;
    border-radius: 0.5rem;
    background-position: center;
    background-repeat: no-repeat;
    background-size: contain;
  }

  .data {
    background: #fff;
    border-radius: 0.5rem;
    box-shadow: var(--some-shadow);
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
    margin-top: 1.5rem;
  }

  .data-item {
    display: flex;
    flex-direction: row;
    align-items: flex-start;
    justify-content: flex-start;
    gap: 0.4rem;
    font-size: 1.1rem;
  }

  .data-label {
    color: #333;
    white-space: nowrap;
  }

  .data-value {
    font-weight: 700;
    flex: 1;
  }

  .map {
    box-shadow: var(--some-shadow);
    border-radius: 0.5rem;
    background: #fff;
    margin-top: 1rem;
  }

  .section {
    margin-top: 5rem;
  }

  .section-title {
    font-weight: 600;
    font-size: 1.75rem;
    text-align: start;
  }

  .section-box {
    background: #fff;
    box-shadow: var(--some-shadow);
    border-radius: 0.5rem;
    margin-top: 1.5rem;
    padding: 0.5rem;
  }

  .account-item {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    padding: 0.75rem;
    transition: background-color 200ms ease;
    border-radius: 0.25rem;
  }

  .account-item:hover {
    background: rgba(0,0,0,0.05)
  }

  .account-name {
    font-size: 1.1rem;
    font-weight: 600;
  }

  .account-listeners {
    color: #333;
    font-size: 0.9rem;
  }

  .section-empty {
    padding: 1rem;
  }

  .dialog-btns {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: flex-end;
		gap: 1.5rem;
		margin-top: 2rem;
	}

	.dialog-btn-delete,
	.dialog-btn-cancel {
		padding: 0.5rem 0.75rem;
		display: flex;
		flex-direction: row;
		align-items: center;
		border-radius: 0.25rem;
		transition: background-color 150ms ease;
	}

	.dialog-btn-delete:hover,
	.dialog-btn-cancel:hover {
		background: rgba(0, 0, 0, 0.05);
	}

  .dialog-btn-delete {
		font-weight: 500;
		color: var(--red);
		border: 2px solid var(--red);
		box-shadow: 0 4px 8px #0000001f, 0 2px 4px #00000014;
	}

	.dialog-btn-cancel {
		color: #555;
	}
</style>

<svelte:head>
  <title>{data.station.name}</title>
</svelte:head>

<Page>
  <PageTop>
    <div slot="icon" class="title-pic" 
      style:background-image="url({data.config.storage_public_url}/url(station-pictures/webp/64/{data.station.picture_id}.webp?v={STATION_PICTURES_VERSION})"
    />
    
    <svelte:fragment slot="title">
      {data.station.name}
    </svelte:fragment>

    <svelte:fragment slot="subtitle">
      Station
    </svelte:fragment>

    <svelte:fragment slot="menu" let:close_menu>
      <PageMenuItem icon={mdiTrashCanOutline} on_click={() => { delete_open = true; close_menu() }}>
        Delete this station
      </PageMenuItem>
    </svelte:fragment>
  </PageTop>

  <div class="data">
    <div class="data-item">
      <div class="data-label">
        Id:
      </div>
      <div class="data-value">
        {data.station._id}
      </div>
    </div>

    <div class="data-item">
      <div class="data-label">
        Account Id:
      </div>
      <div class="data-value">
        {data.station.account_id}
      </div>
    </div>

    <div class="data-item">
      <div class="data-label">
        Created at:
      </div>
      <div class="data-value">
        {date(data.station.created_at)}
      </div>
    </div>

    {#if data.station.deleted_at != null}
      <div class="data-item">
        <div class="data-label">
          Deleted at:
        </div>
        <div class="data-value">
          {date(data.station.deleted_at)}
        </div>
      </div>
    {/if}
  </div>

  <div class="map">
    <StatsMap
      kind="station"
      record_id={data.station._id}
      locale={$locale.stats_map}
      country_names={$locale.countries}
      bind:data={data.stats}
    />
  </div>

  <div class="section">
    <div class="section-title">
      Account
    </div>
    <div class="section-box accounts">
      {#if data.account != null}
        <a href="/accounts/{data.account._id}" class="na section-item account-item ripple-container" use:ripple>
          <div class="account-name">
            {data.account.name}
          </div>
          <div class="account-listeners">
            {data.account.limits.listeners.used}
            {data.account.limits.listeners.used === 1 ? "listener" : "listeners"}
          </div>
        </a>
      {:else}
        <div class="section-empty">
          Account with id {data.station.account_id} not found
        </div>
      {/if}
    </div>
  </div>
</Page>

{#if delete_open}
  <Dialog title="Delete station {data.station.name}" width="500px" on_close={() => { delete_open = false }}>
    <div class="dialog">
      <div class="dialog-text">
        Delete station <b>{data.station.name}</b>.<br /><br />
        This action is permanent.
      </div>
      <div class="dialog-btns">
        <button
          class="dialog-btn-cancel ripple-container"
          use:ripple
          on:click={() => { delete_open = false }}
        >
          Cancel
        </button>

        <button class="dialog-btn-delete ripple-container" use:ripple on:click={del}>
          Delete
        </button>
      </div>
    </div>
  </Dialog>
{/if}