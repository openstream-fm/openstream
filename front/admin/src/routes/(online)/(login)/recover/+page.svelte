<script lang="ts">
	import Email from "$share/Form/Email.svelte";
	import Formy from "$share/formy/Formy.svelte";
	import Validator from "$share/formy/Validator.svelte";
	import { _email } from "$share/formy/validate";
	import { _post, action } from "$share/net.client";
	import { ripple } from "$share/ripple";
	import { scale, slide } from "svelte/transition";
	import { form } from "../transitions";
	import Icon from "$share/Icon.svelte";
	import { mdiCheck } from "@mdi/js";
	import { _progress } from "$share/notify";
	import CircularProgress from "$share/CircularProgress.svelte";
  import "$share/LoginDashboard/login-page.css";

  let email = "";
  let sent_to: string | null = null;
  let sending = false;

  const send = action(async () => {
    if(sending) return;
    sending = true;
    try {
      const payload: import("$api/auth/user/recover/POST/Payload").Payload = { email };
      await _post(`/api/auth/admin/recover`, payload);
      sent_to = email;
      email = "";
      sending = false;
    } catch(e) {
      sending = false;
      throw e;
    }
  })
</script>

<style>
  .sent {
    background: #E8F4E9;
    color: #24531E;
    width: min(78%, 25rem);
    margin: 0 auto 2rem auto;
    display: flex;
    flex-direction: row;
    align-items: center;
    padding: 0.75rem;
    border-top: currentColor 1px solid;
    border-radius: 0 0 0.5rem 0.5rem;
  }

  .comment {
    margin-top: 0.5rem;
    margin-inline-start: 0.25rem;
  }
  .sent-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    flex: none;
    font-size: 1.25rem;
    margin-inline-end: 0.75rem;
  }

  .sent-message > b {
    word-break: break-all;
  }

  .field-box {
    display: flex;
    flex-direction: column;
    align-items: stretch;
  }

  .back {
    font-size: 0.9em;
    color: rgba(var(--blue-rgb), 0.6);
    padding: 0.25rem 0.25em;
    margin-top: 0.75rem;
    align-self: flex-start;
    transition: color 200ms ease;
  }

  .back:hover {
    color: var(--blue);
  }
</style>

<svelte:head>
  <title>Recover your account</title>
</svelte:head>


<div class="login-page-box" in:form>
  <div class="login-page-title">Recover</div>
  {#if sent_to != null}
    <div class="sent" in:slide|local={{ duration: 300 }}>
      <div class="sent-icon">
        <Icon d={mdiCheck} />
      </div>
      <div class="sent-message">
        We sent you an email to <b>{sent_to}</b> with further instructions 
      </div>
    </div>
  {/if}
  <Formy action={send} let:submit>
    <form novalidate class="login-page-fields" on:submit={submit}>
      <div class="login-page-field field-box">
        <Email label="Email" autocomplete="username" bind:value={email} />
        <Validator value={email} fn={_email({ required: true })} />
        <span class="comment login-page-comment">We'll send you an e-mail for you to recover access</span>
        <a class="na back" href="/login">Back to login</a>
      </div>
      <button use:ripple class="ripple-container login-page-button" class:sending>
        {#if sending}
          <div class="login-page-btn-sending-progress" transition:scale|local={{ duration: 300 }}>
            <CircularProgress />
          </div>
        {/if}
        Send
      </button>
    </form>
  </Formy>
</div>