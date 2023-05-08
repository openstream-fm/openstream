<script lang="ts">
	import { goto } from "$app/navigation";
	import Email from "$lib/components/Form/Email.svelte";
	import Password from "$lib/components/Form/Password.svelte";
	import TextField from "$lib/components/Form/TextField.svelte";
	import { ripple } from "$share/ripple";
	import { action, _post } from "$share/net.client";
	import { mdiAccountOutline, mdiPhoneOutline } from "@mdi/js";
	import { form } from "../transitions";
	import Validator from "$share/formy/Validator.svelte";
	import { _confirmation_password, _email, _new_password, _phone, _string } from "$share/formy/validate";
	import Formy from "$share/formy/Formy.svelte";
  import "$share/LoginDashboard/login-page.css";

  let first_name = "";
  let last_name = "";
  let account_name = "";
  let phone = "";
  let email = "";
  let password = "";
  let confirm_password = "";

  const register = action(async () => {
    
    if(password !== confirm_password) {
      throw new Error("Confirmation password does not match");
    }

    const payload: Omit<import("$server/defs/api/auth/user/register/POST/Payload").Payload, "device_id"> = {
      // TODO: add plan id logic
      plan_id: "",
      first_name,
      last_name,
      account_name,
      phone,
      email,
      password,
    };
    
    const { account } = await _post<import("$server/defs/api/auth/user/register/POST/Output").Output>("/api/auth/user/register", payload);
    await goto(`/accounts/${account._id}/welcome`, { invalidateAll: true });
  })
</script>

<svelte:head>
  <title>Sign up</title>
</svelte:head>

<Formy action={register} let:submit>
  <form novalidate on:submit={submit} class="login-page-box" in:form>
    <div class="login-page-title">Sign up</div>
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
        <TextField label="Your organization's name" trim icon={mdiAccountOutline} autocomplete="off" bind:value={account_name} />
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

      <button use:ripple class="ripple-container login-page-button">
        Sign up
      </button>
    </div>
    <div class="login-page-switch-box">
      <span class="login-page-comment">Already have an station?</span>
      <a class="na login-page-link sign-in" href="/login">Sign in</a>
    </div>
  </form>
</Formy>