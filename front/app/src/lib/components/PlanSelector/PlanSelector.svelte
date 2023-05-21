<script lang="ts">
	export let plans: Plan[];
  export let target_url: (plan: Plan) => string;
  export let select_btn_label: string;
  export let show_trial: boolean = false;

  import type { Plan } from "$server/defs/db/Plan";

  import Icon from "$share/Icon.svelte";
	import { ripple } from "$share/ripple";

  import { mdiCheckBold as featureCheckIcon } from "@mdi/js";
	import FeatureI from "./PlanFeatureTip.svelte";
	import Color from "color";
	import PlanFeatureTip from "./PlanFeatureTip.svelte";
	import { locale } from "$lib/locale";

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
  .plans {
    display: flex;
    flex-direction: row;
    align-items: flex-start;
    justify-content: center;
    align-self: center;
    border-radius: 0.5rem;
    box-shadow: var(--some-shadow);
    overflow: hidden;
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

  .tip {
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
</style>

<div class="plans">
      {#each plans as plan (plan._id)}
        <div class="plan" style:--plan-color={plan_color(plan.color)} style:--plan-color-radial-center={plan_color_radial_center(plan.color)}>
          <div class="plan-name">{plan.display_name}</div>
          <div class="plan-price">
            <div class="plan-price-n">
              $ {plan.price}
            </div>
            <div class="plan-price-per">
              {$locale.plan_selector.price.per_month}
            </div>
          </div>

          <div class="plan-top-select">
            <a href={target_url(plan)} class="na plan-top-btn ripple-container" use:ripple>
              {select_btn_label}
            </a>
          </div>

          <div class="plan-features">
            <div class="feature">
              <span class="feature-n">
                {plan.limits.stations}
              </span>
              <span class="feature-label">
                {plan.limits.stations === 1 ? $locale.plan_selector.features.station : $locale.plan_selector.features.stations}
              </span>
              <span class="tip">
                {#if plan.limits.stations === 1}
                  <PlanFeatureTip text={$locale.plan_selector.tooltips.one_station} />
                {:else}
                  <PlanFeatureTip text={$locale.plan_selector.tooltips.n_stations.replace("@n", String(plan.limits.stations))} />
                {/if}
              </span>
            </div>

            <div class="feature">
              <span class="feature-n">
                {new Intl.NumberFormat().format(plan.limits.listeners)}
              </span>
              <span class="feature-label">
                {$locale.plan_selector.features.listeners}
              </span>
              <span class="tip">
                <PlanFeatureTip text={$locale.plan_selector.tooltips.listeners.replace("@n", String(plan.limits.listeners))} />
              </span>
            </div>

            <div class="feature">
              <span class="feature-n">
                {plan.limits.transfer / 1_000_000_000_000} TB
              </span>
              <span class="feature-label">
                {$locale.plan_selector.features.transfer}
              </span>
              <span class="tip">
                <FeatureI text={
                  $locale.plan_selector.tooltips.transfer
                    .replace("@tb", String(plan.limits.transfer / 1_000_000_000_000))
                    .replace("@hours",    new Intl.NumberFormat().format(aprox_listening_hours(plan.limits.transfer)))
                } />
              </span>
            </div>

            <div class="feature">
              <span class="feature-n">
                {plan.limits.storage / 1_000_000_000} GB
              </span>
              <span class="feature-label">
                {$locale.plan_selector.features.storage}
              </span>
              <span class="tip">
                <PlanFeatureTip text={
                  $locale.plan_selector.tooltips.storage.replace("@gb", String(plan.limits.storage / 1_000_000_000))
                } />
              </span>
            </div>

            <div class="feature">
              <span class="feature-n">
                {$locale.plan_selector.unlimited}
              </span>
              <span class="feature-label">
                {$locale.plan_selector.features.staff}
              </span>
              <span class="tip">
                <PlanFeatureTip text={$locale.plan_selector.tooltips.staff} />
              </span>
            </div>

            <div class="feature feature-check">
              <div class="feature-check-icon">
                <Icon d={featureCheckIcon} />
              </div>
              <div class="feature-label">
                {$locale.plan_selector.features.auto_dj}
              </div>
              <span class="tip">
                <PlanFeatureTip text={$locale.plan_selector.tooltips.auto_dj} />
              </span>
            </div>

            <div class="feature feature-check">
              <div class="feature-check-icon">
                <Icon d={featureCheckIcon} />
              </div>
              <div class="feature-label">
                {$locale.plan_selector.features.stats}
              </div>
              <span class="tip">
                <PlanFeatureTip text={$locale.plan_selector.tooltips.stats} />
              </span>
            </div>

            <div class="feature feature-check">
              <div class="feature-check-icon">
                <Icon d={featureCheckIcon} />
              </div>
              <div class="feature-label">
                {$locale.plan_selector.features.android_app}
              </div>
              <span class="tip">
                <PlanFeatureTip text={$locale.plan_selector.tooltips.android_app} />
              </span>
            </div>

            {#if show_trial}
              <div class="feature feature-check">
                <div class="feature-check-icon">
                  <Icon d={featureCheckIcon} />
                </div>
                <div class="feature-n">{$locale.plan_selector.trial["30_day"]}</div>
                <div class="feature-label">{$locale.plan_selector.trial.free_trial}</div>
                <span class="tip">
                  <PlanFeatureTip text={$locale.plan_selector.trial.tooltip} />
                </span>
              </div>
            {/if}
          </div>

          <div class="plan-bottom">
            <div class="plan-bottom-name">
              {plan.display_name}
            </div>
            <div class="plan-bottom-price">
              {$locale.plan_selector.price.$_n_per_month.replace("@n", String(plan.price))}
            </div>
            <div class="plan-bottom-select">
              <a href={target_url(plan)} class="na plan-bottom-btn ripple-container" use:ripple>
                {select_btn_label}
              </a> 
            </div>  
          </div>
        </div>
      {/each}
    </div>

