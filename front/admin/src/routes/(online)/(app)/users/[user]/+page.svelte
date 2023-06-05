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
  }

  .account-station-count {
    color: #666;
    font-size: 0.9rem;
  }

  .station-item {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-start;
    padding: 0.75rem;
    gap: 1rem;
    transition: background-color 200ms ease;
  }

  .station-item:hover {
    background: rgba(0,0,0,0.05)
  }

  .station-pic {
    width: 2.75rem;
    height: 2.75rem;
    border-radius: 0.5rem;
    background-position: center;
    background-size: contain;
    background-repeat: no-repeat
  }

  .station-name {
    font-weight: 600;
    font-size: 1.1rem;
  }

  .station-account-name {
    font-size: 0.9rem;
    color: #333; 
  }

  .station-data {
    flex: 1;
    gap: 0.2rem;
    display: flex;
    flex-direction: column;
    align-items: flex-start
  }

  .section-empty {
    padding: 1rem;
  }
</style>

<svelte:head>
  <title>{data.admin.first_name} {data.admin.last_name}</title>
</svelte:head>

<Page>
  <h1>{data.admin.first_name} {data.admin.last_name}</h1>
  <p>User</p>

  <div class="data">

    <div class="data-item">
      <div class="data-label">
        Id:
      </div>
      <div class="data-value">
        {data.user._id}
      </div>
    </div>

    <div class="data-item">
      <div class="data-label">
        First name:
      </div>
      <div class="data-value">
        {data.user.first_name}
      </div>
    </div>

    <div class="data-item">
      <div class="data-label">
        Last name:
      </div>
      <div class="data-value">
        {data.user.last_name}
      </div>
    </div>


    <div class="data-item">
      <div class="data-label">
        Email:
      </div>
      <div class="data-value">
        {data.user.email}
      </div>
    </div>

    <div class="data-item">
      <div class="data-label">
        Registered at:
      </div>
      <div class="data-value">
        {date(data.user.created_at)}
      </div>
    </div>
  </div>

  <div class="section">
    <div class="section-title">
      Accounts
    </div>
    <div class="section-box accounts">
      {#each data.user_accounts.items as account (account._id)}
        {@const stations = data.stations.items.filter(item => item.account_id === account._id)}
        <a href="/accounts/{account._id}" class="na section-item account-item ripple-container" use:ripple>
          <div class="account-name">
            {account.name}
          </div>
          <div class="account-station-count">
            {stations.length}
            {stations.length === 1 ? "station" : "stations"}
          </div>
        </a>
      {:else}
        <div class="section-empty">
          This user doesn't have accounts
        </div>
      {/each}
    </div>
  </div>

  <div class="section">
    <div class="section-title">
      Stations
    </div>
    <div class="section-box accounts">
      {#each data.user_stations as station (station._id)}
        {@const account = data.user_accounts.items.find(item => item._id === station.account_id)}
        <a href="/stations/{station._id}" class="na section-item station-item ripple-container" use:ripple>
          <div class="station-pic" 
            style:background-image="url({data.config.storage_public_url}/station-pictures/webp/64/{station.picture_id}.webp)"
          />
          <div class="station-data">
            <div class="station-name">
              {station.name}
            </div>
            <div class="station-account-name">
              {account?.name || `#${station.account_id}`}
            </div>
          </div>
        </a>
      {:else}
        <div class="section-empty">
          This user doesn't have stations
        </div>
      {/each}
    </div>
  </div>

</Page>