<script lang="ts">
  export let data: import("./$types").PageData;
  import PlanSelector from '$lib/components/PlanSelector/PlanSelector.svelte';
	import Page from "$lib/components/Page.svelte";
	import { locale } from '$lib/locale';
</script>

<style>

  .page {
    display: flex;
    flex-direction: column;
  }

  .top {
    padding: 0 2rem;
  }

  h1 {
    font-weight: 900;
    font-size: 2.75rem;
    text-align: center;
  }

  h2 {
    margin-top: 1rem;
  }

  h3 {
    margin-top: 0.25rem;
  }

  h2, h3 {
    font-weight: 600;
    font-size: 1.25rem;
    text-align: center;
  }

  .plans {
    margin-top: 4rem;
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  @media screen and (max-width: 800px) {
    h1 {
      font-size: 2.5rem;
    }

    h2, h3 {
      font-size: 1.1rem;
    }
  }

  @media screen and (max-width: 550px) {
    h1 {
      font-size: 2rem;
    }

    h2, h3 {
      font-size: 1rem;
    }
  }

  @media screen and (max-width: 420px) {
    h1 {
      font-size: 1.65rem;
    }

    h2, h3 {
      font-size: 1rem;
    }
  }

</style>

<svelte:head>
  <title>{$locale.pages.plans.head.title}</title>
</svelte:head>

<Page compact>
  <div class="page">
    <div class="top">
      <h1>{$locale.pages.plans.title_1}</h1>
      <h2>{$locale.pages.plans.title_2}</h2>
      <h3>{$locale.pages.plans.title_3}</h3>
    </div>
    
    <div class="plans">
      <PlanSelector 
        plans={data.plans.items}
        show_trial
        target_url={plan => `/register/plan/${plan.slug}`}
        select_btn_label={$locale.pages.plans.plan_selector.select_btn_label}
      />
    </div>
  </div>
</Page>
