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

	let create_open = false;
	let create_title = "";
	let create_password = "";

	let api_key_show_to_save: string | null = null;

	const create = action(async () => {
		const payload: import("$api/me/api-keys/POST/Payload").Payload = {
			title: create_title.trim(),
			password: create_password,
		};
		
		const {
			token,
			media_key,
			api_key
		} = await _post<import("$api/me/api-keys/POST/Output").Output>(`/api/me/api-keys`, payload);

		// TODO: locale
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

		{#if current == null && keys.length === 0}
			<!--
				<div class="empty" transition:slide|local={{ duration: 400 }}>
					<-- TODO: locale 
					You didn't create any API key yet.
				</div>
			-->
		{:else}
		
			<div class="list" transition:slide|local={{ duration: 400 }}>
				{#if current != null}
					<div class="key-wrap" aria-current transition:slide|local={{ duration: 400 }}>
						<Key key={current} />
					</div>
				{/if}
				{#each keys as key (key._id)}
					<div class="key-wrap" transition:slide|local={{ duration: 400 }}>
						<Key {key} on_remove={() => (delete_item = key)} />
					</div>
				{/each}
			</div>
		{/if}

		<div class="create-out">
			<button class="create-btn ripple-container" use:ripple on:click={() => create_open = true}>
				Create a new API key
				<!-- TODO: locale -->
			</button>
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

{#if create_open}
  <Dialog
		width="400px"
		on_close={() => {
			api_key_show_to_save = null;
			create_open = false
		}}
		title={
			// TODO: locale
			"Create a new API key"
		}
	>
		{#if api_key_show_to_save == null}
			<Formy action={create} let:submit>
	      <form novalidate class="create-dialog" on:submit={submit} transition:slide|local={{ duration: 300 }}>
					<div class="create-dialog-fields">
						<div class="create-dialog-field">
							<TextField
								label={
									// $locale.pages["account/dashboard"].edit.dialog.field_label
									"API key title"
								}
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
								<!-- TODO: locale -->
								The title will be used by you to identify this API key.
							</div>
						</div>

						<div class="create-dialog-user">
							<div class="create-dialog-user-explain">
								<!-- TODO: locale -->
								Type your password to proceed with this action.
							</div>
							<div class="create-dialog-user-field">
								<!-- TODO: locale -->
								<Email label="You" readonly value={data.user.email} />
							</div>
							<div class="create-dialog-user-field">
								<!-- TODO: locale -->
								<Password label="Your password" autocomplete="off" bind:value={create_password} />
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
							<!-- TODO: locale  -->
							<!-- {$locale.pages['me/devices'].dialogs.disconnect.cancel} -->
							Cancel
						</button>
			
						<button type="submit" class="create-dialog-btn ripple-container" use:ripple>
							<!-- TODO: locale -->
							<!-- {$locale.pages["account/dashboard"].edit.dialog.save} -->
							Create
						</button>
					</div>
				</form>
    	</Formy>
		{:else}
			<div class="copy-dialog" transition:slide|local={{ duration: 300 }}>
				<div class="copy-dialog-text">
					<!-- TODO: locale -->
					Copy the API key contents. This code will not be shown again.
				</div>
				<div class="copy-dialog-field">
					<!-- TODO: locale -->
					<TextField
						label="API key contents"
						readonly
						value={api_key_show_to_save}
						btn={{
							// TODO: locale
							label: "Copy",
							icon: mdiContentCopy,
							action: () => {
								copy(api_key_show_to_save ?? "");
								// TODO: locale
								_message("Copied to clipboard");
							}
						}}
					/>
				</div>

				<div class="copy-ok-btn-out">
					<button class="copy-ok-btn ripple-container" use:ripple on:click={() => {
						api_key_show_to_save = null;
						create_open = false
					}}>
						<!-- TODO: locale -->
						Done
					</button>
				</div>
			</div>
		{/if}
  </Dialog>
{/if}