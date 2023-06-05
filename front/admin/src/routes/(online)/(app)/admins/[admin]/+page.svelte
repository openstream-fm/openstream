<script lang="ts">
  export let data: import("./$types").PageData;
  import Page from "$lib/components/Page.svelte";
	import { ripple } from "$share/ripple";
</script>

<style>
  p {
    color: #444;
    font-size: 0.9rem;
    margin-inline-start: 0.25rem;
  }

  .data {
    background: #fff;
    border-radius: 0.5rem;
    box-shadow: var(--some-shadow);
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
    margin-top: 1.5rem;
  }

  .data-item {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-start;
    gap: 0.4rem;
    font-size: 1.1rem;
  }

  .data-label {
    color: #333;
  }

  .data-value {
    font-weight: 700;
  }
</style>

<svelte:head>
  <title>{data.admin.first_name} {data.admin.last_name}</title>
</svelte:head>

<Page>
  <h1>{data.admin.first_name} {data.admin.last_name}</h1>
  <p>Admin</p>

  <div class="data">
    
    <div class="data-item">
      <div class="data-label">
        Id:
      </div>
      <div class="data-value">
        {data.admin._id}
      </div>
    </div>

    <div class="data-item">
      <div class="data-label">
        First name:
      </div>
      <div class="data-value">
        {data.admin.first_name}
      </div>
    </div>

    <div class="data-item">
      <div class="data-label">
        Last name:
      </div>
      <div class="data-value">
        {data.admin.last_name}
      </div>
    </div>


    <div class="data-item">
      <div class="data-label">
        Email:
      </div>
      <div class="data-value">
        {data.admin.email}
      </div>
    </div>

  </div>

</Page>