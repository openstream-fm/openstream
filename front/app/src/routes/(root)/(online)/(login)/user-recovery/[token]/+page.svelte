<script lang="ts">
  export let data: import("./$types").PageData;
  import Page from "$lib/components/Page.svelte";
	import Formy from "$share/formy/Formy.svelte";
	import { _post, action } from "$share/net.client";
  import { goto } from "$app/navigation";
	import Password from "$lib/components/Form/Password.svelte";
	import Validator from "$share/formy/Validator.svelte";
	import { _confirmation_password, _new_password } from "$share/formy/validate";
	import { ripple } from "$share/ripple";
	import Email from "$lib/components/Form/Email.svelte";
  import { _message } from "$share/notify";
  import { default_logger } from "$share/logger";
	import { locale } from "$lib/locale";
	import { invalidate_siblings } from "$lib/invalidate";

  const logger = default_logger.scoped("recovery");

  let new_password = "";
  let confirm_password = "";

  let sending = false;
  const send = action(async () => {
    
    // this will always be true
    if(data.result.kind !== "found") throw new Error("Internal error (kind !== 'found')")

    if(sending) return;
    sending = true;
    
    try {
      let payload: import("$api/auth/user/recovery-token/[token]/set-password/POST/Payload").Payload = {
        new_password
      }

      await _post(`/api/auth/user/recovery-token/${data.token}/set-password`, payload);

      let login_payload: Omit<import("$api/auth/user/login/POST/Payload").Payload, "device_id"> = {
        email: data.result.user_email,
        password: new_password,    
      }

      new_password = "";
      confirm_password = "";

      _message($locale.pages.user_recovery.notifier.password_updated);

      await _post("/api/auth/user/login", login_payload).catch(e => {
        logger.error("error on login after token password set")
        logger.error(e)
      });

      goto("/", { invalidateAll: true });
      invalidate_siblings();

      sending = false;
    } catch(e) {
      sending = false;
      throw e;
    }
  })

  $: title = 
    data.result.kind === "found" ?
      data.result.expired ? $locale.pages.user_recovery.head_page_title.expired : 
        data.result.already_used ? $locale.pages.user_recovery.head_page_title.used : 
      $locale.pages.user_recovery.head_page_title.ok :
    $locale.pages.user_recovery.head_page_title.not_found;

  const error_message = (base: string, link: string): string => {
    return base.replace("@user_recovery_page", `<a class="na" href="recover">${link}</a>`)
  }
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

  .box {
    background: #fff;
    display: flex;
    flex-direction: column;
    width: min(90%, 24rem);
    margin-top: 4rem;
    padding: 2rem;
    border-radius: 0.5rem;
    box-shadow: var(--some-shadow);
  }

  .fields {
    margin-top: 2rem;
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .send-out {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    padding: 2rem 0 0 0;
  }
  
  .send {
    padding: 0.75rem;
    color: #fff;
    box-shadow: var(--some-shadow);
    background: var(--blue);
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
</style>

<svelte:head>
  <title>{title}</title>
</svelte:head>

<Page compact>
  <div class="page">
    <h1>{title}</h1>

    {#if data.result.kind === "found"}
      {#if !data.result.expired}
        {#if !data.result.already_used}  
          <Formy action={send} let:submit>
            <form class="box" on:submit={submit}>
              <div class="fields">
                <div class="field">
                  <Email
                    label={$locale.pages.user_recovery.fields.email}
                    readonly
                    value={data.result.user_email}
                  />
                </div>
                <div class="field">
                  <Password
                    label={$locale.pages.user_recovery.fields.password}
                    autocomplete="new-password"
                    bind:value={new_password}
                  />
                  <Validator value={new_password} fn={_new_password({ minlen: 8, maxlen: 100 })} />
                </div>
                <div class="field">
                  <Password
                    label={$locale.pages.user_recovery.fields.confirm_password}
                    autocomplete="new-password"
                    bind:value={confirm_password}
                  />
                  <Validator value={{ password: new_password,  confirm_password }} fn={_confirmation_password()} />
                </div>
              </div>

              <div class="send-out">
                <button type="submit" class="send ripple-container" use:ripple>
                  {$locale.pages.user_recovery.submit}
                </button> 
              </div>
            </form>
          </Formy>
        {:else}
          <div class="error">
            <div class="error-message">
              {@html error_message(
                $locale.pages.user_recovery.error.used_message_html,
                $locale.pages.user_recovery.error.user_recovery_page
              )}
            </div>
          </div>
        {/if}
      {:else}
        <div class="error">
          <div class="error-message">
            {@html error_message(
              $locale.pages.user_recovery.error.expired_message_html,
              $locale.pages.user_recovery.error.user_recovery_page
            )}
          </div>
        </div>
      {/if}
    {:else}
      <div class="error">
        <div class="error-message">
          {@html error_message(
            $locale.pages.user_recovery.error.not_found_message_html,
            $locale.pages.user_recovery.error.user_recovery_page
          )}
        </div>
      </div>
    {/if}
  </div>
</Page>
