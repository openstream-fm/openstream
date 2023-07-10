<script lang="ts">
	import '$share/LoginDashboard/login-page.css';

  export let data: import("./$types").PageData;
  import Page from "$lib/components/Page.svelte";
	import Formy from "$share/formy/Formy.svelte";
	import { _post, action } from "$share/net.client";
  import Password from "$lib/components/Form/Password.svelte";
	import Validator from "$share/formy/Validator.svelte";
	import { _confirmation_password, _new_password, _string } from "$share/formy/validate";
	import { ripple } from "$share/ripple";
	import Email from "$lib/components/Form/Email.svelte";
  import { _message, _error } from "$share/notify";
  import { default_logger } from "$share/logger";
	import { assert_never } from "$share/assert-never";
	import TextField from "$lib/components/Form/TextField.svelte";
	import { mdiAccountOutline } from "@mdi/js";
	import CircularProgress from "$share/CircularProgress.svelte";
	import { scale } from "svelte/transition";
	import { invalidateAll, invalidate_siblings } from '$lib/invalidate';
	import { goto } from '$app/navigation';
  import { locale } from "$lib/locale";

  const logger = default_logger.scoped("recovery");

  const get_title = (...args: any[]) => {
    if(data.result.kind === "not-found") {
      return $locale.pages.email_invitation.head_page_title.not_found;
    } else if(data.result.kind === "ok") {
      if(data.result.invitation.is_expired) {
        return $locale.pages.email_invitation.head_page_title.expired;
      } else if(data.result.invitation.state === "accepted") {
        return $locale.pages.email_invitation.head_page_title.accepted;
      } else if(data.result.invitation.state === "rejected") {
        return $locale.pages.email_invitation.head_page_title.rejected;
      } else {
        return $locale.pages.email_invitation.head_page_title.ok;
      }
    } else {
      assert_never(data.result, "invitation_result.kind")
    }
  }

  const get_error_message = (...args: any[]) => {
    if(data.result.kind === "not-found") {
      return $locale.pages.email_invitation.error_message.not_found;
    } else if(data.result.kind === "ok") {
      if(data.result.invitation.is_expired) {
        return $locale.pages.email_invitation.error_message.expired;
      } else if(data.result.invitation.state === "accepted") {
        return $locale.pages.email_invitation.error_message.accepted;
      } else if(data.result.invitation.state === "rejected") {
        return $locale.pages.email_invitation.error_message.rejected;
      } else {
        return "";
      }
    } else {
      assert_never(data.result, "invitation_result.kind")
    }
  }

  const get_desc = (...args: any[]) => {
    if(data.result.kind === "not-found") {
      return "";
    } else if(data.result.kind === "ok") {
      const account_name = data.result.invitation.account?.name ?? `#${data.result.invitation.account_id}`;
      if(data.result.invitation.user_sender) {
        return $locale.pages.email_invitation.description.with_sender_name_html
          .replace("@sender", data.result.invitation.user_sender.first_name)
          .replace("@account", account_name)
      } else {
        return $locale.pages.email_invitation.description.without_sender_name_html.replace("@account", account_name); 
      }
    } else {
      assert_never(data.result, "invitation_result.kind")
    }
  }

  $: title = get_title(data, $locale);
  $: error_message = get_error_message(data, $locale);
  $: invitation_description_html = get_desc(data, $locale);

  const email = () => {
    if(data.result.kind === "ok") {
      return data.result.invitation.receiver_email;
    } else {
      return "";
    }
  } 

  let first_name = "";
  let last_name = "";
  let password = "";
  let confirm_password = "";
  let sending = false;

  const accept = action(async () => {
    if(sending) return;
    sending = true;
    try {

      const payload: import("$api/invitations/accept/POST/Payload").Payload = {
        first_name,
        last_name,
        password,        
        phone: null,
        token: data.token,
      };

      const { result }: import("$api/invitations/accept/POST/Output").Output = await _post("/api/invitations/accept", payload);
      if(result === "ok") {
        const payload: Omit<import("$api/auth/user/login/POST/Payload").Payload, "device_id"> = {
          email: email(),
          password,
        };
        await _post("/api/auth/user/login", payload);
        goto("/", { invalidateAll: true });
        invalidate_siblings();
      } else {
        _error($locale.pages.email_invitation.notifier.accept_error.replace("@error", result));
        invalidateAll();
      }

      sending = false;
    } catch(e) {
      invalidateAll();      
      sending = false;
      throw e;
    }
  })
</script>

