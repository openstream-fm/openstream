<script lang="ts">
  export let data: import("./$types").PageData;
  import Page from "$lib/components/Page.svelte";
	import { ripple } from "$share/ripple";
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
  <title>Welcome</title>
</svelte:head>

<Page>
  <h1>Hello <b>{data.user.first_name}</b></h1>
  <p>Welcome to <b>openstream</b></p>
  <p>You are now the owner of your newly created account</p>
  <p>To start broadcasting right away, add your first station to your account</p>
  <div class="btn-out">
    <a class="na btn ripple-container" href="/accounts/{data.account._id}/stations/create-station" use:ripple>
      Create my first station
    </a>
  </div>
</Page>