<script lang="ts">
	import { goto } from "$app/navigation";
  export let data: import("./$types").PageData;
  import Page from "$lib/components/Page.svelte";
	import PageMenuItem from "$lib/components/PageMenu/PageMenuItem.svelte";
	import PageTop from "$lib/components/PageMenu/PageTop.svelte";
	import { invalidate_siblings } from "$lib/invalidate";
	import { lang } from "$lib/locale";
	import Dialog from "$share/Dialog.svelte";
	import { _delete, _post, action } from "$share/net.client";
	import { _message } from "$share/notify";
	import { ripple } from "$share/ripple";
	import { mdiLogin, mdiTrashCanOutline } from "@mdi/js";
  import { STATION_PICTURES_VERSION } from "$defs/constants";
	import { DELETE, unwrap } from "$lib/client";

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

  const login_as = action(async (close: () => void) => {
    const payload = ({
      title: "Admin login as user",
    }) satisfies import("$server/defs/api/auth/admin/delegate/[user]/POST/Payload").Payload;
    
    await _post<import("$server/defs/api/auth/admin/delegate/[user]/POST/Output").Output>(`/api/auth/admin/delegate/${data.user._id}`, payload);
    const target = `${data.config.studio_public_url}/`;
    window.open(target, "_blank")
    close();  
  })

  let delete_open = false;
  let deleting = false;
  const del = action(async () => {
    if(deleting) return;
    deleting = true;
    try {
      unwrap(await DELETE("/users/{user}", { params: { path: { user: data.user._id } } }));
      delete_open = false;
      _message("User deleted");
      await goto("/users", { invalidateAll: true });
      invalidate_siblings();
      deleting = false;
    } catch(e) {
      deleting = false;
      throw e;
    }
  })
</script>

<style>
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
    font-weight: var(--font-bold);
    flex: 1;
  }

  .section {
    margin-top: 5rem;
  }

  .section-title {
    font-weight: var(--font-bold);
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
    font-weight: var(--font-bold);
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

  .letter {
    border-radius: 50%;
    width: 3.5rem;
    height: 3.5rem;
    box-shadow: var(--some-shadow);
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--red);
    color: #fff;
    font-weight: var(--font-bold);
    font-size: 1.75rem;
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
  <title>{data.admin.first_name} {data.admin.last_name}</title>
</svelte:head>

<Page>
  <PageTop>
    <div slot="icon" class="letter">
      {data.user.first_name[0] || ""}
    </div>

    <svelte:fragment slot="title">
      {data.user.first_name} {data.user.last_name}
    </svelte:fragment>

    <svelte:fragment slot="subtitle">
      {data.user.email}
    </svelte:fragment>

    <svelte:fragment slot="menu" let:close_menu>
      <PageMenuItem icon={mdiLogin} on_click={() => login_as(close_menu)}>
        Login as this user
      </PageMenuItem>
      <PageMenuItem icon={mdiTrashCanOutline} on_click={() => { delete_open = true; close_menu() }}>
        Delete this user
      </PageMenuItem>
    </svelte:fragment>
  </PageTop>

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
        {@const stations = data.stations.filter(item => item.account_id === account._id)}
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
            style:background-image="url({data.config.storage_public_url}/station-pictures/webp/64/{station.picture_id}.webp?v={STATION_PICTURES_VERSION})"
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

{#if delete_open}
  <Dialog title="Delete user {data.user.first_name} {data.user.last_name}" width="500px" on_close={() => { delete_open = false }}>
    <div class="dialog">
      <div class="dialog-text">
        Delete user <b>{data.user.first_name} {data.user.last_name}</b>.<br /><br />
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