<script lang="ts">
  export let data: import("./$types").PageData;

  import TextField from "$lib/components/Form/TextField.svelte";
	import NullTextField from "$lib/components/Form/Nullable/NullTextField.svelte";
	import { ripple } from "$share/ripple";
	import { clone, diff, equals } from "$server/util/collections";
	import { _patch, action } from "$share/net.client";
	import { _message } from "$share/notify";
	import { invalidate } from "$app/navigation";
	import Email from "$lib/components/Form/Email.svelte";
	import Password from "$lib/components/Form/Password.svelte";
	import { mdiAccountOutline, mdiDevices, mdiPhoneOutline } from "@mdi/js";
	import Icon from "$share/Icon.svelte";
  import Formy from "$share/formy/Formy.svelte";
  import Validator from "$share/formy/Validator.svelte";
  import {
    _new_password,
    _confirmation_password,
    _string,
    _phone,
  } from "$share/formy/validate";
  import Page from "$lib/components/Page.svelte";

  let profile_db = {
    first_name: data.user.first_name,
    last_name: data.user.last_name,
    phone: data.user.phone,
  };

  let profile_current = clone(profile_db);

  $: can_save_profile = !equals(profile_db, profile_current);

  const save_profile = action(async () => {
    if(!can_save_profile) {
      _message("No changes to save");
      return;
    };
    
    const dif = diff(profile_db, profile_current);
    // TODO: remove this partial
    const payload: Partial<import("$server/defs/api/users/[user]/PATCH/Payload").Payload> = dif;
    await _patch(`/api/users/${data.user._id}`, payload);
    profile_db = clone(profile_current);
    _message("Profile updated");
    invalidate("resource:users"); 
  });

  let new_password = "";
  let confirm_new_password = "";

  const change_password = action(async () => {
    if(new_password === "") throw new Error("New password is required");
    if(new_password !== confirm_new_password) throw new Error("Confirmation password doesn't match");
    // TODO: remove this partial
    const payload: Partial<import("$server/defs/api/users/[user]/PATCH/Payload").Payload> = {
      password: new_password,
    };

    await _patch(`/api/users/${data.user._id}`, payload);
    new_password = "";
    confirm_new_password = "";
    _message("Password updated");
  })

  // let new_email = "";
  // let confirm_new_email = "";

  // const change_email = action(() => {
  //   if(new_email === "") throw new Error("New email is required");
  //   if(new_email !== confirm_new_email) throw new Error("Emails does not match");
  //   if(new_email === data.user.email) throw new Error("Email should not be the same as the current email");
  //   throw new Error("This feature is not yet implemented");
  // })
</script>

