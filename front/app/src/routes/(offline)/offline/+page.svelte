<script lang="ts">
  import LoginDashboard from "$share/LoginDashboard/LoginDashboard.svelte";
	import Page from "$lib/components/Page.svelte";
	import { ripple } from "$share/ripple";
	import { locale } from "$lib/locale";
</script>

<style>
  h1 {
    font-size: 2.5rem;
    font-weight: 600;
  }

  p {
    font-size: 1.4rem;
    margin-top: 1.5rem;
  }

  .btns {
    display: flex;
    flex-direction: row;
    justify-content: flex-start;
    margin: 2rem 0;
  }

  .retry {
    display: block;
    padding: 0.75rem 1rem;
    color: #fff;
    background-color: var(--blue);
    border: 0;
    appearance: none;
    text-decoration: none;
    font-weight: 500;
    box-shadow: 0 4px 8px #0000001f, 0 2px 4px #00000014;
    border-radius: 0.25rem;
  }
</style>


<LoginDashboard>
  <Page>
    <h1>{$locale.pages.error.offline.title}</h1>    
    
    <p>{$locale.pages.error.offline.text.replace("@app_name", $locale.app_name)}</p>
    
    <div class="btns">
      <!-- svelte-ignore a11y-invalid-attribute -->
      <a class="na retry ripple-container" use:ripple href="javascript:location.reload()">
        {$locale.pages.error.retry}
      </a>
    </div>
  </Page>
</LoginDashboard>