<script lang="ts">
  export let accounts: import("$server/defs/api/accounts/GET/Output").Output;
  export let account: import("$server/defs/api/accounts/[account]/GET/Output").Output["account"] | null;
  export let user: import("$server/defs/api/users/[user]/GET/Output").Output["user"];

  import { fly } from "svelte/transition";
	import { ripple } from "$lib/ripple";
	import { clickOut } from "$lib/actions";
	import { action, _post } from "$share/net.client";
	import Icon from "$share/Icon.svelte";
	import { mdiAccountCircleOutline, mdiCastAudioVariant, mdiLogout } from "@mdi/js";

  const sign_out = action(async () => {
    await _post("/api/logout", {});
    location.assign("/");
  })

  let menu_open = false;
</script>

<style>
  .account {
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
    width: 300px;
    box-shadow: 0 5px 25px 0 rgb(0 0 0 / 10%);
    border: 1px solid rgba(0,0,0,.1);
    border-radius: 0.25rem;
    display: flex;
    flex-direction: column;
    align-items: stretch;
    min-width: 0;
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

  .menu-section-link:hover {
    background-color: #f6f6f6;
  }

  .menu-account {
    padding: 0.75rem 1.5rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    transition: background-color 150ms ease;
  }
  
  .menu-account.current {
    background: #f6f6f6;
  }

  .menu-account:hover {
    background: #e8e8e8;
  }
</style>

<div class="account">
  <div class="names">
    <div class="user-name">{user.first_name} {user.last_name}</div>
    {#if account != null}
      <div class="account-name">{account.name}</div>
    {/if}
  </div>
  <div class="menu-holder" use:clickOut={() => menu_open = false}>
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div class="pic ripple-container" use:ripple on:click={() => menu_open = !menu_open} use:ripple>
      {user.first_name.charAt(0).toUpperCase()}
    </div>
    <div class="menu-position-out">
      <div class="menu-position-in">
        {#if menu_open}
          <div class="menu" transition:fly|local={{ y: -25, x: 10, duration: 200 }}>
            <div class="menu-section">
              <a href="/me" class="na menu-section-link ripple-container" use:ripple>
                <div class="menu-icon">
                  <Icon d={mdiAccountCircleOutline} />
                </div>
                Profile
              </a>
            </div>
            <div class="menu-section">
              <a href="/stations" class="na menu-section-link ripple-container" use:ripple>
                <div class="menu-icon">
                  <Icon d={mdiCastAudioVariant} />
                </div>
                Stations
              </a>
              {#each accounts.items as item (item._id)}
                <a href="/stations/{item._id}" class="na menu-account" class:current={item._id === account?._id} on:click={() => menu_open = false}>
                  {item.name}
                </a>
              {/each}
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