<style>
  .page {
    display: flex;
    flex-grow: 1;
    flex-direction: column;
    align-items: center;
    padding-bottom: 3rem;
  }

  .page-top {
    display: flex;
    flex-direction: row;
    align-items: center;
    margin-top: 5rem;
    margin-bottom: 5rem;
    padding: 0 1.5rem;
    max-width: 600px;
  }

  .page-user-logo {
    width: 1.85em;
    height: 1.85em;
    font-size: 2em;
    color: #fff;
    font-weight: 600;
    box-shadow: var(--some-shadow);
    border-radius: 50%;
    flex: none;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--red);
    margin-inline-end: 0.75em;
  }

  .page-title {
    font-weight: 600;
    font-size: 1.5em;
    text-align: left;
  }

  .page-subtitle {
    font-size: max(1rem, 1.1em);
    margin-top: 0.25rem;
    text-align: left;
  }

  @media screen and (max-width: 500px) {
    .page-top {
      font-size: 0.8rem;
    }
  }

  .section {
    width: min(80%, 500px);
    background: #fff;
    box-shadow: 0 0 15px 0 rgb(0 0 0 / 10%);
    border-radius: 0.5rem;
    padding: 1rem 0;
    display: flex;
    flex-direction: column;
    align-items: stretch;
    min-width: 0;
    padding: 2rem;
  }

  .section-password {
    margin-top: 5rem;
  }

  .section-title {
    text-align: center;
    font-weight: 600;
    font-size: 1.5rem;
    margin: 1rem 0;
  }

  .fields {
    margin-top: 3rem;
  }

  .field + .field {
    margin-top: 2rem;
  }

  .submit-wrap {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    margin-top: 3rem;
  }

  .submit {
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

  /* .submit.disabled {
    background: #999;
  } */

  .more {
    width: min(80%, 500px);
    margin-top: 6rem;
  }

  .more-title {
    font-weight: 600;
    font-size: 1.5rem;
    text-align: center;
  }

  .more-content {
    margin-top: 2.5rem;
    padding: 1rem 0;
    background: #fff;
    box-shadow: 0 0 15px 0 rgb(0 0 0 / 10%);
    border-radius: 0.5rem;
  }

  .more-link {
    display: flex;
    flex-direction: row;
    align-items: center;
    padding: 0.75rem 1.5rem;
  }

  .more-link:hover {
    background: rgba(0,0,0,0.05);
  }

  .more-link-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1.5rem;
    margin-inline-end: 1rem;
  }

  .more-link-text {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>

<svelte:head>
  <title>{data.user.first_name} {data.user.last_name}</title>
</svelte:head>

<Page compact>
  <div class="page"> 
    <div class="page-top">
      <div class="page-user-logo">
        {data.user.first_name.slice(0, 1)}
      </div>
      <div class="page-titles">
        <div class="page-title">
          {data.user.first_name} {data.user.last_name}
        </div>
        <div class="page-subtitle">
          {data.user.email}
        </div>
      </div>
    </div>

    <Formy action={save_profile} let:submit>
      <form novalidate class="section section-profile" on:submit={submit}>
        <div class="section-title">Profile</div>
        <div class="fields">
          <div class="field">
            <Email label="Your email" disabled value={data.user.email} />
          </div>
          <div class="field">
            <TextField
              label="Your first name"
              icon={mdiAccountOutline}
              trim
              maxlength={50}
              bind:value={profile_current.first_name}
            />
            <Validator value={profile_current.first_name} fn={_string({ required: true, maxlen: 50 })} /> 
          </div>
          <div class="field">
            <TextField
              label="Your last name"
              icon={mdiAccountOutline}
              trim
              bind:value={profile_current.last_name}
              maxlength={50}
            />
            <Validator value={profile_current.last_name} fn={_string({ required: true, maxlen: 50 })} /> 
          </div>
          <div class="field">
            <NullTextField
              type="tel"
              label="Your phone number"
              icon={mdiPhoneOutline}
              trim
              bind:value={profile_current.phone}
              maxlength={40}  
            />
            <Validator value={profile_current.phone} fn={_phone()} />
          </div>
        </div>
        <div class="submit-wrap">
          <!--
            <button
            class="submit ripple-container"
            type="submit"
            use:ripple={{ opacity: !can_save_profile ? 0 : void 0 }}
            class:disabled={!can_save_profile}
            aria-disabled={!can_save_profile}
            use:tooltip={can_save_profile ? null : "No changes to save"}
          >
          -->
          <button class="submit ripple-container" type="submit" use:ripple>
            Save
          </button>
        </div>
      </form>
    </Formy>

    <!-- <form class="section" on:submit|preventDefault={change_email}>
      <div class="section-title">Change your email</div>
      <div class="fields">
        <div class="field">
          <Email label="Current email" disabled value={data.user.email} />
        </div>
        <div class="field">
          <Email label="New email" bind:value={new_email} />
        </div>
        <div class="field">
          <Email label="Confirm email" bind:value={confirm_new_email} />
        </div>
      </div>
      <div class="submit-wrap">
        <button class="submit ripple-container" type="submit" use:ripple>
          Save
        </button>
      </div>
    </form> -->

    <Formy action={change_password} let:submit>
      <form novalidate class="section section-password" on:submit={submit}>
        <div class="section-title">Change your password</div>
        <div class="fields">
          <div class="field">
            <Password label="New password" autocomplete="new-password" bind:value={new_password} />
            <Validator value={new_password} fn={_new_password({ minlen: 8, maxlen: 50 })} />
          </div>
          <div class="field">
            <Password label="Confirm password" autocomplete="new-password" bind:value={confirm_new_password} />
            <Validator value={{password: new_password, confirm_password: confirm_new_password }} fn={_confirmation_password()} />
          </div>
        </div>
        <div class="submit-wrap">
          <button class="submit ripple-container" type="submit" use:ripple> 
            Save
          </button>
        </div>
      </form>
    </Formy>

    <div class="more">
      <div class="more-title">
        More
      </div>

      <div class="more-content">
        <a href="/me/devices" class="na more-link ripple-container" use:ripple>
          <div class="more-link-icon">
            <Icon d={mdiDevices} />
          </div>
          <span class="more-link-text">Connected devices</span>
        </a>
      </div>
    </div>
  </div>
</Page>
