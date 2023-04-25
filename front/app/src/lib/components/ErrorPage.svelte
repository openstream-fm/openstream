<script lang="ts">
	import { page } from '$app/stores';
	export let error: App.Error = $page.error!;
	import Page from '$lib/components/Page.svelte';
	import { ripple } from '$share/ripple';
</script>

<svelte:head>
	<title>{error?.status || 500} {error?.message || 'Error'}</title>
</svelte:head>

<Page>
	<div class="page">
		<h1>{error.status ?? 500}</h1>

		<h2>{error.message ?? 'An error ocurred'}</h2>

		<div class="code">
			{error.code ?? 'CLIENT_PAGE_MISSING_CODE'}
		</div>

		<div class="btns">
			{#if error.status !== 404}
				<button on:click={() => location.reload()} use:ripple class="ripple-container btn retry">
					Retry
				</button>
			{/if}
			<a href="/" use:ripple class="ripple-container btn home"> Take me to home </a>
		</div>
	</div>
</Page>

<style>
	.page {
		padding: 0 1rem;
		display: flex;
		flex-direction: column;
		align-items: flex-start;
	}

	h1 {
		text-align: left;
		font-size: 7rem;
		filter: drop-shadow(var(--red) 0.025em 0.025em 0);
	}

	h2 {
		text-align: left;
		margin-top: 1rem;
		font-size: 2.5rem;
	}

	.code {
		margin-top: 2rem;
		padding: 0.75rem;
		border-radius: 0.25rem;
		border: rgba(0, 0, 0, 0.065) 1px solid;
		background: rgba(0, 0, 0, 0.1);
		align-self: flex-start;
	}

	.btns {
		margin-top: 1rem;
		display: flex;
		flex-direction: row;
		gap: 2rem;
		align-items: center;
		justify-content: center;
	}

	.btn {
		display: block;
		margin: 1rem 0;
		padding: 0.75rem 1rem;
		color: #fff;
		background-color: var(--blue);
		border: 0;
		appearance: none;
		text-decoration: none;
		font-weight: 500;
		box-shadow: 0 4px 8px 0 rgb(0 0 0 / 12%), 0 2px 4px 0 rgb(0 0 0 / 8%);
		border-radius: 0.25rem;
	}
</style>
