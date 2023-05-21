<script lang="ts">
	export let data: import("./$types").PageData;

  import Page from "$lib/components/Page.svelte";
	import { _delete, _patch, _post, action } from "$share/net.client";
	import { _message } from "$share/notify";
	import { ripple } from "$share/ripple";
  import { goto } from "$app/navigation";
  import Formy from "$share/formy/Formy.svelte";
	import Dialog from "$share/Dialog.svelte";
	import TextField from "$lib/components/Form/TextField.svelte";
	import Validator from "$share/formy/Validator.svelte";
	import { mdiTrashCanOutline } from "@mdi/js";
	import Icon from "$share/Icon.svelte";
	import { locale } from "$lib/locale";

  let delete_name_input_value = "";
  
  $: delete_name_is_match = delete_name_input_value.trim() === data.station.name.trim();

  let delete_open = false;

  const _station_name_validate = (v: string | null | undefined) => {
    if(v?.trim() !== data.station.name.trim()) {
      return $locale.pages["station.settings"].validate.station_name; 
    } else {
      return null;
    }
  }

  let deleting = false;
  const del = action(async () => {
    if(deleting) return;
    deleting = true;
    try {
      await _delete<import("$api/stations/[station]/DELETE/Output").Output>(`/api/stations/${data.station._id}`);
      deleting = false;
      _message($locale.pages["station.settings"].notifier.station_deleted);
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
  <title>{$locale.pages["station.settings"].head.title}</title>
</svelte:head>

<Page>
  <div class="page">
    <div class="page-title">{$locale.pages["station.settings"].title}</div>

    <h2>{$locale.pages["station.settings"].actions.title}</h2>

    <div class="actions">
      <button class="action action-delete" on:click={() => delete_open = true}>
        <div class="action-icon">
          <Icon d={mdiTrashCanOutline} />
        </div>
        {$locale.pages["station.settings"].actions.delete_station}
      </button>
    </div>
  </div>
</Page>

{#if delete_open}
  <Dialog
    width="500px"
    on_close={() => delete_open = false}
    title={$locale.pages["station.settings"].dialogs.delete_station.title.replace("@name", data.station.name)}
  >
    <Formy action={del} let:submit>
      <form novalidate on:submit={submit} class="delete-dialog">
        <div class="delete-dialog-content">
          {@html $locale.pages["station.settings"].dialogs.delete_station.message_html.replaceAll("@name", data.station.name)}
        </div>
        
        <div class="delete-dialog-field">
          <TextField
            label={$locale.pages["station.settings"].dialogs.delete_station.field_label}
            trim
            bind:value={delete_name_input_value}
          />
          <Validator value={delete_name_input_value} fn={_station_name_validate} /> 
        </div>

        <div class="delete-dialog-btns">
          <button
            class="delete-dialog-btn-cancel ripple-container"
            use:ripple
            on:click={() => (delete_open = false)}
          >
            {$locale.pages["station.settings"].dialogs.delete_station.cancel}
          </button>

          <button class="delete-dialog-btn-delete ripple-container" class:disabled={!delete_name_is_match} use:ripple>
            <div class="delete-dialog-btn-icon">
              <Icon d={mdiTrashCanOutline} />
            </div>
            {$locale.pages["station.settings"].dialogs.delete_station.submit}
          </button>
        </div>
      </form>
    </Formy>
  </Dialog>
{/if}

