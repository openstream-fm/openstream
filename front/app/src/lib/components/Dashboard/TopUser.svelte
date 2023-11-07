<script lang="ts">
  let user: import("$api/users/[user]/GET/Output").Output["user"];
  let accounts: import("$api/accounts/GET/Output").Output; 
  let account: import("$api/accounts/[account]/GET/Output").Output["account"] | null; 
  let stations: import("$api/stations/GET/Output").Output | null;
  let station: import("$api/stations/[station]/GET/Output").Output["station"] | null;
  
  import { page } from "$app/stores";

  $: user = $page.data.user || null;
  $: accounts = $page.data.accounts || null;
  $: account = $page.data.account || null;
  $: stations = $page.data.stations || null;
  $: station = $page.data.station || null;

	import { ripple } from "$share/ripple";
	import { click_out } from "$share/actions";
	import { action, _post } from "$share/net.client";
	import Icon from "$share/Icon.svelte";
	import { mdiAccountCircleOutline, mdiAccountMultipleOutline, mdiAccountPlusOutline, mdiCastAudioVariant, mdiLogout } from "@mdi/js";
	import { goto } from "$app/navigation";
	import { locale } from "$lib/locale";
	import { invalidate_siblings } from "$lib/invalidate";
	import { logical_fly } from "$share/transition";
  import { STATION_PICTURES_VERSION } from "$defs/constants";

  const sign_out = action(async () => {
    await _post("/api/auth/user/logout", {});
    goto("/", { invalidateAll: true })
    invalidate_siblings();
  })

  let menu_open = false;
</script>

<style>
  .station {
    align-self: center;
    margin-inline-start: auto;
    display: flex;
    flex-direction: row;
  }
  
  .names {
    display: flex;
    flex-direction: column;
    text-align: right;
    align-items: flex-end;
    margin-inline-end: 1rem;
    align-self: center;
  }


  .user-name {
    font-weight: var(--font-bold);
    font-size: 1rem;
    max-width: 12rem;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .account-name {
    font-size: 0.9rem;
    color: #999;
    max-width: 12rem;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    margin-top: 0.125rem;
  }

  .pic {
    width: 2.75rem;
    height: 2.75rem;
    border-radius: 50%;
    background: var(--red);
    color: #fff;
    font-weight: var(--font-bold);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.35rem;
    user-select: none;
    cursor: pointer;
    box-shadow: 0 4px 8px 0 rgba(0,0,0,.12),0 2px 4px 0 rgba(0,0,0,.08);
  }

  .menu-holder {
    z-index: var(--z-user-menu);
    position: relative;
    margin-inline-end: 1rem;
    flex: none;
  }

  .menu-position-out {
    z-index: var(--z-user-menu);
    position: absolute;
    inset-block-end: 0;
    inset-inline-end: 0;
  }

  .menu-position-in {
    position: relative;
    z-index: var(--z-user-menu);
  }

  .menu {
    position: absolute;
    inset-block-start: 0;
    inset-inline-end: 0;
    background: #fff;
    width: min(calc(100vw - 3rem), 21rem);
    box-shadow: 0 5px 25px 0 rgb(0 0 0 / 10%);
    border: 1px solid rgba(0,0,0,.1);
    border-radius: 0.25rem;
    display: flex;
    flex-direction: column;
    align-items: stretch;
    min-width: 0;
    max-height: calc(100vh - var(--top-h) - min(30vh, 6rem));
    overflow-x: hidden;
    overflow-y: auto;
    z-index: var(--z-user-menu);
  }

  .menu-section {
    display: flex;
    flex-direction: column;
    align-items: stretch;
  }

  .menu-section + .menu-section {
    border-top: #ddd 1px solid;
  }

  .menu-section-link {
    padding: 1rem 0.75rem;
    font-weight: 500;
    cursor: pointer;
    transition: background-color 200ms ease;
    display: flex;
    flex-direction: row;
    align-items: center;
  }

  .menu-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.25rem;
    margin-inline-end: 0.75rem;
  }

  .menu-section-link:not(.not-link):hover {
    background-color: #f6f6f6;
  }

  .item-list {
    /* max-height: 15rem; */
    overflow-y: auto;
  }

  .menu-station, .menu-account {
    display: block;
    padding: 0.75rem 1rem 0.75rem 0.75rem;
    transition: background-color 150ms ease;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-start;
    gap: 0.75rem;
  }
  
  .menu-station.current, .menu-account.current {
    background: #f6f6f6;
  }

  .menu-station:hover, .menu-account:hover {
    background: #e8e8e8;
  }

  .menu-account {
    padding-inline-start: 2.75rem;
  }

  .station-pic {
    border-radius: 0.25rem;
    width: 1.25rem;
    height: 1.25rem;
    background-size: contain;
    background-position: center;
    background-repeat: no-repeat;
  }

  .station-name {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    flex: 1;
  }

  .menu-head {
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: row;
    align-items: center;
  }

  .menu-head-icon {
    font-size: 3rem;
    padding: 1rem;
    display: flex;
    flex-direction: row;
  }

  .menu-head-info {
    flex: 1;
    padding: 0 1.25rem 0 0;
    font-size: 1.05rem;
  }

  .menu-head-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-weight: var(--font-bold);
  }

  .menu-head-email {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: #666;
    margin-top: 0.2rem;
    font-size: 0.95rem;
  }

  .menu-section-sign-out {
    position: sticky;
    bottom: 0;
    background: #fff;
  }

