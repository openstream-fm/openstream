<script lang="ts">
	export let data: import('./$types').PageData;

	type Item = import('./$types').PageData['api_keys']['items'][number];

	import Key from './key.svelte';
	import { ripple } from '$share/ripple';
	import { slide } from 'svelte/transition';
	import { _delete, _post, action } from '$share/net.client';
	import { mdiContentCopy, mdiTrashCanOutline } from '@mdi/js';
	import Icon from '$share/Icon.svelte';
	import Dialog from '$share/Dialog.svelte';
	import { _message } from '$share/notify';
	import { invalidate } from '$app/navigation';
	import Page from '$lib/components/Page.svelte';
	import Validator from '$share/formy/Validator.svelte';
	import { _string } from '$share/formy/validate';
	import TextField from '$share/Form/TextField.svelte';
	import Formy from '$share/formy/Formy.svelte';
	import Password from '$share/Form/Password.svelte';
	import Email from '$share/Form/Email.svelte';
	import copy from 'copy-to-clipboard';
	import { locale } from '$lib/locale';
	import { DELETE, POST, unwrap } from '$lib/client';

	$: current = data.api_keys.items.find(item => item.is_current);
	
	$: keys = data.api_keys.items.filter(item => !item.is_current);

	let delete_item: Item | null = null;

	const delete_key = action(async () => {
		if (delete_item == null) return;
		unwrap(await DELETE("/me/api-keys/{id}", { params: { path: { id: delete_item._id } } }));
		_message($locale.misc.api_keys.API_key_deleted);
		delete_item = null;
		invalidate('resource:api-keys');
	});

	let create_open = false;
	let create_title = "";
	let create_password = "";

	let api_key_show_to_save: string | null = null;

	const create = action(async () => {
		
		const { token, media_key, api_key } = unwrap(
			await POST("/me/api-keys", {
				body: {
					title: create_title.trim(),
					password: create_password
				}
			})
		)

		invalidate('resource:api-keys');
		create_title = "";
		create_password = "";
	
		api_key_show_to_save = token;
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
		color: #444;
	}

	.delete-dialog-btn-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		margin-inline: -0.25rem 0.5rem;
		font-size: 1.2rem;
	}

	/* .empty {
		margin-top: 2rem;
	} */

	.create-btn {
		color: #fff;
		background-color: var(--blue);
		padding: 0.75rem 1rem;
		border-radius: 0.25rem;
		margin-top: 2rem;
		box-shadow: var(--some-shadow);
	}

	.create-dialog-btns {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: flex-end;
		margin-top: 2rem;
	}

	.create-dialog-btn-cancel {
		padding: 0.5rem 0.75rem;
		display: flex;
		flex-direction: row;
		align-items: center;
		border-radius: 0.25rem;
		transition: background-color 150ms ease;
		margin-inline-end: 1rem;
		color: #444;
	}

	.create-dialog-btn {
		color: #fff;
		background-color: var(--blue);
		padding: 0.75rem 1rem;
		border-radius: 0.25rem;
		box-shadow: var(--some-shadow);
		align-self: flex-end;
	}

	.create-dialog-user-explain {
		margin-top: 2.5rem;
		font-size: 0.9rem;
		color: #666; 
	}

	.create-dialog-user-field {
		margin-top: 1.5rem;
	}

	.title-explain {
		font-size: 0.9rem;
		color: #666;
		margin: 0.5rem 0.25rem 0 0.25rem;
	}

	.copy-ok-btn-out {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: flex-end;
	}

	.copy-ok-btn {
		padding: 0.75rem 1rem;
		color: #444;
	}

	.copy-dialog-field {
		margin: 1.5rem 0;
	}
</style>


<svelte:head>
	<title>
		{$locale.misc.api_keys.API_Keys}
	</title>
</svelte:head>

