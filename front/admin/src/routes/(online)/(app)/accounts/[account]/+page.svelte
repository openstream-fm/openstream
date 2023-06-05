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

  .member-item {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    padding: 0.75rem;
    transition: background-color 200ms ease;
    border-radius: 0.25rem;
  }

  .member-item:hover {
    background: rgba(0,0,0,0.05)
  }

  .member-name {
    font-size: 1.1rem;
    font-weight: 600;
  }

  .member-relation {
    color: #666;
    font-size: 0.8rem;
  }

  .plan-item {
    padding: 0.75rem;
    transition: background-color 200ms ease;
    border-radius: 0.25rem;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
  }

  .plan-item:hover {
    background: rgba(0,0,0,0.05)
  }

  .plan-name {
    font-weight: 600;
    font-size: 1.1rem;
  }

  .plan-data {
    font-size: 0.9rem;
    color: #333;
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
  <title>{data.account.name}</title>
</svelte:head>

<Page>
  <h1>{data.account.name}</h1>
  <p>Account</p>

  <div class="data">
    <div class="data-item">
      <div class="data-label">
        Id:
      </div>
      <div class="data-value">
        {data.account._id}
      </div>
    </div>
    <div class="data-item">
      <div class="data-label">
        Created at:
      </div>
      <div class="data-value">
        {date(data.account.created_at)}
      </div>
    </div>

    <!-- {#if data.account.deleted_at != null}
      <div class="data-item">
        <div class="data-label">
          Deleted at:
        </div>
        <div class="data-value">
          {date(data.account.deleted_at)}
        </div>
      </div>
    {/if} -->
  </div>

  <div class="section">
    <div class="section-title">
      Members
    </div>
    <div class="section-box accounts">
      {#each data.members as member (member._id)}
        <a href="/users/{member._id}" class="na section-item member-item ripple-container" use:ripple>
          <div class="member-name">{member.first_name} {member.last_name}</div>
          <div class="member-relation">
            {#if member.relation === "owner"}
              Owner
            {:else if member.relation === "staff"}
              Staff
            {/if}
          </div>
        </a>
      {:else}
        <div class="section-empty">
          This account doesn't have members
        </div>
      {/each}
    </div>
  </div>

  <div class="section">
    <div class="section-title">
      Plan
    </div>
    <div class="section-box accounts">
      {#if data.plan != null}
        <a href="/plans/{data.plan._id}" class="na section-item plan-item ripple-container" use:ripple>
          <div class="plan-name">{data.plan.display_name}</div>
          <div class="plan-data">
            $ {data.plan.price}
          </div>
        </a>
      {:else}
        <div class="section-empty">
          Plan with id {data.account.plan_id} not found
        </div>
      {/if}
    </div>
  </div>

  <div class="section">
    <div class="section-title">
      Stations
    </div>
    <div class="section-box accounts">
      {#each data.account_stations as station (station._id)}
        <a href="/stations/{station._id}" class="na section-item station-item ripple-container" use:ripple>
          <div class="station-pic" 
            style:background-image="url({data.config.storage_public_url}/station-pictures/webp/64/{station.picture_id}.webp)"
          />
          <div class="station-data">
            <div class="station-name">
              {station.name}
            </div>
          </div>
        </a>
      {:else}
        <div class="section-empty">
          This account doesn't have stations
        </div>
      {/each}
    </div>
  </div> 

</Page>