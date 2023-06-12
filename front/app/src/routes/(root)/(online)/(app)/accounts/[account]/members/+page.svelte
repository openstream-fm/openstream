<script lang="ts">
  export let data: import("./$types").PageData;
	import Email from "$lib/components/Form/Email.svelte";
  import Page from "$lib/components/Page.svelte";
	import type { PublicInvitation } from "$server/defs/api/PublicInvitation";
	import CircularProgress from "$share/CircularProgress.svelte";
	import Dialog from "$share/Dialog.svelte";
	import Icon from "$share/Icon.svelte";
	import Formy from "$share/formy/Formy.svelte";
	import Validator from "$share/formy/Validator.svelte";
	import { _email } from "$share/formy/validate";
	import { _post, action } from "$share/net.client";
	import { _message } from "$share/notify";
	import { ripple } from "$share/ripple";
	import { mdiAccountPlusOutline } from "@mdi/js";
	import { scale } from "svelte/transition";
  import { locale } from "$lib/locale";


  type Relation = Exclude<typeof data.access.members, null>[number]["relation"];
  const relation_names: Record<Relation, string> = {
    "owner": "Admin",
    "staff": "Staff",
  };

  let invite_open = false;

  let invite_email = "";
  let sending = false;
  let invite = action(async () => {
    if(sending) return;
    if(!data.access.is_owner) return;
    sending = true;

    try {
      let payload: import("$api/invitations/POST/Payload").Payload = {
        account_id: data.account._id,
        email: invite_email,
      }

      const { invitation } = await _post<import("$api/invitations/POST/Output").Output>("/api/invitations", payload);

      invite_email = "";
      invite_open = false;

      data.access.invitations = {
        total: data.access.invitations.total + 1,
        limit: data.access.invitations.limit,
        skip: data.access.invitations.skip,
        items: [...data.access.invitations.items, invitation],
      }

      _message($locale.pages["account/members"].notifier.invitation_sent)
      sending = false;
    } catch(e) {
      sending = false;
      throw e;
    }
  })

  const filter_invitations = (items: PublicInvitation[]): PublicInvitation[] => {
    const map = new Map<string, PublicInvitation>();
    // we only show the last pending invitation for each receiver_email 
    // sort the invitations in reverse order and filter already existing invitations
    for(const item of items.slice().sort((a, b) => b.created_at.localeCompare(a.created_at))) {
      if(item.is_expired || item.state !== "pending") continue;
      if(map.has(item.receiver_email)) continue;
      map.set(item.receiver_email, item);
    }

    // we show the invitaitons in creation date (ascending) order
    // so we have to reverse the order of the map
    return [...map.values()].reverse()
  }

  const email_validate = (() => {
    const parent = _email({ required: true });
    return (email: string) => {
      const message = parent(email);
      if(message != null) {
        return message;
      }

      const exists = (data.access.members || []).some(item => item.email.trim().toLowerCase() === email.trim().toLowerCase());
      
      if(exists) {
        return $locale.pages["account/members"].validate.user_account_exists
          .replace("@email", email);
      }

      return null;
    }
  })()
</script>