</style>

<div class="station">
  <div class="names">
    <div class="user-name">{user.first_name} {user.last_name}</div>
    {#if account != null}
      <div class="account-name">{account.name}</div>
    {/if}
  </div>
  <div class="menu-holder" use:click_out={() => menu_open = false}>
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <button class="pic ripple-container" use:ripple on:click={() => menu_open = !menu_open} use:ripple>
      {user.first_name.charAt(0).toUpperCase()}
    </button>
    <div class="menu-position-out">
      <div class="menu-position-in">
        {#if menu_open}
          <div class="menu super-thin-scroll" transition:logical_fly|local={{ y: -25, x: 10, duration: 200 }}>
            <div class="menu-head menu-section">
              <div class="menu-head-icon">
                <Icon d={mdiAccountCircleOutline} />
              </div>
              <div class="menu-head-info">
                <div class="menu-head-name">{user.first_name} {user.last_name}</div>
                <div class="menu-head-email">{user.email}</div>
              </div>
            </div>
            <div class="menu-section">
              <a href="/me" class="na menu-section-link ripple-container" use:ripple on:click={() => menu_open = false}>
                <div class="menu-icon">
                  <Icon d={mdiAccountCircleOutline} />
                </div>
                {$locale.user_menu.profile}
              </a>
            </div>
            <div class="menu-section">
              <a href="/me/invitations" class="na menu-section-link ripple-container" use:ripple on:click={() => menu_open = false}>
                <div class="menu-icon">
                  <Icon d={mdiAccountPlusOutline} />
                </div>
                {$locale.user_menu.invitations}
              </a>
            </div>
            <div class="menu-section">
              <a href="/accounts" class="na menu-section-link ripple-container" use:ripple on:click={() => menu_open = false}>
                <div class="menu-icon">
                  <Icon d={mdiAccountMultipleOutline} />
                </div>
                {$locale.user_menu.accounts}
              </a>
              <div class="item-list">
                {#each accounts.items as item (item._id)}
                  <a href="/accounts/{item._id}" class="na menu-account ripple-container" class:current={item._id === account?._id} use:ripple on:click={() => menu_open = false}>
                    {item.name}
                  </a>
                {/each}
              </div>
            </div>

            <!-- {#if stations != null}
              <div class="menu-section">
                
                {#if account != null}
                  <a href="/accounts/{account._id}/stations" class="na menu-section-link ripple-container" use:ripple on:click={() => menu_open = false}>
                    <div class="menu-icon">
                      <Icon d={mdiCastAudioVariant} />
                    </div>
                    {$locale.user_menu.stations}
                  </a>
                {:else}
                  <div class="menu-section-link not-link">
                    <div class="menu-icon">
                      <Icon d={mdiCastAudioVariant} />
                    </div>
                    {$locale.user_menu.stations}
                  </div>
                {/if}
                <div class="station-list super-thin-scroll">
                  {#each stations.items as item (item._id)}
                    <a href="/accounts/{item.account_id}/stations/{item._id}" class="na menu-station ripple-container" class:current={item._id === station?._id} use:ripple on:click={() => menu_open = false}>
                      <div class="station-pic" style="background-image: url({$page.data.config.storage_public_url}/station-pictures/webp/32/{item.picture_id}.webp?v={STATION_PICTURES_VERSION})" />
                      <span class="station-name">{item.name}</span>
                    </a>
                  {/each}
                </div>
              </div>
            {/if} -->

            <div class="menu-section menu-section-sign-out">
              <!-- svelte-ignore a11y-click-events-have-key-events -->
              <div class="menu-section-link ripple-container" use:ripple on:click={sign_out}>
                <div class="menu-icon">
                  <Icon d={mdiLogout} />
                </div>
                {$locale.user_menu.sign_out}
              </div>
            </div>
          </div>
        {/if}
      </div>
    </div>
  </div>
</div>