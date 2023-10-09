<script lang="ts">
	import { page } from '$app/stores';
	export let error: App.Error = $page.error!;
	import Page from '$lib/components/Page.svelte';
	import { ripple } from '$share/ripple';

	let online = true;

	$: status = error.status || 500;
	$: message = error?.message || "An error ocurred";
	$: code = error?.code || "CLIENT_PAGE_MISSING_CODE";
	$: title = online ? `${status} ${message}` : "Offline";
</script>

<style>
	.page {
		padding: 0 1rem;
		display: flex;
		flex-direction: column;
		align-items: flex-start;
	}

	.online {
		display: flex;
		flex-direction: column;
	}

	.online h1 {
		text-align: start;
		font-size: 7rem;
		filter: drop-shadow(var(--red) 0.025em 0.025em 0);
	}

	.online h2 {
		text-align: start;
		margin-top: 1rem;
		font-size: 2.5rem;
	}

	.online .code {
		margin-top: 2rem;
		padding: 0.75rem;
		border-radius: 0.25rem;
		border: rgba(0, 0, 0, 0.065) 1px solid;
		background: rgba(0, 0, 0, 0.1);
		align-self: flex-start;
	}

	.online .btns {
		margin-top: 1rem;
		display: flex;
		flex-direction: row;
		gap: 2rem;
		align-items: center;
		justify-content: flex-start;
	}

	.offline h1 {
    font-size: 2.5rem;
    font-weight: var(--font-bold);
  }

  .offline p {
    font-size: 1.4rem;
    margin-top: 1.5rem;
  }

  .offline .btns {
    display: flex;
    flex-direction: row;
    justify-content: flex-start;
    margin: 2rem 0;
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


<svelte:window bind:online />

<svelte:head>
	<title>{title}</title>
</svelte:head>

<Page>
	<div class="page">
		
		{#if online}
			<div class="online">
				<h1>{status}</h1>

				<h2>{message}</h2>

				<div class="code">
					{code}
				</div>

				<div class="btns">
					{#if error.status !== 404}
						<button on:click={() => location.reload()} use:ripple class="ripple-container btn retry">
							Retry
						</button>
					{/if}
					{#if $page.url.pathname !== "/"}
						<a href="/" use:ripple class="ripple-container btn home">Take me to home</a>
					{/if}
				</div>
			</div>
		{:else}
			<div class="offline">	
				<h1>Seems that you are offline</h1>    
					
				<p>You need internet access to use openstream studio</p>
				
				<div class="btns">
					<!-- svelte-ignore a11y-invalid-attribute -->
					<a class="na btn ripple-container" use:ripple href="javascript:location.reload()">
						Retry
					</a>
				</div>
			</div>
		{/if}
	</div>
</Page>