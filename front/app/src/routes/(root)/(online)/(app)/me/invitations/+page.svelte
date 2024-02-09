<script lang="ts">
	export let data: import('./$types').PageData;

	type Item = import('./$types').PageData['invitations']['items'][number];

	import { ripple } from '$share/ripple';
	import { slide } from 'svelte/transition';
	import { _delete, _post, action } from '$share/net.client';
	import { mdiTrashCanOutline } from '@mdi/js';
	import Icon from '$share/Icon.svelte';
	import Dialog from '$share/Dialog.svelte';
	import { _message, _error } from '$share/notify';
	import { invalidate } from '$app/navigation';
	import Page from '$lib/components/Page.svelte';
	import { invalidateAll } from '$lib/invalidate';
	
  import { locale } from "$lib/locale";

  const get_current = (...args: any) => {
    const map = new Map<string, Item>();
    // we only show the last invitation for each account
    for(const item of data.invitations.items.slice().sort((a, b) => b.created_at.localeCompare(a.created_at))) {
      if(item.deleted_at != null) continue;
      if(item.is_expired) continue;
      if(item.state !== "pending") continue;
      if(map.has(item.account_id)) continue;
      map.set(item.account_id, item);
    }

    // resort the invitations in created_at ascending order
    return [...map.values()].reverse();
  }
  $: current = get_current(data);

	let to_reject_item: Item | null = null;
  const reject = action(async () => {
		if (to_reject_item == null) return;
		const item = to_reject_item;
    const payload: import("$api/invitations/reject/POST/Payload").Payload = { invitation_id: item.id };
    
    await _post(`/api/invitations/reject`, payload);
		_message($locale.pages['me/invitations'].notifier.rejected);
    to_reject_item = null;
    
    await delete_siblings(item).catch(() => {});
      
    invalidate('api:invitations');
	});

  /** delete other invitations to same account */
  const delete_siblings = async (item: Item) => {
    const ids = new Set();
      for(const each of data.invitations.items) {
      if(each.deleted_at != null) continue;
      if(each.is_expired) continue;
      if(each.state !== "pending") continue;
      if(each.id === item.id) continue;
      if(each.account_id !== item.account_id) continue;
      ids.add(each.id);
    } 

    await Promise.all([...ids].map(async id => {
      await _delete<import("$api/invitations/[invitation]/DELETE/Output").Output>(`/api/invitations/${id}`);
    }))
  }

  let accepting = false;
  const accept = action(async (item: Item) => {
    if(accepting) return;
    accepting = true;
   
    try {
      const payload: import("$api/invitations/accept/POST/Payload").Payload = {
        invitation_id: item.id,
      }

      const { result } = await _post<import("$api/invitations/accept/POST/Output").Output>("/api/invitations/accept", payload);
      if(result !== "ok") {
        _error($locale.pages['me/invitations'].notifier.accept_error.replace("@error", result))
      } else {
        await delete_siblings(item).catch(() => {});
        _message($locale.pages["me/invitations"].notifier.accepted);
      }

      accepting = false;
      invalidateAll();
    } catch(e) {
      accepting = false;
      invalidateAll();
      throw e;
    } 
  })
</script>


<style>
	.page {
		display: flex;
		flex-grow: 1;
		flex-direction: column;
		align-items: center;
	}

	.page-title {
		margin-top: 2rem;
		font-size: 2rem;
		font-weight: var(--font-bold);
		text-align: center;
	}

	.list {
		margin-top: 3.5rem;
		margin-bottom: 4rem;
		width: min(90%, 600px);
		background: #fff;
		box-shadow: 0 0 15px 0 rgb(0 0 0 / 10%);
		border-radius: 0.5rem;
		display: flex;
		flex-direction: column;
		align-items: stretch;
		min-width: 0;
		padding: 0.25rem 0;
	}

  .empty-message {
    font-size: 1.2rem;
    margin-top: 3rem;
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

  .invitation {
    padding: 2rem 2rem;
  }

  .invitation-data {
    font-size: 1.2rem;
  }

  .invitation:nth-child(even) {
    background: rgba(0,0,0,0.025);
  }

  .invitation-actions {
    display: flex;
    flex-direction: row;
    justify-content: flex-end;
    gap: 1rem;
    margin-top: 2rem;
  }

  .invitation-action {
    padding: 0.75rem 1rem;
    border-radius: 0.25rem;
  }

  .invitation-action.reject {
    color: #555;
    transition: background-color 200ms ease;
  }

  .invitation-action.reject:hover {
    background: rgba(0,0,0,0.05);
  }

  .invitation-action.accept {
    background: var(--blue);
    color: #fff;
    font-weight: var(--font-bold);
    box-shadow: var(--some-shadow);
    border-radius: 0.25rem;
  }
</style>


<svelte:head>
	<title>{$locale.pages['me/invitations'].head.title}</title>
</svelte:head>

<Page compact>
	<div class="page">
		<div class="page-title">
			{$locale.pages['me/invitations'].head.title}
		</div>
		{#if current.length === 0}
      <div class="empty-message" transition:slide={{ duration: 400 }}>
        {$locale.pages['me/invitations'].no_items_message}
      </div>
    {:else}
      <div class="list" transition:slide={{ duration: 400 }}>
        {#each current as item (item.id)}
          <div class="invitation" data-invitation-id={item.id} transition:slide={{ duration: 400 }}>
            <div class="invitation-data">
              {#if item.user_sender}
                {@html 
                  $locale.pages['me/invitations'].item_message_with_sender_html
                    .replace("@sender", item.user_sender.first_name)
                    .replace("@account", item.account ? item.account.name : `#${item.account_id}`)
                }
              {:else}
                {@html
                  $locale.pages['me/invitations'].item_message_without_sender_html
                    .replace("@account", item.account ? item.account.name : `#${item.account_id}`)
                }
              {/if}
            </div>
            <div class="invitation-actions">
              <button class="invitation-action reject ripple-container" use:ripple on:click={() => to_reject_item = item}>
                {$locale.pages['me/invitations'].actions.reject}
              </button>
              <button class="invitation-action accept ripple-container" use:ripple on:click={() => accept(item)}>
                {$locale.pages['me/invitations'].actions.accept}
              </button>
            </div>
          </div>
        {/each}
      </div>
    {/if}
	</div>
</Page>

{#if to_reject_item != null}
	<Dialog
    title={$locale.pages['me/invitations'].dialogs.reject.title}
    width="400px"
    on_close={() => (to_reject_item = null)}
  >
		<div class="delete-dialog">
			<div class="delete-dialog-text">
				{$locale.pages['me/invitations'].dialogs.reject.message}
			</div>
			<div class="delete-dialog-btns">
				<button
					class="delete-dialog-btn-cancel ripple-container"
					use:ripple
					on:click={() => (to_reject_item = null)}
				>
					{$locale.pages['me/invitations'].dialogs.reject.cancel}
				</button>

				<button class="delete-dialog-btn-delete ripple-container" use:ripple on:click={reject}>
					<div class="delete-dialog-btn-icon">
						<Icon d={mdiTrashCanOutline} />
					</div>
					{$locale.pages['me/invitations'].dialogs.reject.reject}
				</button>
			</div>
		</div>
	</Dialog>
{/if}