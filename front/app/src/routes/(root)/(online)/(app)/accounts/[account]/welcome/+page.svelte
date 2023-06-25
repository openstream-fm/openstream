<script lang="ts">
  export let data: import("./$types").PageData;
  import Page from "$lib/components/Page.svelte";
	import { ripple } from "$share/ripple";
  import { locale } from "$lib/locale";
</script>

<style>
  h1 {
    font-weight: 400;
  }

  p {
    font-size: 1.1rem;
    margin-top: 1.25rem;
  }

  p + p {
    margin-top: 0.5rem;
  }

  .btn-out {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    margin-top: 1.5rem;
  }

  .btn {
    color: #fff;
    font-weight: 600;
    background: var(--blue);
    padding: 0.75rem 1rem;
    border-radius: 0.25rem;
    box-shadow: var(--some-shadow);
    display: flex;
  }
</style>

<svelte:head>
  <title>{$locale.misc.Welcome}</title>
</svelte:head>

<Page>
  
  <h1>
    {@html $locale.misc.account_welcome_title_html.replace("@name", data.user.first_name)}
  </h1>

  <p>{@html $locale.misc.account_welcome_message_1_html.replace("@brand", $locale.brand_name)}</p>
  <p>{@html $locale.misc.account_welcome_message_2_html}</p>
  <p>{@html $locale.misc.account_welcome_message_3_html}</p>
  
  <div class="btn-out">
    <a class="na btn ripple-container" href="/accounts/{data.account._id}/stations/create-station" use:ripple>
      {$locale.misc.Create_my_first_station}
    </a>
  </div>
</Page>