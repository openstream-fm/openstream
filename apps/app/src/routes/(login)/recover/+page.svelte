<script>
	import Email from "$lib/components/Form/Email.svelte";
	import { ripple } from "$lib/ripple";
	import { form } from "../transitions";
</script>

<style>

  .login {
    flex: 1;
    display: flex;
    align-self: stretch;
    justify-self: stretch;
    flex-direction: column;
    padding-inline: min(20%, max(12rem, 10%));
    padding-block: 6.5rem;
    position: relative;
    background-image: url("$lib/img/login-bg.jpg");
    background-image: url("$lib/img/login-bg.jpg");
    background-size: cover;
    background-position: center;
  }

  .logo {
    position: absolute;
    top: 2rem;
    left: 2rem;
    font-size: 2rem;
    font-weight: 500;
    color: var(--red);
    text-shadow: #fff 0 0 3px;
  }

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

  .field-box {
    display: flex;
    flex-direction: column;
    align-items: stretch;
  }

  .comment {
    font-size: 0.9em;
    color: #aaa;
    padding: 0.25rem 0.25em;
    margin-top: 0.75rem;
    align-self: flex-start;
    transition: color 200ms ease;
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

  @media screen and (max-width: 600px) {
    .login {
      padding-inline: 0;
    }

    .box {
      width: min(20rem, 90%);
      margin-inline: auto;
    }
  }
</style>

<div class="login">
  <div class="logo">openstream</div>
  <div class="box" in:form>
    <div class="title">Recover</div>
    <div class="fields">
      <div class="field field-box">
        <Email label="Email" value="" />
        <span class="comment" href="/recover">We'll send you an e-mail to recover your account</span>
        <a class="na back" href="/login">Back to login</a>
      </div>
      <button use:ripple class="ripple-container">
        Send
      </button>
    </div>
  </div>
</div>