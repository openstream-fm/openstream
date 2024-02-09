<script lang="ts">
	export let data: import("./$types").PageData;

  import Page from "$lib/components/Page.svelte";
	import { _delete, _patch, _post, action } from "$share/net.client";
	import { _message } from "$share/notify";
	import { ripple } from "$share/ripple";
  import { goto } from "$app/navigation";
  import Formy from "$share/formy/Formy.svelte";
	import Dialog from "$share/Dialog.svelte";
	import TextField from "$share/Form/TextField.svelte";
	import Validator from "$share/formy/Validator.svelte";
	import { mdiSwapHorizontal, mdiTrashCanOutline } from "@mdi/js";
	import Icon from "$share/Icon.svelte";
	import { locale } from "$lib/locale";
	import { invalidateAll, invalidate_siblings } from "$lib/invalidate";
	import { _url } from "$share/formy/validate";
	import CircularProgress from "$share/CircularProgress.svelte";
	import { scale } from "svelte/transition";
	import BooleanField from "$share/Form/BooleanField.svelte";
	import TransferAccountSelector from "./TransferAccountSelector.svelte";

  let delete_name_input_value = "";
  
  $: delete_name_is_match = delete_name_input_value.trim() === data.station.name.trim();

  let delete_open = false;

  const _station_name_validate = (v: string | null | undefined) => {
    if(v?.trim() !== data.station.name.trim()) {
      return $locale.pages["station/settings"].validate.station_name; 
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
      _message($locale.pages["station/settings"].notifier.station_deleted);
      goto(`/accounts/${data.station.account_id}/stations`, { invalidateAll: true });
      invalidate_siblings();
    } catch(e) {
      deleting = false;
      throw e;
    }
  });

  const KEY = "front_previous_external_relay_url";
  const _prev = data.station.user_metadata[KEY];
  let prev = typeof _prev === "string" ? _prev : ""; 
  let external_relay_enabled = data.station.external_relay_url != null;
  let external_relay_url = data.station.external_relay_url || prev;
  let external_relay_redirect = data.station.external_relay_redirect ?? false;

  let saving_relay = false;
  const save_external_relay = action(async () => {
    
    if(saving_relay) return;
    saving_relay = true;
    
    prev = external_relay_url;

    try {
      let payload: import("$api/stations/[station]/PATCH/Payload").Payload;
      if(external_relay_enabled) {
        payload = {
          external_relay_url,
          external_relay_redirect,
          user_metadata: {
            [KEY]: external_relay_url,
          },
        }
      } else {
        payload = {
          external_relay_url: null,
          external_relay_redirect,
          user_metadata: {
            [KEY]: external_relay_url,
          }
        }
      }

      await _patch(`/api/stations/${data.station._id}`, payload);
      await invalidateAll();
      _message($locale.misc.Settings_updated);

      saving_relay = false;
    } catch(e) {
      saving_relay = false;
      throw e;
    }
  })
  

  const _parent = _url({ required: true });
  const _validate_external_relay_url = (value: string | null | undefined): string | null => {
    if(external_relay_enabled) {
      return _parent(value);
    } else {
      return null;
    }
  }

  let transfer_open = false;
  let transfer_name_check_value = "";
  $: transfer_name_is_match = transfer_name_check_value.trim() === data.station.name.trim();

  let transfer_selected_account: typeof data.accounts.items[number] | null = null;

  let _transfer_name_validate = (v: string): string | null => {
    if(v.trim() !== data.station.name.trim()) {
      return $locale.misc.Station_name_do_not_match; 
    }

    return null;
  }

  let transferring = false;
  
  let transfer = action(async () => {
    
    if(transferring) return;
    transferring = true;

    try {
      if(transfer_selected_account == null) {
        throw new Error($locale.misc.Target_account_is_required);
      }

      const payload: import("$api/stations/[station]/transfer/POST/Payload").Payload = {
        target_account_id: transfer_selected_account._id
      } 

      const { station } = await _post<import("$api/stations/[station]/transfer/POST/Output").Output>(
        `/api/stations/${data.station._id}/transfer`,
        payload
      );

      _message($locale.misc.Station_transferred);

      await goto(`/accounts/${station.account_id}`, { invalidateAll: true })
      invalidate_siblings();

      transferring = false;    
    } catch(e) {
      transferring = false;
      throw e;
    }
  })
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
    font-weight: var(--font-bold);
  }
  
  .section {
    display: flex;
    flex-direction: column;
    align-items: center;
    align-self: stretch;  
  }

  .section + .section {
    margin-top: 3rem;
  }

  h2 {
    margin-top: 3rem;
    font-weight: var(--font-bold);
  }

  .section-box {
    width: min(500px, 95%);
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

  .delete-dialog-btns, .transfer-dialog-btns {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: flex-end;
		gap: 1.5rem;
		margin-top: 2rem;
	}

	.delete-dialog-btn-delete,
	.delete-dialog-btn-cancel,
  .transfer-dialog-btn-cancel,
  .transfer-btn-no-target-ok,
	.transfer-dialog-btn-transfer {
		padding: 0.5rem 0.75rem;
		display: flex;
		flex-direction: row;
		align-items: center;
		border-radius: 0.25rem;
		transition: background-color 150ms ease;
	}

	.delete-dialog-btn-delete:hover,
	.delete-dialog-btn-cancel:hover,
  .transfer-dialog-btn-transfer:hover,
  .transfer-btn-no-target-ok:hover,
	.transfer-dialog-btn-cancel:hover {
		background: rgba(0, 0, 0, 0.05);
	}

	.delete-dialog-btn-delete, .transfer-dialog-btn-transfer {
		font-weight: 500;
		color: var(--red);
		border: 2px solid var(--red);
		box-shadow: 0 4px 8px #0000001f, 0 2px 4px #00000014;
	}

  .transfer-dialog-btn-transfer {
    border-color: var(--blue);
    color: var(--blue);
  }


	.delete-dialog-btn-cancel, .transfer-dialog-btn-cancel, .transfer-btn-no-target-ok {
		color: #555;
	}

	.delete-dialog-btn-icon, .transfer-dialog-btn-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		margin-inline: -0.25rem 0.5rem;
		font-size: 1.2rem;
	}

  .delete-dialog-field {
    margin-top: 2rem;
  }

  .relay-box {
    padding: 1.5rem;
  }

  .relay-field {
    margin-top: 1.5rem;
    transition: opacity 200ms ease;
  }

  .redirect-field {
    margin-top: 0.5rem;
  }

  .redirect-field.disabled {
    opacity: 0.5;
    pointer-events: none;
  }

  .relay-send {
    color: #fff;
    font-weight: var(--font-bold);
    background: var(--blue);
    padding: 0.75rem;
    border-radius: 0.25rem;
    display: flex;
    box-shadow: var(--some-shadow);
    position: relative;
    align-self: flex-end;
    margin-top: 1.5rem;
  }

  .relay-send-text {
    display: flex;
    flex-direction: row;
    align-items: center;
    transition: opacity 200ms ease;
  }

  .relay-send.sending > .relay-send-text {
    opacity: 0;
  }

  .relay-send-sending {
    position: absolute;
    display: flex;
    font-size: 1.25rem;
    top: calc(50% - (1.25rem / 2));
    left: calc(50% - (1.25rem / 2));
  }

  .transfer-name-field {
    margin-top: 1.5rem;
  }

  .transfer-account-selector {
    margin-top: 1rem;
  }

  .transfer-btn-no-target-ok {
    margin-inline-start: auto;
    margin-top: 1.5rem;
  }
