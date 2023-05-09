<script lang="ts">
	import Icon from "$share/Icon.svelte";
	import { ripple } from "$share/ripple";

  import { mdiCheckBold as featureCheckIcon } from "@mdi/js";
	import FeatureI from "./feature-i.svelte";
	import Page from "$lib/components/Page.svelte";
	import Color from "color";

  export let data: import("./$types").PageData;

  const aprox_listening_hours = (bytes: number): number => {
    return Math.round(bytes / 16000 / 60 / 60 / 1_000) * 1_000;
  }

  const plan_color = (color: string): string => {
    try {
      return new Color(color).toString();
    } catch(e) {
      return "rgb(0, 116, 217)"
    }
  }

  const plan_color_radial_center = (color: string): string => {
    try {
      return new Color(color).lighten(0.25).toString();
    } catch(e) {
      return "rgb(83, 151, 211)"
    }
  }
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
    display: flex;
    flex-direction: row;
    align-items: flex-start;
    justify-content: center;
    align-self: center;
    border-radius: 0.5rem;
    box-shadow: var(--some-shadow);
    overflow: hidden;
    margin-top: 3rem;
  }

  .plan {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    flex: 1;
    padding: 1rem 0;
    background: #fff;
  }

  .plan-name {
    font-size: 2rem;
    padding: 0 2rem;
    font-weight: 700;
    margin-top: 1.25rem;
    color: var(--plan-color);
    text-shadow: #fff 0 0 3px;
  }

  .plan-price {
    margin-top: 1.5rem;
  }

  .plan-price-n {
    font-size: 2rem;
    font-weight: 700;
    margin-inline-start: -1rem;
  }

  .plan-price-per {
    margin-top: 0.25rem;
    font-size: 0.9rem;
  }
  
  .plan-top-select {
    margin-top: 1.75rem;
    display: flex;
    flex-direction: column;
  }

  .plan-top-btn, .plan-bottom-btn {
    background:
      radial-gradient(circle at center, var(--plan-color-radial-center) , var(--plan-color));
    color: #fff;
    padding: 1rem 2.5rem;
    border-radius: 5rem;
    font-weight: 600;
    box-shadow: var(--some-shadow);
    white-space: nowrap;
    --ripple-color: #fff;
    --ripple-opacity: 0.2;
    display: flex;
    align-self: center;
  }


  .plan-features {
    display: flex;
    flex-direction: column;
    text-align: center;
    align-items: center;
    font-size: 1.1rem;
    margin-top: 1.25rem;
    padding: 0 3rem;
  }

  .feature {
    padding: 0.75rem 0.5rem;
    white-space: nowrap;
    display: flex;
    flex-direction: row;
  }

  .feature-n {
    font-weight: 700;
    margin-inline-end: 0.4rem;
  }

  .feature-i {
    color: #bbb;
    font-size: 1rem;
    margin-inline-start: 0.4rem;
    align-self: center;
  }
  
  .feature-check {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
  }

  .feature-check-icon {
    color: var(--green);
    display: flex;
    margin-inline-end: 0.5rem;
    margin-inline-start: -1.5rem;
  }

  .plan-bottom {
    border-top: 1px solid #aaa;
    margin-top: 1.25rem;
    margin-inline: 1.5rem;
    padding-top: 1.25rem;
    padding-bottom: 1rem;
    align-self: stretch;
  }

  .plan-bottom-select {
    display: flex;
    flex-direction: column;
  }

  .plan-bottom-name {
    font-weight: 700;
    font-size: 1.2rem;
  }

  .plan-bottom-price {
    font-size: 1.1rem;
    margin-top: 0.75rem;
    font-weight: 600;
  }

  .plan-bottom-btn {
    margin-top: 1rem;
    padding: 0.75rem 1rem;
    align-self: center;
  }

  @media screen and (max-width: 1050px) {
    .plans {
      display: grid;
      grid-template-columns: repeat(2, 1fr);
      gap: 3rem;
      padding: 0 5%;
      align-items: space-evenly;
      background: transparent;
      box-shadow: none;
      overflow: visible;
    }

    .plan {
      border-radius: 0.5rem;
      box-shadow: var(--some-shadow);
    }

    .plan-features {
      padding-inline: 4rem;
    }
  }

  @media screen and (max-width: 560px) {
    .plans {
      grid-template-columns: 1fr;
    }
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

<Page compact>
  <div class="page">
    <div class="top">
      <h1>Going live in 3... 2... 1...</h1>
      <h2>Start your radio station in less than 60 seconds.</h2>
      <h3>You won't be billed until the end of your trial. And you can cancel anytime.</h3>
    </div>
    
    <div class="plans">
      {#each data.plans.items as plan (plan._id)}
        <div class="plan" style:--plan-color={plan_color(plan.color)} style:--plan-color-radial-center={plan_color_radial_center(plan.color)}>
          <div class="plan-name">{plan.display_name}</div>
          <div class="plan-price">
            <div class="plan-price-n">
              $ {plan.price}
            </div>
            <div class="plan-price-per">
              per month
            </div>
          </div>

          <div class="plan-top-select">
            <a href="/register/plan/{plan._id}" class="na plan-top-btn ripple-container" use:ripple>
              Start Trial
            </a>
          </div>

          <div class="plan-features">
            <div class="feature">
              <span class="feature-n">
                {plan.limits.stations}
              </span>
              <span class="feature-label">
                {plan.limits.stations === 1 ? "Station" :  "Stations"}
              </span>
              <span class="feature-i">
                {#if plan.limits.stations === 1}
                  <FeatureI text={`You can only create one station with this plan`} />
                {:else}
                  <FeatureI text={`Up to ${plan.limits.stations} different stations`} />
                {/if}
              </span>
            </div>

            <div class="feature">
              <span class="feature-n">
                {new Intl.NumberFormat().format(plan.limits.listeners)}
              </span>
              <span class="feature-label">
                Listeners
              </span>
              <span class="feature-i">
                <FeatureI text={`Up to ${plan.limits.listeners} concurrent listeners`} />
              </span>
            </div>

            <div class="feature">
              <span class="feature-n">
                {plan.limits.transfer / 1_000_000_000_000} TB
              </span>
              <span class="feature-label">
                Bandwidth
              </span>
              <span class="feature-i">
                <FeatureI text={`
                  ${
                    plan.limits.transfer / 1_000_000_000_000
                  } TB of monthly transfer will give you around ${
                    new Intl.NumberFormat().format(aprox_listening_hours(plan.limits.transfer))
                  } listening hours`} />
              </span>
            </div>

            <div class="feature">
              <span class="feature-n">
                {plan.limits.storage / 1_000_000_000} GB
              </span>
              <span class="feature-label">
                Storage
              </span>
              <span class="feature-i">
                <FeatureI text={`${plan.limits.storage / 1_000_000_000} GB of storage for music or old episodes`} />
              </span>
            </div>

            <div class="feature">
              <span class="feature-n">
                Unlimited
              </span>
              <span class="feature-label">
                DJs
              </span>
              <span class="feature-i">
                <FeatureI text={`Add all the staff users that you want`} />
              </span>
            </div>

            <div class="feature feature-check">
              <div class="feature-check-icon">
                <Icon d={featureCheckIcon} />
              </div>
              <div class="feature-label">
                Auto DJ
              </div>
              <span class="feature-i">
                <FeatureI text={`Broadcast from a playlist when you're not online`} />
              </span>
            </div>

            <div class="feature feature-check">
              <div class="feature-check-icon">
                <Icon d={featureCheckIcon} />
              </div>
              <div class="feature-label">
                Advanced Stats
              </div>
              <span class="feature-i">
                <FeatureI text={`Advanced live and historical stats, see who's listening your stations`} />
              </span>
            </div>

            <div class="feature feature-check">
              <div class="feature-check-icon">
                <Icon d={featureCheckIcon} />
              </div>
              <div class="feature-label">
                Android App
              </div>
              <span class="feature-i">
                <FeatureI text={`An Android application branded to your stations and available worldwide through Google Play`} />
              </span>
            </div>
          </div>

          <div class="plan-bottom">
            <div class="plan-bottom-name">
              {plan.display_name}
            </div>
            <div class="plan-bottom-price">
              $ {plan.price} / month  
            </div>
            <div class="plan-bottom-select">
              <a href="/register/plan/{plan._id}" class="na plan-bottom-btn ripple-container" use:ripple>
                Start Trial
              </a> 
            </div>  
          </div>
        </div>
      {/each}
    </div>
  </div>
</Page>
