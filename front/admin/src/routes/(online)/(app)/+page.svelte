<script lang="ts">
  export let data: import("./$types").PageData;
	import Page from "$lib/components/Page.svelte";
	import Icon from "$share/Icon.svelte";
	import { ripple } from "$share/ripple";
	import { mdiAccountMultipleOutline, mdiAccountOutline, mdiCurrencyUsd, mdiPoll, mdiRadioTower, mdiShieldAccountOutline } from "@mdi/js";

  $: admins_subtitle = data.admins.total === 1 ? `${data.admins.total} admin` : `${data.admins.total} admins`;
  $: users_subtitle = data.users.total === 1 ? `${data.users.total} user` : `${data.users.total} users`;
  $: accounts_subtitle = data.accounts.total === 1 ? `${data.accounts.total} account` : `${data.accounts.total} accounts`;
  $: stations_subtitle = data.stations.total === 1 ? `${data.stations.total} station` : `${data.stations.total} stations`;
  $: plans_subtitle = data.plans.items.length === 1 ? `${data.plans.items.length} plan` : `${data.plans.items.length} plans`;
</script>

<style>

  .search-bar {
    box-shadow: var(--some-shadow);
    border-radius: 5rem;
    background: #fff;
    display: block;
    width: 100%;
    font: inherit;
    font-size: 1.1rem;
    padding: 1.75rem 1.25rem;
    border: 0;
    height: 3rem;
  }

  .sections {
    margin-top: 2rem;
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(20rem, 1fr));
    gap: 1.5rem;
  }

  .section {
    background: #fff;
    border-top: var(--red) 3px solid;
    padding: 1rem;
    border-radius: 0 0 0.5rem 0.5rem;
    box-shadow: var(--some-shadow);
    flex-basis: 15rem;
    flex-grow: 1;
    flex-shrink: none;
    transition: background-color 200ms ease;
    display: flex;
    flex-direction: row;
    align-items: center;
  }

  .section:hover {
    background-color: rgba(0,0,0,0.025);
  }

  .section-icon {
    display: flex;
    font-size: 1.75rem;
    flex: none;
    margin-inline-end: 1.25rem;
  }

  .section-data {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
  }

  .section-title {
    font-weight: 600;
    font-size: 1.25rem;
  }
  
  .section-subtitle {
    color: #555;
    font-size: 0.9rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    margin-top: 0.25rem;
  }
</style>

<svelte:head>
  <title>Admin</title>
</svelte:head>

<Page>
  <input type="text" class="search-bar" placeholder="Search anything..." />

  <div class="sections">
    
    <a href="/admins" class="na ripple-container section" use:ripple>
      <div class="section-icon">
        <Icon d={mdiShieldAccountOutline} />
      </div>
      <div class="section-data">
        <div class="section-title">Admins</div>
        <div class="section-subtitle">{admins_subtitle}</div>
      </div>
    </a>

    <a href="/users" class="na ripple-container section" use:ripple>
      <div class="section-icon">
        <Icon d={mdiAccountMultipleOutline} />
      </div>
      <div class="section-data">
        <div class="section-title">Users</div>
        <div class="section-subtitle">{users_subtitle}</div>
      </div>
    </a>
    
    <a href="/accounts" class="na ripple-container section" use:ripple>
      <div class="section-icon">
        <Icon d={mdiAccountOutline} />
      </div>
      <div class="section-data">
        <div class="section-title">Accounts</div>
        <div class="section-subtitle">{accounts_subtitle}</div>
      </div>
    </a>

    <a href="/stations" class="na ripple-container section" use:ripple>
      <div class="section-icon">
        <Icon d={mdiRadioTower} />
      </div>
      <div class="section-data">
        <div class="section-title">Stations</div>
        <div class="section-subtitle">{stations_subtitle}</div>
      </div>
    </a>

    <a href="/plans" class="na ripple-container section" use:ripple>
      <div class="section-icon">
        <Icon d={mdiCurrencyUsd} />
      </div>
      <div class="section-data">
        <div class="section-title">Plans</div>
        <div class="section-subtitle">{plans_subtitle}</div>
      </div>
    </a>

    <a href="/analytics" class="na ripple-container section" use:ripple>
      <div class="section-icon">
        <Icon d={mdiPoll} />
      </div>
      <div class="section-data">
        <div class="section-title">Analytics</div>
        <div class="section-subtitle">Get insights from the data</div>
      </div>
    </a>

  </div>
</Page> 