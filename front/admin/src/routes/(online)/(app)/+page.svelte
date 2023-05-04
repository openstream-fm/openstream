<script lang="ts">
  export let data: import("./$types").PageData;
	import Page from "$lib/components/Page.svelte";
	import { ripple } from "$share/ripple";

  $: admins_subtitle = data.admins.total === 1 ? `${data.admins.total} admin` : `${data.admins.total} admins`;
  $: users_subtitle = data.users.total === 1 ? `${data.users.total} user` : `${data.users.total} users`;
  $: accounts_subtitle = data.accounts.total === 1 ? `${data.accounts.total} account` : `${data.accounts.total} accounts`;
  $: stations_subtitle = data.stations.total === 1 ? `${data.stations.total} station` : `${data.stations.total} stations`;
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
    grid-template-columns: repeat(auto-fit, minmax(15rem, 1fr));
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
  }

  .section:hover {
    background-color: rgba(0,0,0,0.025);
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

<Page>
  <input type="text" class="search-bar" placeholder="Search anything..." />

  <div class="sections">
    <a href="/admins" class="na ripple-container section" use:ripple>
      <div class="section-title">Admins</div>
      <div class="section-subtitle">{admins_subtitle}</div>
    </a>

    <a href="/users" class="na ripple-container section" use:ripple>
      <div class="section-title">Users</div>
      <div class="section-subtitle">{users_subtitle}</div>
    </a>
    
    <a href="/accounts" class="na ripple-container section" use:ripple>
      <div class="section-title">Accounts</div>
      <div class="section-subtitle">{accounts_subtitle}</div>
    </a>

    <a href="/stations" class="na ripple-container section" use:ripple>
      <div class="section-title">Stations</div>
      <div class="section-subtitle">{stations_subtitle}</div>
    </a>

  </div>
</Page> 