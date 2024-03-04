<script lang="ts">
  export let data: import("./$types").PageData;
  import Page from "$lib/components/Page.svelte";
	import PageTop from "$lib/components/PageMenu/PageTop.svelte";
	import { mdiShieldAccountOutline } from "@mdi/js";
</script>

<style>
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
    font-weight: var(--font-bold);
  }
</style>

<svelte:head>
  <title>{data.page_admin.first_name} {data.page_admin.last_name}</title>
</svelte:head>

<Page>
  <PageTop icon={mdiShieldAccountOutline}>
    <svelte:fragment slot="title">
      {data.page_admin.first_name} {data.page_admin.last_name}
    </svelte:fragment>

    <svelte:fragment slot="subtitle">
      {data.page_admin.email}
    </svelte:fragment>
  </PageTop>

  <div class="data">
    
    <div class="data-item">
      <div class="data-label">
        Id:
      </div>
      <div class="data-value">
        {data.page_admin._id}
      </div>
    </div>

    <div class="data-item">
      <div class="data-label">
        First name:
      </div>
      <div class="data-value">
        {data.page_admin.first_name}
      </div>
    </div>

    <div class="data-item">
      <div class="data-label">
        Last name:
      </div>
      <div class="data-value">
        {data.page_admin.last_name}
      </div>
    </div>


    <div class="data-item">
      <div class="data-label">
        Email:
      </div>
      <div class="data-value">
        {data.page_admin.email}
      </div>
    </div>

  </div>

</Page>