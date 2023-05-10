<script lang="ts">
	export let data: import("./$types").PageData;  
  import "$share/LoginDashboard/login-page.css";

  import { ripple } from "$share/ripple";
  import TextField from "$lib/components/Form/TextField.svelte";
	import { action, _post } from "$share/net.client";
	import { mdiAccountOutline } from "@mdi/js";
	import Validator from "$share/formy/Validator.svelte";
	import { _string } from "$share/formy/validate";
	import Formy from "$share/formy/Formy.svelte";
  import { goto } from "$app/navigation";
	import Color from "color";
	import { fly } from "svelte/transition";
  import "$share/LoginDashboard/login-page.css";
	
  let account_name = "";
  let sending = false;

  const send = action(async () => {
    
    if(sending) return;
    sending = true;

    try {
      const payload: import("$server/defs/api/accounts/POST/Payload").Payload = {
        plan_id: data.plan._id,
        name: account_name,
      };
      
      const { account } = await _post<import("$server/defs/api/accounts/POST/Output").Output>("/api/accounts", payload);
      goto(`/accounts/${account._id}`, { invalidateAll: true });
      sending = true;
    } catch(e) {
      sending = false;
      throw e;
    }
  })

  let color: Color;
  try {
    color = new Color(data.plan.color);
  } catch(e) {
    color = new Color("#000")
  }

  const bg_color = color.alpha(0.1).toString();
</script>

<style>

  .page {
    display: flex;
    flex-direction: column;
    padding: 4rem 0 6rem 0;
  }

  h2 {
    font-weight: 600;
    font-size: 1.5rem;
    text-align: center;
    margin: 4rem 0 3rem 0;
    padding: 0 1.5rem;
  }

  .org-explain {
    color: #999;
    font-size: 0.8rem;
    margin: 0.5rem 0.25rem;
  }

  .plan {
    align-self: stretch;
    display: flex;
    flex-direction: column;
    align-items: center;
    margin: -1rem 0 0 0;
    background: var(--bg-color);
    padding: 2rem 0;
  }

  .plan-pretitle {
    font-weight: 600;
    font-size: 1.5rem;
  }

  .plan-title {
    color: var(--color);
    font-size: 1.5rem;
    font-weight: 900;
    margin-top: 1rem;
  }

  .plan-price {
    font-weight: 700;
    font-size: 1.1rem;
    margin-top: 0.75rem;
  }

  .plan-features {
    margin-top: 1rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    gap: 0.5rem;
  }

  .plan-feature > b {
    font-weight: 700;
  }

  .plan-back {
    margin-top: 1rem;
    font-size: 0.9rem;
    border-radius: 0.25rem;
    padding: 0.5rem 1rem;
    align-self: center;
    transition: background-color 200ms ease;
  }

  .plan-back:hover {
    background: rgba(0,0,0,0.05); 
  }
</style>

<svelte:head>
  <title>Create an account</title>
</svelte:head>

<div class="page" in:fly|local={{ y: -25, duration: 200 }}>
  <Formy action={send} let:submit>
    <form novalidate on:submit={submit} class="login-page-box">
      <div class="login-page-title">
        Create an account
      </div>

      <div class="plan" style:--bg-color={bg_color} style:--color={color.toString()}>
        <div class="plan-pretitle">Selected plan</div>
        <div class="plan-title">{data.plan.display_name}</div>
        <div class="plan-price">$ {data.plan.price} / month</div>
        <div class="plan-features">
          <div class="plan-feature">
            <b>{data.plan.limits.stations}</b> {data.plan.limits.stations === 1 ? "station" : "stations"}
          </div>
          <div class="plan-feature">
            <b>{new Intl.NumberFormat().format(data.plan.limits.listeners)}</b> Listeners
          </div>
          <div class="plan-feature">
            <b>{data.plan.limits.transfer / 1_000_000_000_000} TB</b> Bandwidth
          </div>
          <div class="plan-feature">
            <b>{data.plan.limits.storage / 1_000_000_000} GB</b> Storage
          </div>
        </div>

        <a href="/accounts/create-account" class="na plan-back ripple-container" use:ripple>
          Back to plans and pricing
        </a>
      </div>

      <h2>Tell us about the new account</h2>

      <div class="login-page-fields">
        <div class="login-page-field">
          <TextField label="A name for your new account" trim icon={mdiAccountOutline} autocomplete="off" bind:value={account_name} />
          <div class="org-explain">
            If you are creating an account for an organization you can fill this field with the organization's name 
          </div>
          <Validator value={account_name} fn={_string({ required: true, maxlen: 50 })} />
        </div>
        <button type="submit" class="ripple-container login-page-button" use:ripple>
          Create
        </button>
      </div>
    </form>
  </Formy>
</div>