<style>
  .page {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  h1 {
    font-size: 2.25rem;
    text-align: center;
    max-width: 80%;
  }

  .error {
    text-align: center;
    margin-top: 2rem;
    font-size: 1.2rem;
    line-height: 2rem;
  }

  .error :global(a) {
    color: var(--blue);
  }

  .description {
    margin-top: 3rem;
    text-align: center;
    max-width: 80%;
    font-size: 1.25rem;
  }

  .login {
    margin-top: 3rem;
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .login-btn {
    line-height: 1.75rem;
    background: var(--blue);
    color: #fff;
    text-align: center;
    max-width: min(25rem, 80%); 
    font-size: 1.1rem;
    box-shadow: var(--some-shadow);
    border-radius: 0.25rem;
    padding: 1rem 1.25rem;
  }

  .login-btn > :global(b) {
    word-break: break-all;
  }

  .accept {
    display: flex;
    flex-direction: column;
    align-items: center;
    align-self: stretch;
  }

  .accept-title {
    margin-top: 1.5rem;
    margin-bottom: 2rem;
    text-align: center;
    max-width: 80%;
    font-size: 1.25rem;
  }

  .login-page-button {
    margin-block-start: 2.5rem;
    margin-inline-end: 2.5rem;
  }
</style>

<svelte:head>
  <title>{title}</title>
</svelte:head>

<Page compact>
  <div class="page">
    <h1>{title}</h1>

    {#if invitation_description_html}
      <div class="description">
        {@html invitation_description_html}
      </div>
    {/if}

    {#if 
      data.result.kind === "ok" &&
      !data.result.invitation.is_expired &&
      data.result.invitation.state === "pending"
    }
      {#if data.result.invitation.receiver != null}
        <div class="login">
          <a 
            href="/login?email={encodeURIComponent(data.result.invitation.receiver.email)}#/me/invitations"
            class="na login-btn ripple-container"
            use:ripple
          >
            {@html $locale.pages.email_invitation.login_as_btn_html.replace("@email", data.result.invitation.receiver.email)}
          </a>
        </div>
      {:else}
        <div class="accept">
          <div class="accept-title">
            {@html $locale.pages.email_invitation.form.pre_message_html}
          </div>

          <Formy action={accept} let:submit>
            <form
              novalidate
              on:submit={submit}
              class="login-page-box"
            >
              <h2 class="login-page-title">
                {$locale.pages.email_invitation.form.title}
              </h2>
        
              <div class="login-page-fields">
                <div class="login-page-field">
                  <Email autocomplete="username" label={$locale.pages.email_invitation.form.fields.email} disabled value={email()} />
                </div>
                
                <div class="login-page-field">
                  <TextField
                    label={$locale.pages.email_invitation.form.fields.first_name}
                    trim
                    icon={mdiAccountOutline}
                    autocomplete="given-name"
                    bind:value={first_name}
                  />
                  <Validator value={first_name} fn={_string({ required: true, maxlen: 50 })} />
                </div>
              
                <div class="login-page-field">
                  <TextField
                    label={$locale.pages.email_invitation.form.fields.last_name}
                    trim
                    icon={mdiAccountOutline}
                    autocomplete="family-name"
                    bind:value={last_name}
                  />
                  <Validator value={last_name} fn={_string({ required: true, maxlen: 50 })} />
                </div>
              
                <div class="login-page-field">
                  <Password
                    label={$locale.pages.email_invitation.form.fields.password}
                    autocomplete="new-password"
                    bind:value={password}
                  />
                  <Validator value={password} fn={_new_password({ minlen: 8, maxlen: 50 })} />
                </div>
              
                <div class="login-page-field">
                  <Password
                    label={$locale.pages.email_invitation.form.fields.confirm_password}
                    autocomplete="new-password"
                    bind:value={confirm_password}
                  />
                  <Validator value={{ password, confirm_password }} fn={_confirmation_password()} />
                </div>
              </div>
      
              <button
                type="submit"
                class="ripple-container login-page-button"
                class:sending
                use:ripple
              >
                {#if sending}
                  <div class="login-page-btn-sending-progress" transition:scale|local={{ duration: 300 }}>
                    <CircularProgress />
                  </div>
                {/if}
                {$locale.pages.email_invitation.form.submit}
              </button>
            </form>
          </Formy>        
        </div>
      {/if}

    {:else}
      <div class="error">
        <div class="error-message">
          {error_message}
        </div>
      </div>
    {/if}
  </div>
</Page>
