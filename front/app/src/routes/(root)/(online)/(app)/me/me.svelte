<script lang="ts">
  export let data: import("./$types").PageData;

  import TextField from "$share/Form/TextField.svelte";
	import NullTextField from "$share/Form/Nullable/NullTextField.svelte";
	import { ripple } from "$share/ripple";
	import { clone, diff, equals } from "$server/util/collections";
	import { _patch, action } from "$share/net.client";
	import { _message } from "$share/notify";
	import { invalidateAll } from "$lib/invalidate";
	import Email from "$share/Form/Email.svelte";
	import Password from "$share/Form/Password.svelte";
	import { mdiAccountOutline, mdiDevices, mdiKeyOutline, mdiPhoneOutline, mdiTranslate } from "@mdi/js";
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
	import { tick } from "svelte";
	import { locale } from "$lib/locale";
	import SelectField from "$share/Form/SelectField.svelte";
	import { VALIDATE_USER_FIRST_NAME_MAX_LEN, VALIDATE_USER_LAST_NAME_MAX_LEN, VALIDATE_USER_PASSWORD_MAX_LEN, VALIDATE_USER_PASSWORD_MIN_LEN, VALIDATE_USER_PHONE_MAX_LEN } from "$server/defs/constants";

  let show_change_password = true;

  const language = Object.keys($locale.language).includes(data.user.language || "") ? (data.user.language as string) : "auto";

  let profile_db = {
    first_name: data.user.first_name,
    last_name: data.user.last_name,
    phone: data.user.phone,
    language
  };

  $: language_options = [
    { label: $locale.language.auto, value: "auto" },
    ...Object.entries($locale.language).filter(([code, value]) => code !== "auto").map(([code, name]) => {
      return { label: name, value: code };
    }).sort((a, b) => a.label.localeCompare(b.label) ),
  ];

  let profile_current = clone(profile_db);

  $: can_save_profile = !equals(profile_db, profile_current);

  const save_profile = action(async () => {
    if(!can_save_profile) {
      _message($locale.pages.me.notifier.no_changes);
      return;
    };
    
    const dif = diff(profile_db, profile_current);
    
    let language: string | null | undefined = undefined;
    if(dif.language != null) {
      if(dif.language === "auto") language = null;
      else language = dif.language;
    }

    // TODO: remove this partial
    const payload: Partial<import("$api/users/[user]/PATCH/Payload").Payload> = {
      first_name: dif.first_name,
      last_name: dif.last_name,
      phone: dif.phone,
      language,
    };

    await _patch(`/api/users/${data.user._id}`, payload);
    profile_db = clone(profile_current);
    _message($locale.pages.me.notifier.profile_updated);
    invalidateAll(); 
  });

  let new_password = "";
  let confirm_new_password = "";

  const change_password = action(async () => {
    if(new_password === "") throw new Error("New password is required");
    if(new_password !== confirm_new_password) throw new Error("Confirmation password doesn't match");
    // TODO: remove this partial
    const payload: Partial<import("$api/users/[user]/PATCH/Payload").Payload> = {
      password: new_password,
    };

    await _patch(`/api/users/${data.user._id}`, payload);
    new_password = "";
    confirm_new_password = "";
    _message($locale.pages.me.notifier.password_updated);

    // force chrome prompt to update password 
    show_change_password = false;
    tick().then(() => {
      show_change_password = true;
    })
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
    font-weight: var(--font-bold);
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
    font-weight: var(--font-bold);
    font-size: 1.5em;
    text-align: start;
  }

  .page-subtitle {
    font-size: max(1rem, 1.1em);
    margin-top: 0.25rem;
    text-align: start;
  }

  @media screen and (max-width: 500px) {
    .page-top {
      font-size: 0.8rem;
    }
  }

  .section {
    width: min(90%, 500px);
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
    font-weight: var(--font-bold);
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
    cursor: pointer;
    user-select: none;
    align-self: flex-end;
    font-weight: var(--font-bold);
  }

  /* .submit.disabled {
    background: #999;
  } */

  .more {
    width: min(90%, 500px);
    margin-top: 6rem;
  }

  .more-title {
    font-weight: var(--font-bold);
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

  .hidden-field {
    display: none;
  }
</style>

<svelte:head>
  <title>{data.user.first_name} {data.user.last_name}</title>
</svelte:head>

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
      <div class="section-title">{$locale.pages.me.title}</div>
      <div class="fields">
        <div class="field">
          <Email
            label={$locale.pages.me.fields.email}
            disabled
            value={data.user.email}
          />
        </div>
        <div class="field">
          <TextField
            label={$locale.pages.me.fields.first_name}
            icon={mdiAccountOutline}
            trim
            maxlength={VALIDATE_USER_FIRST_NAME_MAX_LEN}
            bind:value={profile_current.first_name}
          />
          <Validator value={profile_current.first_name} fn={_string({
              required: true,
              maxlen: VALIDATE_USER_FIRST_NAME_MAX_LEN
            })}
          /> 
        </div>
        <div class="field">
          <TextField
            label={$locale.pages.me.fields.last_name}
            icon={mdiAccountOutline}
            trim
            bind:value={profile_current.last_name}
            maxlength={VALIDATE_USER_LAST_NAME_MAX_LEN}
          />
          <Validator value={profile_current.last_name}
            fn={_string({
              required: true,
              maxlen: VALIDATE_USER_LAST_NAME_MAX_LEN,
            })}
          /> 
        </div>
        <div class="field">
          <NullTextField
            type="tel"
            label={$locale.pages.me.fields.phone}
            icon={mdiPhoneOutline}
            trim
            bind:value={profile_current.phone}
            maxlength={VALIDATE_USER_PHONE_MAX_LEN}  
          />
          <Validator value={profile_current.phone} fn={_phone({
              maxlen: VALIDATE_USER_PHONE_MAX_LEN
            })}
          />
        </div>
        <div class="field">
          <SelectField
            label={$locale.pages.me.fields.language}
            icon={mdiTranslate}
            bind:value={profile_current.language}
            options={language_options}
          />
        </div>          
      </div>
      <div class="submit-wrap">
        <button class="submit ripple-container" type="submit" use:ripple>
          {$locale.pages.me.submit.profile}
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

  {#if show_change_password}
    <Formy action={change_password} let:submit>
      <form novalidate class="section section-password" on:submit={submit}>
        <div class="hidden-field" hidden>
          <input type="email" readonly autocomplete="username" value={data.user.email} />
        </div>
        <div class="section-title">{$locale.pages.me.change_password.title}</div>
        <div class="fields">
          <div class="field">
            <Password label={$locale.pages.me.fields.new_password} autocomplete="new-password" bind:value={new_password} />
            <Validator value={new_password} fn={_new_password({
              minlen: VALIDATE_USER_PASSWORD_MIN_LEN,
              maxlen: VALIDATE_USER_PASSWORD_MAX_LEN
            })} />
          </div>
          <div class="field">
            <Password label={$locale.pages.me.fields.confirm_password} autocomplete="new-password" bind:value={confirm_new_password} />
            <Validator value={{password: new_password, confirm_password: confirm_new_password }} fn={_confirmation_password()} />
          </div>
        </div>
        <div class="submit-wrap">
          <button class="submit ripple-container" type="submit" use:ripple> 
            {$locale.pages.me.submit.password}
          </button>
        </div>
      </form>
    </Formy>
  {/if}

  <div class="more">
    <div class="more-title">
      {$locale.pages.me.more.title}
    </div>

    <div class="more-content">
      <a href="/me/devices" class="na more-link ripple-container" use:ripple>
        <div class="more-link-icon">
          <Icon d={mdiDevices} />
        </div>
        <span class="more-link-text">
          {$locale.pages.me.more.connected_devices}
        </span>
      </a>

      <a href="/me/api-keys" class="na more-link ripple-container" use:ripple>
        <div class="more-link-icon">
          <Icon d={mdiKeyOutline} />
        </div>
        <span class="more-link-text">
          <!-- TODO: locale-->
          API Keys
        </span>
      </a>
    </div>
  </div>
</div>