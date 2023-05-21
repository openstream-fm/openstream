<script lang="ts">
	export let data: import('./$types').PageData;

	type Item = import('./$types').PageData['devices']['items'][number];

	import Device from './device.svelte';
	import { ripple } from '$share/ripple';
	import { slide } from 'svelte/transition';
	import { _delete, action } from '$share/net.client';
	import { mdiTrashCanOutline } from '@mdi/js';
	import Icon from '$share/Icon.svelte';
	import Dialog from '$share/Dialog.svelte';
	import { _message } from '$share/notify';
	import { invalidate } from '$app/navigation';
	import Page from '$lib/components/Page.svelte';
	import { locale } from '$lib/locale';

	$: current = data.devices.items.find(item => item.is_current);

	$: devices = data.devices.items.filter(item => !item.is_current);

	let disconnect_item: Item | null = null;

	const disconnect = action(async () => {
		if (disconnect_item == null) return;
		await _delete(`/api/me/devices/${disconnect_item._id}`);
		_message($locale.pages['me.devices'].notifier.device_disconnected);
		disconnect_item = null;
		invalidate('resource:devices');
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
		font-weight: 600;
		text-align: center;
	}

	.note {
		width: min(80%, 400px);
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
	<title>{$locale.pages['me.devices'].head.title}</title>
</svelte:head>

<Page compact>
	<div class="page">
		<div class="page-title">
			{$locale.pages['me.devices'].title}
		</div>
		<div class="note">
			{$locale.pages['me.devices'].note}
		</div>
		<div class="list">
			{#if current != null}
				<div class="device-wrap" aria-current>
					<Device device={current} />
				</div>
			{/if}
			{#each devices as device (device._id)}
				<div class="device-wrap" transition:slide|local={{ duration: 400 }}>
					<Device {device} on_remove={() => (disconnect_item = device)} />
				</div>
			{/each}
		</div>
	</div>
</Page>

{#if disconnect_item != null}
	<Dialog title={$locale.pages['me.devices'].dialogs.disconnect.title} width="400px" on_close={() => (disconnect_item = null)}>
		<div class="delete-dialog">
			<div class="delete-dialog-text">
				{$locale.pages['me.devices'].dialogs.disconnect.message}
			</div>
			<div class="delete-dialog-btns">
				<button
					class="delete-dialog-btn-cancel ripple-container"
					use:ripple
					on:click={() => (disconnect_item = null)}
				>
					{$locale.pages['me.devices'].dialogs.disconnect.cancel}
				</button>

				<button class="delete-dialog-btn-delete ripple-container" use:ripple on:click={disconnect}>
					<div class="delete-dialog-btn-icon">
						<Icon d={mdiTrashCanOutline} />
					</div>
					{$locale.pages['me.devices'].dialogs.disconnect.submit}
				</button>
			</div>
		</div>
	</Dialog>
{/if}