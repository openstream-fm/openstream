<script lang="ts">
	import Email from "$share/Form/Email.svelte";
	import Password from "$share/Form/Password.svelte";
	import { ripple } from "$share/ripple";
  import { form } from "../transitions";
  import { action, _post } from "$share/net.client";
	import { goto } from "$app/navigation";
	import Formy from "$share/formy/Formy.svelte";
	import Validator from "$share/formy/Validator.svelte";
	import { _string } from "$share/formy/validate";
  import "$share/LoginDashboard/login-page.css";
	import { invalidate_siblings } from "$lib/invalidate";

  let email = "";
  let password = "";

  const login = action(async () => {
    const payload: Omit<import("$api/auth/admin/login/POST/Payload").Payload, "device_id"> = { email, password };
    await _post("/api/auth/admin/login", payload);
    const target = location.hash.replace(/^#/, "") || "/";
    goto(target, { invalidateAll: true });
    invalidate_siblings();
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

<div class="login-page-box" in:form|global>
  <div class="login-page-title">Sign in</div>
  <Formy action={login} let:submit>
    <form novalidate class="login-page-fields" on:submit={submit}>
      <div class="login-page-field">
        <Email label="Email" autocomplete="username" bind:value={email} />
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
</div>