</style>

<svelte:head>
  <title>{$locale.pages["station/settings"].head.title}</title>
</svelte:head>

<Page>
  <div class="page">
    <div class="page-title">{$locale.pages["station/settings"].title}</div>

    <div class="section">

      <h2>{$locale.misc.Master_relay}</h2>
      
      <Formy action={save_external_relay} let:submit>
        <form class="section-box relay-box" on:submit={submit}>
          <div class="relay-switch">
            <BooleanField bind:value={external_relay_enabled} label={$locale.misc.Enable_master_relay} />
          </div>
          <div class="relay-field">
            <TextField label={$locale.misc.Master_Relay_URL} disabled={!external_relay_enabled} bind:value={external_relay_url}  />
            <Validator value={external_relay_url} fn={_validate_external_relay_url} />
          </div>
          <div class="redirect-field" class:disabled={external_relay_enabled === false}>
            <BooleanField bind:value={external_relay_redirect} label={$locale.misc.Enable_master_relay_redirect_mode} />
          </div>          

          <button type="submit" class="relay-send ripple-container" use:ripple class:sending={saving_relay}>
            <div class="relay-send-text">
              <!-- <div class="invite-dialog-send-icon">
                <Icon d={mdiAccountPlusOutline} />
              </div> -->
              {$locale.misc.Save}
            </div>
            {#if saving_relay}
              <div class="relay-send-sending" transition:scale={{ duration: 300 }}>
                <CircularProgress />
              </div>
            {/if}
          </button>
        </form>
      </Formy>
    </div>

    <div class="section">
      <h2>{$locale.pages["station/settings"].actions.title}</h2>

      <div class="section-box actions">
        <button class="action action-transfer" on:click={() => transfer_open = true}>
          <div class="action-icon">
            <Icon d={mdiSwapHorizontal} />
          </div>
          {$locale.misc.Transfer_station}
        </button>
        <button class="action action-delete" on:click={() => delete_open = true}>
          <div class="action-icon">
            <Icon d={mdiTrashCanOutline} />
          </div>
          {$locale.pages["station/settings"].actions.delete_station}
        </button>
      </div>
    </div>
  </div>
</Page>

{#if transfer_open}
  <Dialog
    title={$locale.misc.station_transfer_title.replace("@station", data.station.name)}  
    width="500px"
    on_close={() => transfer_open = false}
  >
    {#if data.accounts.items.length > 2 && data.is_account_owner}
      <Formy action={transfer} let:submit>
        <form novalidate class="transfer-dialog-content" on:submit={submit}>
          <div class="transfer-message">
            {@html $locale.misc.station_transfer_message_html.replaceAll("@station", data.station.name)}
          </div>

          <div class="transfer-name-field">
            <TextField label={$locale.misc.Station_name} bind:value={transfer_name_check_value} />
            <Validator value={transfer_name_check_value} fn={_transfer_name_validate} /> 
          </div>
          
          <div class="transfer-account-selector">
            <TransferAccountSelector data={data} bind:selected={transfer_selected_account} />
          </div>

          <div class="transfer-dialog-btns">
            <button
              class="transfer-dialog-btn-cancel ripple-container"
              use:ripple
              on:click|preventDefault={() => (transfer_open = false)}
            >
              {$locale.misc.Cancel}
            </button>

            <button class="transfer-dialog-btn-transfer ripple-container" class:disabled={!transfer_name_is_match} use:ripple>
              <div class="transfer-dialog-btn-icon">
                <Icon d={mdiSwapHorizontal} />
              </div>
              {$locale.misc.Transfer_station}
            </button>
          </div>

        </form>
      </Formy>
    {:else if !data.is_account_owner}
      <div class="transfer-dialog-content">
        <div class="transfer-no-target-message">
          {@html $locale.misc.station_transfer_not_owner_message_html}
        </div>
        <button class="transfer-btn-no-target-ok ripple-container" use:ripple on:click|preventDefault={() => transfer_open = false}>
          {$locale.misc.OK}
        </button>
      </div>
    {:else}
      <div class="transfer-dialog-content">
        <div class="transfer-no-target-message">
          {$locale.misc.station_transfer_no_targets_message}
        </div>
        <button class="transfer-btn-no-target-ok ripple-container" use:ripple on:click|preventDefault={() => transfer_open = false}>
          {$locale.misc.OK}
        </button>
      </div>
    {/if}
  </Dialog>
{/if}

{#if delete_open}
  <Dialog
    width="500px"
    on_close={() => delete_open = false}
    title={$locale.pages["station/settings"].dialogs.delete_station.title.replace("@name", data.station.name)}
  >

    {#if data.is_account_owner}
      <Formy action={del} let:submit>
        <form novalidate on:submit={submit} class="delete-dialog">
          <div class="delete-dialog-content">
            {@html $locale.pages["station/settings"].dialogs.delete_station.message_html.replaceAll("@name", data.station.name)}
          </div>
          
          <div class="delete-dialog-field">
            <TextField
              label={$locale.pages["station/settings"].dialogs.delete_station.field_label}
              trim
              bind:value={delete_name_input_value}
            />
            <Validator value={delete_name_input_value} fn={_station_name_validate} /> 
          </div>

          <div class="delete-dialog-btns">
            <button
              class="delete-dialog-btn-cancel ripple-container"
              use:ripple
              on:click|preventDefault={() => (delete_open = false)}
            >
              {$locale.pages["station/settings"].dialogs.delete_station.cancel}
            </button>

            <button class="delete-dialog-btn-delete ripple-container" class:disabled={!delete_name_is_match} use:ripple>
              <div class="delete-dialog-btn-icon">
                <Icon d={mdiTrashCanOutline} />
              </div>
              {$locale.pages["station/settings"].dialogs.delete_station.submit}
            </button>
          </div>
        </form>
      </Formy>
    {:else}
      <div class="delete-no-owner-message">
        
        {@html $locale.misc.delete_station_not_owner_message_html}
        
        <div class="delete-dialog-btns">
          <button
            class="delete-dialog-btn-cancel ripple-container"
            use:ripple
            on:click={() => (delete_open = false)}
          >
            {@html $locale.misc.OK}
          </button>
        </div>

      </div>
    {/if}
  </Dialog>
{/if}

