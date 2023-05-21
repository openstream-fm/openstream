<script lang="ts">
  $: admin = $page.data.admin || null;
  
  import { page } from "$app/stores";
  import { fly } from "svelte/transition";
	import { ripple } from "$share/ripple";
	import { click_out } from "$share/actions";
	import { action, _post } from "$share/net.client";
	import Icon from "$share/Icon.svelte";
	import { mdiAccountCircleOutline, mdiLogout } from "@mdi/js";
	import { goto } from "$app/navigation";

  const sign_out = action(async () => {
    await _post("/api/auth/admin/logout", {});
    goto("/", { invalidateAll: true })
    
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
    font-weight: 600;
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
    font-weight: 600;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.35rem;
    user-select: none;
    cursor: pointer;
    box-shadow: 0 4px 8px 0 rgba(0,0,0,.12),0 2px 4px 0 rgba(0,0,0,.08);
  }

  .menu-holder {
    position: relative;
    margin-inline-end: 1rem;
    flex: none;
  }

  .menu-position-out {
    position: absolute;
    bottom: 0;
    right: 0;
    width: 0;
    height: 0;
  }

  .menu-position-in {
    position: relative;
  }

  .menu {
    position: absolute;
    top: 0;
    right: 0;
    background: #fff;
    width: min(calc(100vw - 3rem), 21rem);
    box-shadow: 0 5px 25px 0 rgb(0 0 0 / 10%);
    border: 1px solid rgba(0,0,0,.1);
    border-radius: 0.25rem;
    display: flex;
    flex-direction: column;
    align-items: stretch;
    min-width: 0;
    max-height: calc(100vh - var(--top-h) - 1rem);
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
    font-weight: 600;
  }

  .menu-head-email {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: #666;
    margin-top: 0.2rem;
    font-size: 0.95rem;
  }


</style>

<div class="station">
  <div class="names">
    <div class="user-name">{admin?.first_name} {admin?.last_name}</div>
    <div class="account-name">{admin?.email}</div>
  </div>
  <div class="menu-holder" use:click_out={() => menu_open = false}>
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <button class="pic ripple-container" use:ripple on:click={() => menu_open = !menu_open} use:ripple>
      {admin?.first_name.charAt(0).toUpperCase()}
    </button>
    <div class="menu-position-out">
      <div class="menu-position-in">
        {#if menu_open}
          <div class="menu thin-scroll" transition:fly|local={{ y: -25, x: 10, duration: 200 }}>
            <div class="menu-head menu-section">
              <div class="menu-head-icon">
                <Icon d={mdiAccountCircleOutline} />
              </div>
              <div class="menu-head-info">
                <div class="menu-head-name">{admin?.first_name} {admin?.last_name}</div>
                <div class="menu-head-email">{admin?.email}</div>
              </div>
            </div>
            <div class="menu-section">
              <a href="/me" class="na menu-section-link ripple-container" use:ripple on:click={() => menu_open = false}>
                <div class="menu-icon">
                  <Icon d={mdiAccountCircleOutline} />
                </div>
                Profile
              </a>
            </div>
            <div class="menu-section">
              <!-- svelte-ignore a11y-click-events-have-key-events -->
              <div class="menu-section-link ripple-container" use:ripple on:click={sign_out}>
                <div class="menu-icon">
                  <Icon d={mdiLogout} />
                </div>
                Sign out
              </div>
            </div>
          </div>
        {/if}
      </div>
    </div>
  </div>
</div>