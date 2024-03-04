<script lang="ts">
	export let error: App.Error | null = $page.error;

  // @ts-ignore
  import bg from "$share/img/login-bg.jpg?w=1920&format=webp&withoutEnlargement";
  // @ts-ignore
  import logo from "$share/img/logo-trans-128.png?w=36&format=webp";
	// import { locale } from '$lib/locale';

  import { page } from '$app/stores';
	import Page from '$lib/components/Page.svelte';
	import { ripple } from '$share/ripple';

	let online = true;

	$: status = Number(error?.status) || 500;
	$: message = String(error?.message || "") || "An error ocurred handling this request";
	$: code = String(error?.code || "") || "CLIENT_PAGE_MISSING_CODE";
	$: title = online ? `${status} ${message}` : "Offline";
</script>



<style>
  .login-dashboard {
    flex: 1;
    display: flex;
    align-self: stretch;
    justify-self: stretch;
    flex-direction: column;
    /* padding-inline: min(5%, max(12rem, 10%)); */
    padding-block: 8rem;
    position: relative;
    --field-container-bg: rgba(0,0,0,0);
    overflow: hidden;
    background: #fff;
  }

  .bg {
    position: absolute;
    z-index: var(--z-login-bg);
    pointer-events: none;
    inset-block-start: 0;
    inset-inline-start: 0;
    width: 100vw;
    height: 100vh;
    background-size: cover;
    background-position: center;
  }

  .logo {
    position: absolute;
    z-index: var(--z-login-front);
    display: flex;
    flex-direction: row;
    align-items: center;
    inset-block-start: 2rem;
    inset-inline-start: 2rem;
  }

  .logo-icon {
    flex: none;
    width: 2.25rem;
    height: 2.25rem;
    margin-inline-end: 0.5rem;
    background-size: contain;
    background-position: center;
    background-repeat: no-repeat;
  }

  .logo-text {
    font-size: 1.75rem;
    font-weight: var(--font-bold);
    color: #000;
  }

  .content {
    position: relative;
    z-index: var(--z-login-front);
    display: flex;
    flex-direction: column;
    flex-grow: 1;
  }

  @media screen and (max-width: 600px) {
    .login-dashboard {
      padding-inline: 0;
    }
  }

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

<div class="login-dashboard">
  <div class="bg" style:background-image="url({ bg })" />
  <div class="logo">
    <div class="logo-icon" style:background-image="url({ logo })" />
    <div class="logo-text">openstream</div>
  </div>
  <div class="content">
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
              {#if status !== 404}
                <button on:click={() => location.reload()} use:ripple class="ripple-container btn retry">
                  Retry
                </button>
              {/if}
              {#if $page.url.pathname !== "/"}
                <a href="/" use:ripple class="ripple-container btn home">
                  Take me to home
                </a>
              {/if}
            </div>
          </div>
        {:else}
          <div class="offline">	
            <h1>Offline</h1>    
              
            <p>You need internet access to use Openstream Studio</p>
            
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
   </div>
</div>