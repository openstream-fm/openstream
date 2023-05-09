<script lang="ts">
	import Icon from "$share/Icon.svelte";
	import { ripple } from "$share/ripple";

  import { mdiCheckBold as featureCheckIcon } from "@mdi/js";

  export let data: import("./$types").PageData;
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
    margin-top: 3rem;
  }

  .plan {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    flex: 1;
    padding: 1rem 0;
    background: 
      /* linear-gradient(to bottom, var(--plan-bg), var(--plan-bg)), */
      linear-gradient(to bottom, #fff, #fff);
      /* var(--plan-bg), #fff; */
  }

  .plan:nth-child(1) {
    --plan-color: rgb(255, 133, 27);
    --plan-color-lite: rgb(255, 180, 113);
    --plan-bg: rgba(255, 133, 27, 0.15);
  }

  .plan:nth-child(2) {
    --plan-color: rgb(0, 116, 217);
    --plan-color-lite: rgb(83, 151, 211);
    --plan-bg: rgba(0, 116, 217, 0.15);
  }

  .plan:nth-child(3) {
    --plan-color: rgb(177, 13, 201);
    --plan-color-lite: rgb(225, 27, 255);
    --plan-bg: rgba(176, 13, 201, 0.15);
  } 

  .plan:nth-child(4) {
    --plan-color: rgb(255, 65, 54);
    --plan-color-lite: rgb(255, 118, 111);
    --plan-bg: rgba(255, 65, 54, 0.2);
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
  }

  .plan-top-btn, .plan-bottom-btn {
    background:
      radial-gradient(circle at center, var(--plan-color-lite) , var(--plan-color));
    color: #fff;
    padding: 1rem 2.5rem;
    border-radius: 5rem;
    font-weight: 600;
    box-shadow: var(--some-shadow);
    white-space: nowrap;
    --ripple-color: #fff;
    --ripple-opacity: 0.2;
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
  }

  .feature-n {
    font-weight: 700;
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
    margin-top: 0.75rem;
    padding: 0.75rem 1rem;
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

<div class="page">
  <div class="top">
    <h1>Going live in 3... 2... 1...</h1>
    <h2>Start your radio station in less than 60 seconds.</h2>
    <h3>You won't be billed until the end of your trial. And you can cancel anytime.</h3>
  </div>
  
  <div class="plans">
    {#each data.plans.items as plan (plan._id)}
      <div class="plan">
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
          <button class="plan-top-btn ripple-container" use:ripple>
            Start Trial
          </button>
        </div>

        <div class="plan-features">
          <div class="feature">
            <span class="feature-n">
              {plan.limits.stations}
            </span>
            <span class="feature-label">
              {plan.limits.stations === 1 ? "Station" :  "Stations"}
            </span>
          </div>

          <div class="feature">
            <span class="feature-n">
              {new Intl.NumberFormat().format(plan.limits.listeners)}
            </span>
            <span class="feature-label">
              {plan.limits.listeners === 1 ? "Listener" :  "Listeners"}
            </span>
          </div>

          <div class="feature">
            <span class="feature-n">
              {plan.limits.transfer / 1_000_000_000_000} TB
            </span>
            <span class="feature-label">
              Bandwidth
            </span>
          </div>

          <div class="feature">
            <span class="feature-n">
              {plan.limits.storage / 1_000_000_000} GB
            </span>
            <span class="feature-label">
              Storage
            </span>
          </div>

          <div class="feature">
            <span class="feature-n">
              Unlimited
            </span>
            <span class="feature-label">
              DJs
            </span>
          </div>

          <div class="feature feature-check">
            <div class="feature-check-icon">
              <Icon d={featureCheckIcon} />
            </div>
            <div class="feature-label">
              Auto DJ
            </div>
          </div>

          <div class="feature feature-check">
            <div class="feature-check-icon">
              <Icon d={featureCheckIcon} />
            </div>
            <div class="feature-label">
              Advanced Stats
            </div>
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
            <button class="plan-bottom-btn ripple-container" use:ripple>
              Start Trial
            </button> 
          </div>  
        </div>
      </div>
    {/each}
  </div>
</div>
