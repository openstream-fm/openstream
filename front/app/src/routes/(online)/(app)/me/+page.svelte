<script lang="ts">
	import TopUser from "$lib/components/Dashboard/TopUser.svelte";
	import TextField from "$lib/components/Form/TextField.svelte";
	import NullTextField from "$lib/components/Form/Nullable/NullTextField.svelte";
	import { ripple } from "$lib/ripple";
	import { clone, diff, equals } from "$server/util/collections";
	import { _patch, action } from "$share/net.client";
	import { _message } from "$share/notify";
	import { fly } from "svelte/transition";
  import { invalidateAll } from "$app/navigation";
	import Email from "$lib/components/Form/Email.svelte";
	import Password from "$lib/components/Form/Password.svelte";
	import { mdiDevices } from "@mdi/js";
	import Icon from "$share/Icon.svelte";
  export let data: import("./$types").PageData;

  let profile_db = {
    first_name: data.user.first_name,
    last_name: data.user.last_name,
    phone: data.user.phone,
  };

  let profile_current = clone(profile_db);

  $: can_save_profile = !equals(profile_db, profile_current);

  const save_profile = action(async () => {
    if(!can_save_profile) return;
    const dif = diff(profile_db, profile_current);
    // @ts-ignore
    const payload: import("$server/defs/api/users/[user]/PATCH/Payload").Payload = dif;
    await _patch(`/users/${data.user._id}`, payload);
    _message("Profile updated");
    invalidateAll();
  });

  let new_password = "";
  let confirm_new_password = "";

  const change_password = action(() => {
    if(new_password === "") throw new Error("New password is required");
    if(new_password !== confirm_new_password) throw new Error("Passwords does not match");

    throw new Error("This feature is not yet implemented");
  })

  let new_email = "";
  let confirm_new_email = "";

  const change_email = action(() => {
    if(new_email === "") throw new Error("New email is required");
    if(new_email !== confirm_new_email) throw new Error("Emails does not match");
    if(new_email === data.user.email) throw new Error("Email should not be the same as the current email");
    throw new Error("This feature is not yet implemented");
  })
</script>

<style>

  .top {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    padding: 1.25rem 1.5rem;
    background: #fff;
    box-shadow: var(--some-shadow);
  }

  .title {
    color: var(--red);
    font-size: min(6vw, 2rem);
    font-weight: 600;
  }

  .user-btn {
    margin-inline-end: -1rem;
  }

  .layout {
    flex: 1;
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    background: var(--bg-gray);
  }

  .page {
    display: flex;
    flex-grow: 1;
    flex-direction: column;
    align-items: center;
    padding-bottom: 3rem;
  }

  .page-title {
    font-weight: 600;
    font-size: 2rem;
    margin-top: 5rem;
    margin-bottom: 5rem;
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

  .section + .section {
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

  .submit.disabled {
    background: #999;
  }

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

<div class="layout" in:fly|local={{ x: -25, duration: 200 }}>
  <div class="top">
    <div class="title">
      openstream
    </div>

    <div class="user-btn">
      <TopUser />
    </div>
  </div>
  <div class="page">
    
    <div class="page-title">
      {data.user.first_name} {data.user.last_name}
    </div>

    <form class="section" on:submit|preventDefault={save_profile}>
      <div class="section-title">Profile</div>
      <div class="fields">
        <div class="field">
          <TextField label="Your first name" trim bind:value={profile_current.first_name} />
        </div>
        <div class="field">
          <TextField label="Your last name" trim bind:value={profile_current.last_name} />
        </div>
        <div class="field">
          <NullTextField type="tel" label="Your phone number" trim bind:value={profile_current.phone} />
        </div>
      </div>
      <div class="submit-wrap">
        <!-- <button class="submit ripple-container" type="submit" use:ripple={{ opacity: can_save_profile ? 0 : void 0 }} class:disabled={!can_save_profile} disabled={!can_save_profile}> -->
        <button class="submit ripple-container" type="submit" use:ripple>
          Save
        </button>
      </div>
    </form>

    <form class="section" on:submit|preventDefault={change_email}>
      <div class="section-title">Change your email</div>
      <div class="fields">
        <div class="field">
          <Email label="Current email" value={data.user.email} />
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
    </form>

    <form class="section" on:submit|preventDefault={change_password}>
      <div class="section-title">Change your password</div>
      <div class="fields">
        <div class="field">
          <Password label="New password" autocomplete="new-password" bind:value={new_password} />
        </div>
        <div class="field">
          <Password label="Confirm password" bind:value={confirm_new_password} />
        </div>
      </div>
      <div class="submit-wrap">
        <button class="submit ripple-container" type="submit" use:ripple> 
          Save
        </button>
      </div>
    </form>

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
</div>
