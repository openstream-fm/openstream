<script lang="ts">
	import Email from "$lib/components/Form/Email.svelte";
	import Password from "$lib/components/Form/Password.svelte";
	import { ripple } from "$share/ripple";
  import { form } from "../transitions";
  import { action, _post } from "$share/net.client";
	import { goto } from "$app/navigation";
	import Formy from "$share/formy/Formy.svelte";
	import Validator from "$share/formy/Validator.svelte";
	import { _string } from "$share/formy/validate";
  import "$share/LoginDashboard/login-page.css";

  let email = "";
  let password = "";

  const login = action(async () => {
    const payload: Omit<import("$server/defs/api/auth/user/login/POST/Payload").Payload, "device_id"> = { email, password };
    await _post("/api/auth/user/login", payload);
    const target = decodeURIComponent(location.hash.replace(/^#/, "")) || "/";
    goto(target, { invalidateAll: true });
  })
</script>

<style>
  .password-box {
    display: flex;
    flex-direction: column;
    align-items: stretch;
  }
  .recover {
    padding: 0.25rem 0.25em;
    margin-top: 0.75rem;
    align-self: flex-start;
  }
</style>

<svelte:head>
  <title>Sign in</title>
</svelte:head>

<div class="login-page-box" in:form>
  <div class="login-page-title">Sign in</div>
  <Formy action={login} let:submit>
    <form novalidate class="login-page-fields" on:submit={submit}>
      <div class="login-page-field">
        <Email label="Email" bind:value={email} />
        <Validator value={email} fn={_string({ required: true })} />
      </div>
      <div class="login-page-field password-box">
        <Password label="Password" bind:value={password} />
        <Validator value={password} fn={_string({ required: true })} />
        <a class="na login-page-link recover" href="/recover">Forgot your password?</a>
      </div>
      <button type="submit" use:ripple class="ripple-container login-page-button">
        Sign in
      </button>
    </form>
  </Formy>

  <div class="login-page-switch-box">
    <span class="login-page-comment">New user?</span>
    <a class="na login-page-link sign-up" href="/plans">Sign up</a>
  </div>
  
</div>