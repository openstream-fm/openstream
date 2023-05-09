<script lang="ts">
  export let data: import("./$types").PageData;
  import Page from "$lib/components/Page.svelte";
	import { clone, diff, equals } from "$server/util/collections";
	import Formy from "$share/formy/Formy.svelte";
	import { _delete, _patch, _post, action } from "$share/net.client";
	import { _message } from "$share/notify";
	import { ripple } from "$share/ripple";
  import { invalidateAll } from "$app/navigation";
  import PlanForm from "../PlanForm.svelte";
	import Icon from "$share/Icon.svelte";
	import { mdiTrashCanOutline } from "@mdi/js";
  import { goto } from "$app/navigation";
  import Dialog from "$share/Dialog.svelte"; 

  let db = {
    identifier: data.plan.identifier,
    display_name: data.plan.display_name,
    color: data.plan.color,
    price: data.plan.price as number | null,
    stations: data.plan.limits.stations as number | null,
    listeners: data.plan.limits.listeners as number | null,
    transfer: data.plan.limits.transfer as number | null,
    storage: data.plan.limits.storage as number | null,
    is_user_selectable: data.plan.is_user_selectable,
  };

  let current = clone(db);

  const save = action(async () => {
    
    if(equals(db, current)) {
      _message("Nothing to save");
      return;
    }

    const {
      identifier,
      display_name,
      price,
      color,
      stations,
      listeners,
      transfer,
      storage,
      is_user_selectable,
    } = diff(db, current);

    if(price === null) throw new Error("Price is required");
    if(stations === null) throw new Error("Stations is required");
    if(listeners === null) throw new Error("Listeners is required");
    if(transfer === null) throw new Error("Transfer is required");
    if(storage === null) throw new Error("Storage is required");
    
    const payload: import("$server/defs/api/plans/[plan]/PATCH/Payload").Payload = {
      identifier,
      display_name,
      color,
      price,
      stations,
      listeners,
      storage,
      transfer,
      is_user_selectable,
    }

    const plan = await _patch(`/api/plans/${data.plan._id}`, payload);
    db = clone(current);
    _message("Plan updated");
    invalidateAll();
  });


  let delete_open = false;
  const del = action(async () => {
    await _delete(`/api/plans/${data.plan._id}`);
    goto("/plans", { invalidateAll: true });
  })
</script>

<style>
  .page {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding-bottom: 5rem;
  }

  h1 {
    text-align: center;
    margin-top: 2rem;
    font-weight: 600;
  }

  h2 {
    text-align: center;
    font-weight: 600;
    margin-top: 7.5rem;
  }

  .box, .actions {
    width: min(500px, 80%);
    background: #fff;
    box-shadow: var(--some-shadow);
    border-radius: 0.5rem;
    padding: 2rem 1rem;
    margin-top: 4rem;
  }

  .send-out {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
  }

  .send {
    padding: 0.75rem;
    font-weight: 600;
    color: #fff;
    background: var(--blue);
    border-radius: 0.25rem;
    box-shadow: var(--some-shadow);
    margin: 1rem 1rem 0 0;
  }

  .actions {
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
</style>


<svelte:head>
  <title>Plan {data.plan.display_name}</title>
</svelte:head>

<Page>
  <div class="page">
    
    <h1>Plan {data.plan.display_name}</h1>
    
    <Formy action={save} let:submit>
      <form novalidate class="box" on:submit={submit}>
        <div class="fields">
          <PlanForm bind:current />
        </div>
        <div class="send-out">
          <button class="send ripple-container" use:ripple type="submit">
            Save
          </button>
        </div>
      </form>
    </Formy>

    <h2>Actions</h2>

    <div class="actions">
      <button class="action action-delete" on:click={() => delete_open = true}>
        <div class="action-icon">
          <Icon d={mdiTrashCanOutline} />
        </div>
        Delete plan
      </button>
    </div>
  </div>

</Page>

{#if delete_open}
  <Dialog title="Delete plan {data.plan.display_name}" width="500px" on_close={() => delete_open = false}>
    <div class="delete-dialog">
      <div class="delete-dialog-text">This action is permanent.</div>
      <div class="delete-dialog-btns">
        <button
          class="delete-dialog-btn-cancel ripple-container"
          use:ripple
          on:click={() => (delete_open = false)}
        >
          Cancel
        </button>

        <button class="delete-dialog-btn-delete ripple-container" use:ripple on:click={del}>
          <div class="delete-dialog-btn-icon">
            <Icon d={mdiTrashCanOutline} />
          </div>
          Delete
        </button>
      </div>
    </div>
  </Dialog>
{/if}