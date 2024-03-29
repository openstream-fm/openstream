<script lang="ts">
	export let plans: Plan[];
  export let target_url: (plan: Plan) => string;
  export let select_btn_label: string;
  export let show_trial: boolean = false;

  import type { Unwrap, GET } from "$lib/client";
  // import type { Plan } from "$server/defs/db/Plan";
  type Plan = Unwrap<Awaited<ReturnType<typeof GET<"/plans/{plan}">>>>["plan"]

  import Icon from "$share/Icon.svelte";
	import { ripple } from "$share/ripple";

  import { mdiCheckBold as featureCheckIcon } from "@mdi/js";
	import FeatureI from "./PlanFeatureTip.svelte";
	import Color from "color";
	import PlanFeatureTip from "./PlanFeatureTip.svelte";
	import { locale, lang } from "$lib/locale";

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

  const format_price = (price: number): string => {
    return new Intl.NumberFormat($lang, {
      maximumFractionDigits: 0,
      minimumFractionDigits: 0,
      currency: "USD",
      style: "currency",
    }).format(price);
  }

  const format_number = (n: number): string => {
    return new Intl.NumberFormat($lang, {
      maximumFractionDigits: 0,
      minimumFractionDigits: 0,   
    }).format(n);
  }

  const format_listening_hours = (n: number): string => {
    return new Intl.NumberFormat($lang, {
      maximumFractionDigits: 0,
      minimumFractionDigits: 0,
      /// 3 zeros
      maximumSignificantDigits: Math.max(2, String(Math.round(n)).length - 3),
    }).format(n);
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
    max-width: 95%;
    padding: 0 1rem;
    background: #fff;
  }

  .plan {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    flex: 1;
    min-width: 0;
    padding: 1rem 0;
    background: #fff;
  }

  .plan-name {
    font-size: 2rem;
    padding: 0 2rem;
    font-weight: var(--font-bold);
    margin-top: 1.25rem;
    color: var(--plan-color);
    text-shadow: #fff 0 0 3px;
  }

  .plan-price {
    margin-top: 1.5rem;
  }

  .plan-price-n {
    font-size: 2rem;
    font-weight: var(--font-bold);
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
    font-weight: var(--font-bold);
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
    padding: 0 2rem;
    min-width: 10rem;
    max-width: 100%;
  }

  .feature {
    display: flex;
    flex: 1;
    flex-direction: row;
    align-items: center;
    padding: 0.75rem 0.5rem;
    max-width: 100%;
  }

  .feature-text {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .feature-ellipsis {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 100%;
  }

  .feature-n {
    display: inline;
    font-weight: var(--font-bold);
  }

  .tip {
    flex: none;
    display: flex;
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
    font-weight: var(--font-bold);
    font-size: 1.2rem;
  }

  .plan-bottom-price {
    font-size: 1.1rem;
    margin-top: 0.75rem;
    font-weight: var(--font-bold);
  }

  .plan-bottom-btn {
    margin-top: 1rem;
    padding: 0.75rem 1rem;
    align-self: center;
  }

  @media screen and (max-width: 1250px) {
    .plans {
      display: grid;
      grid-template-columns: repeat(2, 1fr);
      gap: 2rem;
      padding: 0 2%;
      background: transparent;
      box-shadow: none;
      overflow: visible;
    }

    .plan {
      border-radius: 0.5rem;
      box-shadow: var(--some-shadow);
    }
    
    .plan {
      width: 20rem;
    }

    .plan-features {
      padding-inline: 1.5rem;
    }
  }

  @media screen and (max-width: 700px) {
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
              {format_price(plan.price)}
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
              <div class="feature-text">
                <span class="feature-ellipsis">
                  <span class="feature-n">
                    {format_number(plan.limits.stations)}
                  </span>
                  <span class="feature-label">
                    {plan.limits.stations === 1 ? $locale.plan_selector.features.station : $locale.plan_selector.features.stations}
                  </span>
                </span>
              </div>
              <span class="tip">
                {#if plan.limits.stations === 1}
                  <PlanFeatureTip text={$locale.plan_selector.tooltips.one_station} />
                {:else}
                  <PlanFeatureTip text={$locale.plan_selector.tooltips.n_stations.replace("@n", format_number(plan.limits.stations))} />
                {/if}
              </span>
            </div>

            <div class="feature">
              <div class="feature-text">
                <span class="feature-ellipsis">
                  <span class="feature-n">
                    {format_number(plan.limits.listeners)}
                  </span>
                  <span class="feature-label">
                    {$locale.plan_selector.features.listeners}
                  </span>
                </span>
              </div>
              <span class="tip">
                <PlanFeatureTip text={$locale.plan_selector.tooltips.listeners.replace("@n", format_number(plan.limits.listeners))} />
              </span>
            </div>

            <div class="feature">
              <div class="feature-text">
                <span class="feature-ellipsis">
                  <span class="feature-n">
                    {plan.limits.transfer / 1_000_000_000_000} TB
                  </span>
                  <span class="feature-label">
                    {$locale.plan_selector.features.transfer}
                  </span>
                </span>
              </div>
              <span class="tip">
                <FeatureI text={
                  $locale.plan_selector.tooltips.transfer
                    .replace("@tb", String(plan.limits.transfer / 1_000_000_000_000))
                    .replace("@hours", format_number(aprox_listening_hours(plan.limits.transfer)))
                } />
              </span>
            </div>

            <div class="feature">
              <div class="feature-text">
                <span class="feature-ellipsis">
                  <span class="feature-n">
                    {plan.limits.storage / 1_000_000_000} GB
                  </span>
                  <span class="feature-label">
                    {$locale.plan_selector.features.storage}
                  </span>
                </span>
              </div>
              <span class="tip">
                <PlanFeatureTip text={
                  $locale.plan_selector.tooltips.storage.replace("@gb", String(plan.limits.storage / 1_000_000_000))
                } />
              </span>
            </div>

            <div class="feature">
              <div class="feature-text">
                <span class="feature-ellipsis">
                  <span class="feature-n">
                    {$locale.plan_selector.unlimited}
                  </span>
                  <span class="feature-label">
                    {$locale.plan_selector.features.staff}
                  </span>
                </span>
              </div>
              <span class="tip">
                <PlanFeatureTip text={$locale.plan_selector.tooltips.staff} />
              </span>
            </div>

            <div class="feature feature-check">
              <div class="feature-check-icon">
                <Icon d={featureCheckIcon} />
              </div>
              <div class="feature-text">
                <span class="feature-ellipsis">
                  <span class="feature-label">
                    {$locale.plan_selector.features.auto_dj}
                  </span>
                </span>
              </div>
              <span class="tip">
                <PlanFeatureTip text={$locale.plan_selector.tooltips.auto_dj} />
              </span>
            </div>

            <div class="feature feature-check">
              <div class="feature-check-icon">
                <Icon d={featureCheckIcon} />
              </div>
              <div class="feature-text">
                <span class="feature-ellipsis">
                  <span class="feature-label">
                    {$locale.plan_selector.features.stats}
                  </span>
                </span>
              </div>
              <span class="tip">
                <PlanFeatureTip text={$locale.plan_selector.tooltips.stats} />
              </span>
            </div>

            <div class="feature feature-check">
              <div class="feature-check-icon">
                <Icon d={featureCheckIcon} />
              </div>
              <div class="feature-text">
                <span class="feature-ellipsis">
                  <span class="feature-label">
                    {$locale.plan_selector.features.android_app}
                  </span>
                </span>
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
                <div class="feature-text">
                  <span class="feature-ellipsis">
                    <span class="feature-n">{$locale.plan_selector.trial["30_day"]}</span>
                    <span class="feature-label">{$locale.plan_selector.trial.free_trial}</span>
                  </span>
                </div>
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
              {$locale.plan_selector.price.n_per_month.replace("@n", format_price(plan.price))}
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

