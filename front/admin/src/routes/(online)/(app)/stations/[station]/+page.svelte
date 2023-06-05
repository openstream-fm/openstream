<script lang="ts">
  export let data: import("./$types").PageData;
  import Page from "$lib/components/Page.svelte";
	import { ripple } from "$share/ripple";

  const date = (d: string | Date) => {
    const date = new Date(d);
    return date.toLocaleString(undefined, {
      year: "numeric",
      month: "long"       ,
      day: "numeric",
      weekday: "long",
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
    })
  }
</script>

<style>

  .title {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-start;
    gap: 1rem;
  }

  .title-pic { 
    width: 4rem;
    height: 4rem;
    border-radius: 0.5rem;
    background-position: center;
    background-repeat: no-repeat;
    background-size: contain;
  }

  p {
    color: #444;
    font-size: 0.9rem;
    margin-inline-start: 0.25rem;
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

  .section {
    margin-top: 5rem;
  }

  .section-title {
    font-weight: 600;
    font-size: 1.75rem;
    text-align: center;
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
</style>

<svelte:head>
  <title>{data.station.name}</title>
</svelte:head>

<Page>
  <div class="title">
    <div class="title-pic" 
      style:background-image="url({data.config.storage_public_url}/station-pictures/webp/64/{data.station.picture_id}.webp)"
    />
    <div class="title-data">
      <h1>{data.station.name}</h1>
      <p>Station</p>
    </div>
  </div>

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