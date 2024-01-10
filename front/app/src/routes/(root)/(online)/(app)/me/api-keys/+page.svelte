<script lang="ts">
	export let data: import('./$types').PageData;

	type Item = import('./$types').PageData['api_keys']['items'][number];

	import Key from './key.svelte';
	import { ripple } from '$share/ripple';
	import { slide } from 'svelte/transition';
	import { _delete, action } from '$share/net.client';
	import { mdiTrashCanOutline } from '@mdi/js';
	import Icon from '$share/Icon.svelte';
	import Dialog from '$share/Dialog.svelte';
	import { _message } from '$share/notify';
	import { invalidate } from '$app/navigation';
	import Page from '$lib/components/Page.svelte';
	// import { locale } from '$lib/locale';

	$: current = data.api_keys.items.find(item => item.is_current);
	
	$: keys = data.api_keys.items.filter(item => !item.is_current);

	let delete_item: Item | null = null;

	const delete_key = action(async () => {
		if (delete_item == null) return;
		await _delete(`/api/me/api-keys/${delete_item._id}`);
		// TODO: locale
		_message("API key deleted");
		delete_item = null;
		invalidate('resource:api-keys');
	});
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

	.note {
		width: min(90%, 400px);
		text-align: center;
		margin-top: 1rem;
		color: #333;
		font-size: 0.9rem;
	}

	.list {
		margin-top: 3.5rem;
		margin-bottom: 4rem;
		width: min(100%, 600px);
		background: #fff;
		box-shadow: 0 0 15px 0 rgb(0 0 0 / 10%);
		border-radius: 0.5rem;
		display: flex;
		flex-direction: column;
		align-items: stretch;
		min-width: 0;
		padding: 1rem 0;
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
		<!-- TODO: locale -->
		<!-- {$locale.pages['me/devices'].head.title}</title> -->
	<title>
		API keys
	</title>
</svelte:head>

<Page compact>
	<div class="page">
		<div class="page-title">
			<!-- TODO: locale -->
			<!-- {$locale.pages['me/devices'].title} -->
			API keys
		</div>
		<div class="note">
			<!-- TODO: locale -->
			<!-- {$locale.pages['me/devices'].note} -->
			Create API keys to access your openstream accounts programatically or to grant access to third party apps and services.
		</div>
		<div class="list">
			{#if current != null}
				<div class="key-wrap" aria-current>
					<Key key={current} />
				</div>
			{/if}
			{#each keys as key (key._id)}
				<div class="key-wrap" transition:slide|local={{ duration: 400 }}>
					<Key {key} on_remove={() => (delete_item = key)} />
				</div>
			{/each}
		</div>
	</div>
</Page>

{#if delete_item != null}
	<Dialog
		title={
			// TODO: locale
			// $locale.pages['me/devices'].dialogs.disconnect.title
			"Remove API key"
		}
		width="400px"
		on_close={() => (delete_item = null)}
	>
		<div class="delete-dialog">
			<div class="delete-dialog-text">
				<!-- TODO: locale -->
				<!-- {$locale.pages['me/devices'].dialogs.disconnect.message} -->
				This action is permanent
			</div>
			<div class="delete-dialog-btns">
				<button
					class="delete-dialog-btn-cancel ripple-container"
					use:ripple
					on:click={() => (delete_item = null)}
				>
					<!-- TODO: locale  -->
					<!-- {$locale.pages['me/devices'].dialogs.disconnect.cancel} -->
					Cancel
				</button>

				<button class="delete-dialog-btn-delete ripple-container" use:ripple on:click={delete_key}>
					<div class="delete-dialog-btn-icon">
						<Icon d={mdiTrashCanOutline} />
					</div>
					<!-- TODO: locale -->
					<!-- {$locale.pages['me/devices'].dialogs.disconnect.submit} -->
					Delete
				</button>
			</div>
		</div>
	</Dialog>
{/if}