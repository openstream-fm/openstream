<script lang="ts">
	export let data: import("./$types").PageData;
  import PlanSelector from "$lib/components/PlanSelector/PlanSelector.svelte";
	import { fly } from "svelte/transition";
</script>

<style>
  .page {
    display: flex;
    flex-grow: 1;
    flex-direction: column;
    align-items: center;
  }

  .page-title {
    margin-top: 3rem;
    font-size: 2rem;
    font-weight: 600;
    text-align: center;
    padding: 0 2rem;
  }

  .plans {
    display: flex;
    flex-direction: column;
    align-items: center;
    margin-top: 5rem;
    margin-bottom: 6rem;
  }
</style>

<svelte:head>
  <title>Select a plan</title>
</svelte:head>

<div class="page" in:fly|local={{ x: -25, duration: 300 }}>
  <div class="page-title">Select a plan for your new account</div>
  <div class="plans">
    <PlanSelector plans={data.plans.items} target_url={plan => `/accounts/create-account/plan/${plan._id}`} select_btn_label="Select" />
  </div>
</div>