<Page compact>
	<div class="page">
		<div class="page-title">
			{$locale.misc.api_keys.API_Keys}
		</div>
		<div class="note">
			{$locale.misc.api_keys.API_keys_page_message}
		</div>

		{#if current == null && keys.length === 0}
			<!-- -- -->
		{:else}
		
			<div class="list" transition:slide={{ duration: 400 }}>
				{#if current != null}
					<div class="key-wrap" aria-current transition:slide={{ duration: 400 }}>
						<Key key={current} />
					</div>
				{/if}
				{#each keys as key (key._id)}
					<div class="key-wrap" transition:slide={{ duration: 400 }}>
						<Key {key} on_remove={() => (delete_item = key)} />
					</div>
				{/each}
			</div>
		{/if}

		<div class="create-out">
			<button class="create-btn ripple-container" use:ripple on:click={() => create_open = true}>
				{$locale.misc.api_keys.Create_a_new_API_key}
			</button>
		</div>
	</div>
</Page>

{#if delete_item != null}
	<Dialog
		title={$locale.misc.api_keys.Remove_API_key}
		width="400px"
		on_close={() => (delete_item = null)}
	>
		<div class="delete-dialog">
			<div class="delete-dialog-text">
				{$locale.misc.This_action_is_permanent}
			</div>
			<div class="delete-dialog-btns">
				<button
					class="delete-dialog-btn-cancel ripple-container"
					use:ripple
					on:click={() => (delete_item = null)}
				>
					{$locale.misc.Cancel}
				</button>

				<button class="delete-dialog-btn-delete ripple-container" use:ripple on:click={delete_key}>
					<div class="delete-dialog-btn-icon">
						<Icon d={mdiTrashCanOutline} />
					</div>
					{$locale.misc.Delete}
				</button>
			</div>
		</div>
	</Dialog>
{/if}

{#if create_open}
  <Dialog
		width="400px"
		on_close={() => {
			api_key_show_to_save = null;
			create_open = false
		}}
		title={$locale.misc.api_keys.Create_a_new_API_key}
	>
		{#if api_key_show_to_save == null}
			<Formy action={create} let:submit>
	      <form novalidate class="create-dialog" on:submit={submit} transition:slide={{ duration: 300 }}>
					<div class="create-dialog-fields">
						<div class="create-dialog-field">
							<TextField
								label={$locale.misc.api_keys.API_key_title}
								maxlength={
									// TODO: validate and const
									100
								}
								trim
								bind:value={create_title}
							/>
							<Validator
								value={create_title}
								fn={_string({
									required: true,
									maxlen: 100
								})}
							/>
							<div class="title-explain">
								{$locale.misc.api_keys.API_key_title_explain}
							</div>
						</div>

						<div class="create-dialog-user">
							<div class="create-dialog-user-explain">
								{$locale.misc.Type_password_proceed}
							</div>
							<div class="create-dialog-user-field">
								<Email label={$locale.misc.Your_email} readonly value={data.user.email} />
							</div>
							<div class="create-dialog-user-field">
								<Password label={$locale.misc.Your_password} autocomplete="off" bind:value={create_password} />
								<Validator
									value={create_password}
									fn={_string({
										required: true,
									})}
								/>
							</div>
						</div>
					</div>
					<div class="create-dialog-btns">
						<button
							class="create-dialog-btn-cancel ripple-container"
							use:ripple
							type="button"
							on:click|preventDefault={() => (create_open = false)}
						>
							{$locale.misc.Cancel}
						</button>
			
						<button type="submit" class="create-dialog-btn ripple-container" use:ripple>
							{$locale.misc.Create}
						</button>
					</div>
				</form>
    	</Formy>
		{:else}
			<div class="copy-dialog" transition:slide={{ duration: 300 }}>
				<div class="copy-dialog-text">
					{$locale.misc.api_keys.Copy_contents_message}
				</div>
				<div class="copy-dialog-field">
					<TextField
						label={$locale.misc.api_keys.API_key_contents}
						readonly
						value={api_key_show_to_save}
						btn={{
							label: $locale.misc.Copy,
							icon: mdiContentCopy,
							action: () => {
								copy(api_key_show_to_save ?? "");
								_message($locale.misc.Copied_to_clipboard);
							}
						}}
					/>
				</div>

				<div class="copy-ok-btn-out">
					<button class="copy-ok-btn ripple-container" use:ripple on:click={() => {
						api_key_show_to_save = null;
						create_open = false
					}}>
						{$locale.misc.Done}
					</button>
				</div>
			</div>
		{/if}
  </Dialog>
{/if}