<script lang="ts">
	export let data: import("./$types").PageData;  
  import "$share/LoginDashboard/login-page.css";

  import { ripple } from "$share/ripple";
  import Email from "$lib/components/Form/Email.svelte";
	import Password from "$lib/components/Form/Password.svelte";
	import TextField from "$lib/components/Form/TextField.svelte";
	import { action, _post } from "$share/net.client";
	import { mdiAccountOutline, mdiPhoneOutline } from "@mdi/js";
	import Validator from "$share/formy/Validator.svelte";
	import { _confirmation_password, _email, _new_password, _phone, _string } from "$share/formy/validate";
	import Formy from "$share/formy/Formy.svelte";
  import "$share/LoginDashboard/login-page.css";
	import { goto } from "$app/navigation";
	import Page from "$lib/components/Page.svelte";
	import { form } from "../../../transitions";
	import Color from "color";

  let first_name = "";
  let last_name = "";
  let account_name = "";
  let phone = "";
  let email = "";
  let password = "";
  let confirm_password = "";

  const register = action(async () => {
    const payload: Omit<import("$api/auth/user/register/POST/Payload").Payload, "device_id"> = {
      plan_id: data.plan._id,
      first_name,
      last_name,
      account_name,
      phone,
      email,
      password,
    };
    
    const { account } = await _post<import("$api/auth/user/register/POST/Output").Output>("/api/auth/user/register", payload);
    await goto(`/accounts/${account._id}/welcome`, { invalidateAll: true });
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

  h2 {
    font-weight: 600;
    font-size: 1.5rem;
    text-align: center;
    margin: 4rem 0 3rem 0;
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
  <title>Sign up</title>
</svelte:head>

<Formy action={register} let:submit>
  <form novalidate on:submit={submit} class="login-page-box" in:form>
    <div class="login-page-title">Start your trial</div>

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

      <a href="/plans" class="na plan-back ripple-container" use:ripple>
        Back to plans and pricing
      </a>
    </div>

    <h2>Tell us about yourself</h2>

    <div class="login-page-fields">
      <div class="login-page-field">
        <TextField label="Your first name" trim icon={mdiAccountOutline} autocomplete="given-name" bind:value={first_name} />
        <Validator value={first_name} fn={_string({ required: true, maxlen: 50 })} />
      </div>
      <div class="login-page-field">
        <TextField label="Your last name" trim icon={mdiAccountOutline} autocomplete="family-name" bind:value={last_name} />
        <Validator value={last_name} fn={_string({ required: true, maxlen: 50 })} />
      </div>
      <div class="login-page-field">
        <TextField label="A name for your account" trim icon={mdiAccountOutline} autocomplete="off" bind:value={account_name} />
        <div class="org-explain">
          <!-- If you don't belong to an organization, just fill the field with a name for your new account  -->
          If you are creating an account for an organization, you can fill this field with the organization's name 
        </div>
        <Validator value={account_name} fn={_string({ required: true, maxlen: 50 })} />
      </div>
      <div class="login-page-field">
        <TextField type="tel" label="Your phone number" icon={mdiPhoneOutline} autocomplete="tel" bind:value={phone} />
        <Validator value={phone} fn={_phone({ required: true })} />
      </div>
      <div class="login-page-field">
        <Email label="Your email" bind:value={email} />
        <Validator value={email} fn={_email({ required: true })} /> 
      </div>
      <div class="login-page-field">
        <Password label="Your password" autocomplete="new-password" bind:value={password} />
        <Validator value={password} fn={_new_password({ minlen: 8, maxlen: 50 })} />
      </div>
      <div class="login-page-field">
        <Password label="Confirm your password" autocomplete="new-password" bind:value={confirm_password} />
        <Validator value={{ password, confirm_password }} fn={_confirmation_password()} />
      </div>
      <button type="submit" class="ripple-container login-page-button" use:ripple>
        Sign up
      </button>
    </div>
    <div class="login-page-switch-box">
      <span class="login-page-comment">Already have an account?</span>
      <a class="na login-page-link sign-in" href="/login">Sign in</a>
    </div>
  </form>
</Formy>