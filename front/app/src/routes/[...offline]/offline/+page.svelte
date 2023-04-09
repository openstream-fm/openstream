<script>
	import Page from "$lib/components/Page.svelte";
	import { ripple } from "$share/ripple";
</script>

<style>
  .layout {
    display: flex; 
    flex-direction: column;
  }

  .top {
    padding: 2rem;
  }

  .logo {
    color: var(--red);
    font-weight: 600;
    font-size: 2rem;
  }

  .page {
    padding: 0 2rem 2rem 2rem;
  }

  p {
    margin-top: 1.5rem;
    font-size: 1.15rem;
  }

  .retry {
    padding: 0.75rem 1rem;
    background: var(--blue);
    color: #fff;
    box-shadow: var(--some-shadow);
    margin-top: 2rem;
    border-radius: 0.2rem;
  }
</style>

<div class="layout">
  <div class="top">
    <div class="logo">
      openstream
    </div>
  </div>
  <div class="page">
    <h1>Seems that you are offline</h1>    
    <p>You need internet access to use this app</p>
    <a class="retry ripple-container" use:ripple href="javascript:location.reload()">
      Retry
    </a>
  </div>
</div>