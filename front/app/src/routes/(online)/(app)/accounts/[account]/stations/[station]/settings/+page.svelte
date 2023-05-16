<script lang="ts">
	export let data: import("./$types").PageData;

  import Page from "$lib/components/Page.svelte";
	import { _delete, _patch, _post, action } from "$share/net.client";
	import { _message } from "$share/notify";
	import { ripple } from "$share/ripple";
  import { goto } from "$app/navigation";
  import StationProfile from "$lib/components/StationProfile.svelte";
  import Formy from "$share/formy/Formy.svelte";
	import Dialog from "$share/Dialog.svelte";
	import TextField from "$lib/components/Form/TextField.svelte";
	import Validator from "$share/formy/Validator.svelte";
	import { mdiTrashCanOutline } from "@mdi/js";
	import Icon from "$share/Icon.svelte";

  let delete_name_input_value = "";
  
  $: delete_name_is_match = delete_name_input_value.trim() === data.station.name.trim();

  let delete_open = false;

  const _station_name_validate = (v: string | null | undefined) => {
    if(v?.trim() !== data.station.name.trim()) {
      return "The station name doesn't match"
    } else {
      return null;
    }
  }

  let deleting = false;
  const del = action(async () => {
    if(deleting) return;
    deleting = true;
    try {

      if(delete_name_input_value !== data.station.name) {
        throw new Error("Confirmation name doesn't match");
      }

      await _delete<import("$api/stations/[station]/DELETE/Output").Output>(`/api/stations/${data.station._id}`);
      
      deleting = false;
      _message("Station deleted");
      goto(`/accounts/${data.station.account_id}/stations`, { invalidateAll: true });
    } catch(e) {
      deleting = false;
      throw e;
    }
  });
  
</script>

<style>
  .page {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .page-title {
    margin-top: 4rem;
    margin-bottom: 2rem;
    font-size: 2rem;
    font-weight: 600;
  }

  h2 {
    margin-top: 3rem;
    font-weight: 600;
  }

  .actions {
    width: min(500px, 80%);
    background: #fff;
    box-shadow: var(--some-shadow);
    border-radius: 0.5rem;
    margin-top: 1.5rem;
    padding: 0.5rem 0;
    display: flex;
    flex-direction: column;
    align-items: stretch;
  }

  .action {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-start;
    padding: 0.75rem 1rem;
    transition: background-color 200ms ease;
  } 

  .action:hover {
    background-color: rgba(0,0,0,0.05);
  }

  .action-icon {
    display: flex;
    font-size: 1.25rem;
    margin-inline-end: 0.5rem;
  }

  .action-delete {
    color: var(--red);
  }

  .delete-dialog-btns {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: flex-end;
		gap: 1.5rem;
		margin-top: 2rem;
	}

	.delete-dialog-btn-delete,
	.delete-dialog-btn-cancel {
		padding: 0.5rem 0.75rem;
		display: flex;
		flex-direction: row;
		align-items: center;
		border-radius: 0.25rem;
		transition: background-color 150ms ease;
	}

	.delete-dialog-btn-delete:hover,
	.delete-dialog-btn-cancel:hover {
		background: rgba(0, 0, 0, 0.05);
	}

	.delete-dialog-btn-delete {
		font-weight: 500;
		color: var(--red);
		border: 2px solid var(--red);
		box-shadow: 0 4px 8px #0000001f, 0 2px 4px #00000014;
	}

	.delete-dialog-btn-cancel {
		color: #555;
	}

	.delete-dialog-btn-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		margin-inline: -0.25rem 0.5rem;
		font-size: 1.2rem;
	}

  .delete-dialog-field {
    margin-top: 2rem;
  }
</style>

<svelte:head>
  <title>{data.station.name} Settings</title>
</svelte:head>

<Page>
  <div class="page">
    <div class="page-title">{data.station.name} Settings</div>

    <h2>Actions</h2>

    <div class="actions">
      <button class="action action-delete" on:click={() => delete_open = true}>
        <div class="action-icon">
          <Icon d={mdiTrashCanOutline} />
        </div>
        Delete station
      </button>
    </div>
  </div>
</Page>

{#if delete_open}
  <Dialog width="500px" on_close={() => delete_open = false} title="Delete station {data.station.name}">
    <Formy action={del} let:submit>
      <form novalidate on:submit={submit} class="delete-dialog">
        <div class="delete-dialog-content">
          Deletion of a station is a permanent action, you won't be able to access the station's data again, so be sure of what you are doing. <br />
          <br />
          If you really want to delete the station {data.station.name} type the name of the station in the following box: <b>{data.station.name}</b><br /> 
        </div>
        
        <div class="delete-dialog-field">
          <TextField label="Station name" trim bind:value={delete_name_input_value} />
          <Validator value={delete_name_input_value} fn={_station_name_validate} /> 
        </div>

        <div class="delete-dialog-btns">
          <button
            class="delete-dialog-btn-cancel ripple-container"
            use:ripple
            on:click={() => (delete_open = false)}
          >
            Cancel
          </button>

          <button class="delete-dialog-btn-delete ripple-container" class:disabled={!delete_name_is_match} use:ripple>
            <div class="delete-dialog-btn-icon">
              <Icon d={mdiTrashCanOutline} />
            </div>
            Delete
          </button>
        </div>
      </form>
    </Formy>
  </Dialog>
{/if}

