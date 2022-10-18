<script>
	import FieldWrapper from "$lib/forms/FieldWrapper.svelte";
	import LoginEmail from "$lib/forms/LoginEmail.svelte";
	import TextField from "$lib/forms/TextField.svelte";

</script>
<style>
  .page {
    position: relative;
    min-height: 100%;
    width: 100%;
    display: flex;
  }

  .bg {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    display: flex;
    flex-direction: row;

    background: linear-gradient(to right, var(--navy) 25%, var(--aqua) 75%);
  }

  /*
  .bg-start {
    height: 100%;
    flex: 1;
    background: var(--navy);
  }

  .bg-end {
    height: 100%;
    flex: 1;
    background: var(--aqua);
  }
  */

  .form {
    position: relative;
    max-width: 90%;
    width: 27rem;
    background: #fff;
    margin: auto;
    z-index: 1;
    border-top: #0074D9 4px solid;
    filter: drop-shadow(1px 2px 8px hsl(0deg 0% 0% / 0.25));
    display: flex;
    flex-direction: column;
    /*
    box-shadow:  0 1px 1px hsl(0deg 0% 0% / 0.075),
      0 2px 2px hsl(0deg 0% 0% / 0.075),
      0 4px 4px hsl(0deg 0% 0% / 0.075),
      0 8px 8px hsl(0deg 0% 0% / 0.075),
      0 16px 16px hsl(0deg 0% 0% / 0.075);
    */
  }

  p {
    text-align: center;
    padding-top: 2.5rem;
    color: #333;
    font-size: 0.8rem;
  }

  h1 {
    text-align: center;
    padding-top: 2.5rem;
    font-size: 2rem;
    font-weight: 600;
  }

  .fields {
    display: flex;
    flex-direction: column;
    gap: 2rem;
    padding-inline: min(3rem, 10%);
    padding-block: 4rem 3rem;
  }

  button {
    margin-top: 1rem;
    background: var(--blue);
    color: #fff;
    font-weight: 500;
    align-self: flex-end;
    padding: 0.75rem 1rem;
    border-radius: 2px;
    cursor: pointer;
  }
</style>

<div class="page">
  <div class="bg">
    <div class="bg-start"></div>
    <div class="bg-end"></div>
  </div>

  <div class="form">
    <p>Openstream Admin</p>
    <h1>Sign in</h1>
    <div class="fields">
      <LoginEmail />
      <FieldWrapper>
        <TextField type="password" placeholder="Password" />
      </FieldWrapper>
      <button type="submit">
        Sign in
      </button>
    </div>
  </div>
</div>