<style>
  h1, h2 {
    font-weight: 600;
  }

  .list {
    background: #fff;
    box-shadow: var(--some-shadow);
    padding: 0.5rem 0;
    border-radius: 0.5rem;
    margin-top: 2rem;
  }

  .item {
    padding: 1rem;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
  }

  .item-name {
    font-weight: 600;
    font-size: 1.2rem;
  }

  .item-email {
    color: #444;
    font-size: 0.95rem;
    margin-top: 0.25rem;
  }

  .item-relation {
    font-size: 0.8rem;
    border-radius: 0.25rem;
    background-color: rgba(0,0,0,0.075);
    color: #444;
    padding: 0.4rem 0.5rem;
    margin-top: 0.5rem;
  }

  .item.me {
    background: rgba(var(--blue-rgb), 0.1);
  }

  h2 {
    margin-top: 3rem;
  }

  p {
    margin-top: 0.75rem;
  }

  .invite-out {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    margin-top: 1rem;
  }

  .invite {
    color: #fff;
    font-weight: 600;
    background: var(--blue);
    padding: 0.75rem;
    border-radius: 0.25rem;
    display: flex;
    flex-direction: row;
    box-shadow: var(--some-shadow);
  }

  .invite-icon {
    display: flex;
    font-size: 1.25rem;
    margin-inline-end: 0.5rem;
  }

  .invite-dialog-send-out {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    margin-top: 2rem;
  }

  .invite-dialog-send {
    color: #fff;
    font-weight: 600;
    background: var(--blue);
    padding: 0.75rem;
    border-radius: 0.25rem;
    display: flex;
    box-shadow: var(--some-shadow);
    position: relative;
  }

  .invite-dialog-send-inner {
    display: flex;
    flex-direction: row;
    align-items: center;
    transition: opacity 200ms ease;
  }

  .invite-dialog-send.sending > .invite-dialog-send-inner {
    opacity: 0;
  }

  .invite-dialog-send-sending-icon {
    position: absolute;
    display: flex;
    font-size: 1.25rem;
    top: calc(50% - (1.25rem / 2));
    left: calc(50% - (1.25rem / 2));
  }

  .invite-dialog-send-icon {
    display: flex;
    font-size: 1.25rem;
    margin-inline-end: 0.5rem;
  }

  .no-owner-message {
    margin-top: 2rem;
    font-size: 1.15rem;
    line-height: 2rem;
  }

</style>

<svelte:head>
  <title>{$locale.pages["account/members"].head.title}</title>
</svelte:head>

<Page>
  <h1>{$locale.pages["account/members"].title}</h1>
  
  {#if !data.access.is_owner}
    <div class="no-owner-message">
      <p>
        {$locale.pages["account/members"].no_owner_message_p1}
      </p>
      <p>
        {$locale.pages["account/members"].no_owner_message_p2}
      </p>
    </div>
  {:else}
    
    <div class="list">
      {#each data.access.members as item (item._id)}
        {@const relation_name = relation_names[item.relation]}
        <div class="item" class:me={data.user._id === item._id}>
          <div class="item-name">{item.first_name} {item.last_name}</div>
          <div class="item-email">{item.email}</div>
          <div class="item-relation">{relation_name}</div>
        </div>
      {/each}
    </div>

    {@const invitations = filter_invitations(data.access.invitations.items)}
 
    <h2>{$locale.pages["account/members"].Pending_invitations}</h2>
    
    {#if invitations.length}
      <div class="invitations">
        {#each filter_invitations(data.access.invitations.items) as item (item.id)}
          <div class="invitation-item">
            <div class="invitation-item-email">{item.receiver_email}</div>
          </div>
        {/each}
      </div>
    {:else}
      <p class="invitations-empty">
        {$locale.pages["account/members"].no_pending_invitations_message}
      </p>
    {/if}

    <div class="invite-out">
      <button class="invite ripple-container" use:ripple on:click={() => invite_open = true}>
        <div class="invite-icon">
          <Icon d={mdiAccountPlusOutline} />
        </div>
        {$locale.pages["account/members"].invite_btn_text}
      </button>
    </div>
  {/if}
</Page>

{#if invite_open}
  <Dialog
    width="500px"
    title={
      $locale.pages["account/members"].dialogs.invite.title
        .replace("@role", "Staff")
    }
    on_close={() => invite_open = false}
  >
    <Formy action={invite} let:submit>
      <form novalidate class="invite-dialog" on:submit={submit}>
        <Email label={$locale.pages["account/members"].dialogs.invite.Email} bind:value={invite_email} /> 
        <div class="invite-dialog-validate">
          <Validator value={invite_email} fn={email_validate} /> 
        </div>
        <div class="invite-dialog-send-out">
          <button type="submit" class="invite-dialog-send ripple-container" use:ripple class:sending>
            <div class="invite-dialog-send-inner">
              <div class="invite-dialog-send-icon">
                <Icon d={mdiAccountPlusOutline} />
              </div>
              {$locale.pages["account/members"].dialogs.invite.submit}
            </div>
            {#if sending}
              <div class="invite-dialog-send-sending-icon" transition:scale|local={{ duration: 300 }}>
                <CircularProgress />
              </div>
            {/if}
          </button>
        </div>
      </form>
    </Formy>
  </Dialog>
{/if}