<script lang="ts">
  export let data: import("./$types").PageData;
  import Page from "$lib/components/Page.svelte";
	import PageTop from "$lib/components/PageMenu/PageTop.svelte";
	import { lang } from "$lib/locale";
	import { ripple } from "$share/ripple";
	import { mdiAccountOutline, mdiDotsVertical, mdiTrashCanOutline } from "@mdi/js";
	import Limits from "./limits.svelte";
	import Stats from "./stats.svelte";
	import Icon from "$share/Icon.svelte";
	import { _delete, _patch, action } from "$share/net.client";
	import { _message } from "$share/notify";
	import { invalidateAll, invalidate_siblings } from "$lib/invalidate";
	import Dialog from "$share/Dialog.svelte";
	import { logical_fly } from "$share/transition";
	import { click_out } from "$share/actions";
	import { goto } from "$app/navigation";
	import PageMenuItem from "$lib/components/PageMenu/PageMenuItem.svelte";
  import { STATION_PICTURES_VERSION } from "$defs/constants";
	import { DELETE, PATCH, unwrap } from "$lib/client";

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

  let selected_plan: typeof data.plans[number] | null  = null;
  let changing_plan = false;

  const change_plan = action(async () => {
    if(changing_plan) return;
    if(selected_plan == null) return;
    changing_plan = true;

    try {
      unwrap(await PATCH("/accounts/{account}", {
        params: {
          path: {
            account: data.account._id,
          }
        },
        body: {
          plan_id: selected_plan._id,
        }
      }));
      _message("Account plan updated");
      invalidateAll();
      selected_plan = null;

      changing_plan = false;
    } catch(e) {
      changing_plan = false;
      throw e;
    }
  })

  let plan_selector_open = false;
  const plan_selector_click_out = () => {
    setTimeout(() => plan_selector_open = false, 2);
  }

  let delete_open = false;
  let deleting = false;
  const del = action(async () => {
    if(deleting) return;
    deleting = true;
    try {
      unwrap(await DELETE("/accounts/{account}", {
        params: {
          path: {
            account: data.account._id,
          }
        }
      }));
      delete_open = false;
      _message("Account deleted");
      await goto("/accounts", { invalidateAll: true });
      invalidate_siblings();
      deleting = false;
    } catch(e) {
      deleting = false;
      throw e;
    }
  })
</script>

<style>

  .map {
    margin-top: 1rem;
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
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-start;
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
    font-weight: var(--font-bold);
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
    font-weight: var(--font-bold);
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
    font-weight: var(--font-bold);
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


  .dialog-btns {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: flex-end;
		gap: 1.5rem;
		margin-top: 2rem;
	}

	.dialog-btn-change-plan,
	.dialog-btn-delete,
	.dialog-btn-cancel {
		padding: 0.5rem 0.75rem;
		display: flex;
		flex-direction: row;
		align-items: center;
		border-radius: 0.25rem;
		transition: background-color 150ms ease;
	}

	.dialog-btn-change-plan:hover,
	.dialog-btn-delete:hover,
	.dialog-btn-cancel:hover {
		background: rgba(0, 0, 0, 0.05);
	}

	.dialog-btn-change-plan {
		font-weight: 500;
		color: var(--blue);
		border: 2px solid var(--blue);
		box-shadow: 0 4px 8px #0000001f, 0 2px 4px #00000014;
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

  .plan-selector {
    position: relative;
    z-index: 1;
    margin-inline-start: auto;
  }

  .plan-selector-btn {
    color: #444;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0.5rem;
    font-size: 1.5rem;
    transition: background-color 200ms ease;
    border-radius: 50%;
  }

  .plan-selector-btn:hover, .plan-selector-btn.open {
    background: rgba(0,0,0,0.05);
  }

  .plan-selector-menu {
    position: absolute;
    inset-block-start: 100%;
    inset-inline-end: 0;
    display: flex;
    flex-direction: column;
    padding: 0.25rem;
    box-shadow: var(--some-shadow);
    border-radius: 0.25rem; 
    background: #fff;
  }

  .plan-selector-item {
    font-size: 1rem;
    white-space: nowrap;
    padding: 0.5rem 1rem;
    border-radius: 0.25rem;
    transition: background-color 200ms ease;
    text-align: start;
    font-weight: 400;
  }

  .plan-selector-item:hover {
    background-color: rgba(0,0,0,0.05);
  }
</style>

<svelte:head>
  <title>{data.account.name}</title>
</svelte:head>

<Page>
  <PageTop icon={mdiAccountOutline}>
    <svelte:fragment slot="title">
      {data.account.name}
    </svelte:fragment>
    
    <svelte:fragment slot="subtitle">
      Account
    </svelte:fragment>

    <svelte:fragment slot="menu" let:close_menu>
      <PageMenuItem icon={mdiTrashCanOutline} on_click={() => { delete_open = true; close_menu() }}>
        Delete this account
      </PageMenuItem>
    </svelte:fragment>
  </PageTop>

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


  <div class="limits">
    <Limits bind:data />
  </div>

  <div class="map">
    <Stats bind:data />
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
      <div class="plan-selector">
        <button class="plan-selector-btn" class:open={plan_selector_open} on:click={() => plan_selector_open = !plan_selector_open}>
          <Icon d={mdiDotsVertical} />
        </button>
        {#if plan_selector_open}
          <div class="plan-selector-menu" transition:logical_fly|global={{ y: -15, x: 15, duration: 200 }} use:click_out={plan_selector_click_out}>
            {#each data.all_plans.filter(item => item._id !== data.plan?._id) as plan (plan._id)}
              <button class="plan-selector-item" on:click={() => { plan_selector_open = false; selected_plan = plan }}>
                Set plan to <b>{plan.display_name}</b> - <b>$ {plan.price}</b>
              </button>
            {/each}
          </div>
        {/if}
      </div>
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
            style:background-image="url({data.config.storage_public_url}/station-pictures/webp/64/{station.picture_id}.webp?v={STATION_PICTURES_VERSION})"
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

{#if selected_plan != null}
  <Dialog title="Set account plan to {selected_plan.display_name}" width="500px" on_close={() => selected_plan = null}>
    <div class="dialog">
      <div class="dialog-text">
        Plan <b>{selected_plan.display_name}</b>: <br /><br />
        Price: $ {selected_plan.price}<br />
        Stations: {selected_plan.limits.stations}<br />
        Listeners: {selected_plan.limits.listeners}<br />
        Storage: {selected_plan.limits.storage / 1_000_000_000} GB<br />
        Transfer: {selected_plan.limits.transfer / 1_000_000_000_000} TB<br />
      </div>
      <div class="dialog-btns">
        <button
          class="dialog-btn-cancel ripple-container"
          use:ripple
          on:click={() => (selected_plan = null)}
        >
          Cancel
        </button>

        <button class="dialog-btn-change-plan ripple-container" use:ripple on:click={change_plan}>
          Change plan
        </button>
      </div>
    </div>
  </Dialog>
{/if}

{#if delete_open}
  <Dialog title="Delete account {data.account.name}" width="500px" on_close={() => { delete_open = false }}>
    <div class="dialog">
      <div class="dialog-text">
        Delete account <b>{data.account.name}</b>.<br /><br />
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