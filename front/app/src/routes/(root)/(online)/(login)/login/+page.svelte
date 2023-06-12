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
	import { locale } from "$lib/locale";
	import { invalidate_siblings } from "$lib/invalidate";

  import { page } from "$app/stores";

  let email = $page.url.searchParams.get("email")?.trim() || "";
  let password = "";

  const login = action(async () => {
    const payload: Omit<import("$api/auth/user/login/POST/Payload").Payload, "device_id"> = { email, password };
    await _post("/api/auth/user/login", payload);
    const target = location.hash.replace(/^#/, "") || "/";
    goto(target, { invalidateAll: true }).then(() => {
      invalidate_siblings();
    });
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
  <title>{$locale.pages.login.head.title}</title>
</svelte:head>

<div class="login-page-box" in:form>
  <div class="login-page-title">{$locale.pages.login.title}</div>
  <Formy action={login} let:submit>
    <form novalidate class="login-page-fields" on:submit={submit}>
      <div class="login-page-field">
        <Email label={$locale.pages.login.fields.email} bind:value={email} />
        <Validator value={email} fn={_string({ required: true })} />
      </div>
      <div class="login-page-field password-box">
        <Password label={$locale.pages.login.fields.password} bind:value={password} />
        <Validator value={password} fn={_string({ required: true })} />
        <a class="na login-page-link recover" href="/recover">
          {$locale.pages.login.links.forgot}
        </a>
      </div>
      <button type="submit" use:ripple class="ripple-container login-page-button">
        {$locale.pages.login.submit}
      </button>
    </form>
  </Formy>

  <div class="login-page-switch-box">
    <span class="login-page-comment">{$locale.pages.login.links.new_user}</span>
    <a class="na login-page-link sign-up" href="/plans">{$locale.pages.login.links.sign_up}</a>
  </div>
  
</div>