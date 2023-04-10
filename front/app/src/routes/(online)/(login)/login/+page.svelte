<script lang="ts">
	import Email from "$lib/components/Form/Email.svelte";
	import Password from "$lib/components/Form/Password.svelte";
	import { ripple } from "$lib/ripple";
  import { form } from "../transitions";
  import { action, _post } from "$share/net.client";
	import { goto, invalidate } from "$app/navigation";

  let email = "";
  let password = "";

  const login = action(async () => {
    const payload: Omit<import("$server/defs/api/auth/user/login/POST/Payload").Payload, "device_id"> = { email, password };
    await _post("/api/login", payload);
    await invalidate("user:me");
    goto("/");
  })
</script>

<style>
  .box {
    background: rgba(255,255,255,0.95);
    backdrop-filter: blur(2px);
    box-shadow: 0 4px 20px 0 rgb(0 0 0 / 12%);
    width: max(20rem, min(30rem, 55%));
    align-self: flex-end;
    margin: auto 0;
    padding-bottom: 5rem;
    border-radius: 0.5rem;
    display: flex;
    flex-direction: column;
  }

  .title {
    font-weight: 600;
    font-size: 2rem;
    text-align: center;
    margin: 3.5rem 0;
  }

  .fields {
    display: flex;
    flex-direction: column;
    gap: 2.5rem;
    width: min(78%, 25rem);
    align-self: center;
  }

  .field {
    align-self: stretch;
  }

  button {
    color: #fff;
    background: var(--blue);
    box-shadow: 0 4px 20px 0 rgb(0 0 0 / 16%);
    padding: 0.75rem;
    appearance: none;
    border: 0;
    margin: 0;
    outline: 0;
    cursor: pointer;
    user-select: none;
    align-self: flex-end;
    font-weight: 600;
  }

  .password-box {
    display: flex;
    flex-direction: column;
    align-items: stretch;
  }

  .link, .comment {
    font-size: 0.9em;
    transition: color 200ms ease;
  }

  .link {
    color: rgba(var(--blue-rgb), 0.6);
  }

  .recover {
    padding: 0.25rem 0.25em;
    margin-top: 0.75rem;
    align-self: flex-start;
  }

  .link:hover {
    color: var(--blue);
  }

  .new-box {
    align-self: center;
  }

  .comment {
    color: #999;
  }

  .new-box {
    margin-top: 3.5rem;
    margin-bottom: -2rem;
    display: flex;
    flex-direction: row;
    justify-content: center;
    gap: 0.5rem;
  }

  @media screen and (max-width: 600px) {
    .box {
      width: min(20rem, 90%);
      margin-inline: auto;
    }
  }
</style>

<svelte:head>
  <title>Sign in</title>
</svelte:head>

<form class="box" on:submit|preventDefault={login} in:form>
  <div class="title">Sign in</div>
  <div class="fields">
    <div class="field">
      <Email label="Email" bind:value={email} />
    </div>
    <div class="field password-box">
      <Password label="Password" bind:value={password} />
      <a class="na link recover" href="/recover">Forgot your password?</a>
    </div>
    <button use:ripple class="ripple-container">
      Sign in
    </button>
  </div>
  <div class="new-box">
    <span class="comment">New user?</span>
    <a class="na link sign-up" href="/register">Sign up</a>
  </div>
</form>