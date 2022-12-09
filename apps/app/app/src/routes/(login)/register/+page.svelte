<script>
	import Email from "$lib/components/Form/Email.svelte";
	import Password from "$lib/components/Form/Password.svelte";
	import TextField from "$lib/components/Form/TextField.svelte";
	import { ripple } from "$lib/ripple";
  import { form } from "../transitions";
</script>

<style>
  .box {
    background: rgba(255,255,255,0.95);
    backdrop-filter: blur(2px);
    box-shadow: 0 4px 20px 0 rgb(0 0 0 / 12%);
    width: max(25rem, min(30rem, 55%));
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

  .link, .comment {
    font-size: 0.9em;
    transition: color 200ms ease;
  }

  .link {
    color: rgba(var(--blue-rgb), 0.6);
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

<div class="box" in:form>
  <div class="title">Sign up</div>
  <div class="fields">
    <div class="field">
      <TextField label="Your first name" value="" />
    </div>
    <div class="field">
      <TextField label="Your last name" value="" />
    </div>
    <div class="field">
      <TextField label="Your station's name" value="" />
    </div>
    <div class="field">
      <Email label="Your email" value="" />
    </div>
    <div class="field">
      <Password label="Your password" value="" />
    </div>
    <div class="field">
      <Password label="Confirm your password" value="" />
    </div>

    <button use:ripple class="ripple-container">
      Sign up
    </button>
  </div>
  <div class="new-box">
    <span class="comment">Already have an account?</span>
    <a class="na link sign-in" href="/login">Sign in</a>
  </div>
</div>