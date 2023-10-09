<script lang="ts">
  export let data: import("./$types").PageData;
	import Email from "$share/Form/Email.svelte";
  import Page from "$lib/components/Page.svelte";
	import type { PublicInvitation } from "$server/defs/api/PublicInvitation";
	import CircularProgress from "$share/CircularProgress.svelte";
	import Dialog from "$share/Dialog.svelte";
	import Icon from "$share/Icon.svelte";
	import Formy from "$share/formy/Formy.svelte";
	import Validator from "$share/formy/Validator.svelte";
	import { _email } from "$share/formy/validate";
	import { _delete, _post, action } from "$share/net.client";
	import { _message } from "$share/notify";
	import { ripple } from "$share/ripple";
	import { mdiAccountPlusOutline, mdiDotsVertical, mdiSecurity, mdiTrashCanOutline } from "@mdi/js";
	import { scale, slide } from "svelte/transition";
  import { locale } from "$lib/locale";
	import { logical_fly } from "$share/transition";
	import { click_out } from "$share/actions";
	import { invalidate } from "$app/navigation";
	import { invalidateAll } from "$lib/invalidate";

  type Member = Exclude<typeof data.access.members, null>[number];
  type Relation = Member["relation"];
  const relation_names: Record<Relation, string> = {
    "owner": "Admin",
    "staff": "Staff",
  };

  let member_menu_open_id: string | null = null;

  const toggle_member_menu = (id: string) => {
    if(member_menu_open_id === id) member_menu_open_id = null;
    else member_menu_open_id = id;
  }

  const member_menu_click_out = () => {
    setTimeout(() => {
      member_menu_open_id = null;
    }, 3)
  }

  let roling = false;
  const set_member_role = action(async (member: Member, relation: "owner" | "staff") => {
    if(roling) return;
    roling = true;
    try {
      const payload: import("$api/accounts/[account]/members/[member]/set-role/POST/Payload").Payload = {
        role: relation
      }
      
      await _post(`/api/accounts/${data.account._id}/members/${member._id}/set-role`, payload);
      
      member_menu_open_id = null;

      _message($locale.pages["account/members"].notifier.member_role_changed);
      roling = false;
      invalidateAll();
    } catch(e) {
      roling = false;
      invalidateAll();
      throw e;
    }
  })

  
  let deleting_member = false;
  const del_member = action(async (member: Member) => {
    if(deleting_member == true) return;
    deleting_member = true;
    try {
      await _delete<import("$api/accounts/[account]/members/[member]/DELETE/Output").Output>(`/api/accounts/${data.account._id}/members/${member._id}`);
      _message($locale.pages["account/members"].notifier.member_access_revoked)
      member_menu_open_id = null;
      deleting_member = false;
      invalidateAll();

    } catch(e) {
      deleting_member = false;
      invalidateAll();
      throw e;
    }
  })


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
      if(item.is_expired || item.state !== "pending" || item.deleted_at != null) continue;
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

  let invitation_menu_open_id: string | null = null;

  const toggle_invitation_menu = (id: string) => {
    if(invitation_menu_open_id === id) invitation_menu_open_id = null;
    else invitation_menu_open_id = id;
  }

  const invitation_menu_click_out = () => {
    setTimeout(() => {
      invitation_menu_open_id = null;
    }, 3)
  }

  let deleting = false;
  const del_invitation = action(async (item: PublicInvitation) => {

    if(deleting) return;
    deleting = true;

    try {
      // remove all pending invitations to the same email
      const ids = new Set([item.id]);
      if(data.access.is_owner) {
        for(const each of data.access.invitations.items) {
          if(each.deleted_at != null) continue;
          if(each.state !== "pending") continue;
          if(each.receiver_email !== item.receiver_email) continue;
          ids.add(each.id);
        }
      }

      await Promise.all([...ids].map(async id => {
        await _delete<import("$api/invitations/[invitation]/DELETE/Output").Output>(`/api/invitations/${id}`);
      }));

      invitation_menu_open_id = null;
      deleting = false;
      invalidate("api:invitations");
    } catch(e) {
      deleting = false;
      invalidate("api:invitations");
      throw e;
    }
  })
</script>

<style>
  h1, h2 {
    font-weight: var(--font-bold);
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
    flex-direction: row;
    align-items: center;
  }

  .item-data {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    flex: 1;
  }

  .item-name {
    font-weight: var(--font-bold);
    font-size: 1.2rem;
  }

  .item-email {
    color: #444;
    font-size: 0.95rem;
    margin-top: 0.25rem;
    word-break: break-all;
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
    margin-top: 2rem;
  }

  .invite {
    color: #fff;
    font-weight: var(--font-bold);
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
    font-weight: var(--font-bold);
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

  .invitations {
    background: #fff;
    padding: 0.5rem 0;
    box-shadow: var(--some-shadow);
    border-radius: 0.5rem;
    margin-top: 1rem;
  }

  .invitation-item {
    padding: 0 1rem;
    display: flex;
    flex-direction: row;
    align-items: center;
  }

  .invitation-item:nth-child(even) {
    background: rgba(0,0,0,0.05);
  }

  .invitation-item-email {
    padding: 0.75rem 0;
    font-weight: var(--font-bold);
    flex: 1;
    word-break: break-all;
  }

  .menu-anchor {
    position: relative;
    flex: none; 
  }

  .menu-btn {
    display: flex;
    font-size: 1.5rem;
    border-radius: 50%;
    padding: 0.75rem;
    transition: background-color 200ms ease;
  }

  .menu-btn:hover, .menu-btn.open {
    background-color: rgba(0,0,0,0.05)
  }


  .menu {
    position: absolute;
    inset-block-start: 100%;
    inset-inline-end: 0;
    background: #fff;
    box-shadow: var(--some-shadow);
    border-radius: 0.25rem;
    padding: 0.25rem;
    min-width: 10rem;
    display: flex;
    flex-direction: column;
    z-index: 1;
  }

  .menu-item {
    white-space: nowrap;
    display: flex;
    flex-direction: row;
    align-items: center;
    padding: 0.5rem 0.75rem;
    transition: background-color 200ms ease;
    align-self: stretch;
  }

  .menu-item:hover {
    background-color: rgba(0,0,0,0.05);
  }

  .menu-item-icon {
    display: flex;
    margin-inline-end: 0.75rem;
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
        <br />
        {$locale.pages["account/members"].no_owner_message_p2}
      </p>
    </div>
  {:else}
    
    <div class="list">
      {#each data.access.members as item (item._id)}
        {@const is_self = data.user._id === item._id}
        {@const menu_open = member_menu_open_id === item._id}
        {@const relation_name = relation_names[item.relation]}
        <div class="item" class:me={is_self} transition:slide|local={{ duration: 400 }}>
          <div class="item-data">
            <div class="item-name">{item.first_name} {item.last_name}</div>
            <div class="item-email">{item.email}</div>
            <div class="item-relation">{relation_name}</div>
          </div>
          {#if !is_self}
            <div class="menu-anchor">
              <button
                class="menu-btn ripple-container"
                class:open={menu_open}
                on:click={() => toggle_member_menu(item._id)}
                use:ripple
              >
                <Icon d={mdiDotsVertical} /> 
              </button>
              {#if menu_open}
                <div
                  class="menu"
                  transition:logical_fly|local={{ duration: 200, y: -15, x: 15 }}
                  use:click_out={member_menu_click_out}
                >
                  {#if item.relation === "owner"}
                    <button
                      class="menu-item ripple-container"
                      use:ripple
                      on:click={() => set_member_role(item, "staff")}
                    >
                      <div class="menu-item-icon">
                        <Icon d={mdiSecurity} />
                      </div>
                        {
                          $locale.pages["account/members"].actions.set_role_to
                            .replace("@role", relation_names["staff"])
                        }
                    </button>
                  {:else}
                    <button
                      class="menu-item ripple-container"
                      use:ripple
                      on:click={() => set_member_role(item, "owner")}
                    >
                      <div class="menu-item-icon">
                        <Icon d={mdiSecurity} />
                      </div>
                      {
                        $locale.pages["account/members"].actions.set_role_to
                          .replace("@role", relation_names["owner"])
                      }
                    </button>
                  {/if}
                  <button
                    class="menu-item ripple-container"
                    use:ripple
                    on:click={() => del_member(item)}
                  >
                    <div class="menu-item-icon">
                      <Icon d={mdiTrashCanOutline} />
                    </div>
                      {$locale.pages["account/members"].actions.revoke_access}
                    </button>
                </div>
              {/if}
            </div>
          {/if}
      </div>
      {/each}
    </div>

 
    <h2>{$locale.pages["account/members"].Pending_invitations}</h2>

    {@const invitations = filter_invitations(data.access.invitations.items)}
    
    {#if invitations.length}
      <div class="invitations" transition:slide|local={{ duration: 400 }}>
        {#each invitations as item (item.id)}
          {@const menu_open = invitation_menu_open_id == item.id}
          <div class="invitation-item" transition:slide|local={{ duration: 400 }}>
            <div class="invitation-item-email">{item.receiver_email}</div>
            <div class="menu-anchor">
              <button
                class="menu-btn ripple-container"
                class:open={menu_open}
                use:ripple
                on:click={() => toggle_invitation_menu(item.id)}
              >
                <Icon d={mdiDotsVertical} />
              </button>
              {#if menu_open}
                <div
                  class="menu"
                  transition:logical_fly|local={{ duration: 200, y: -15, x: 15 }}
                  use:click_out={invitation_menu_click_out}
                >
                  <button
                    class="menu-item ripple-container"
                    use:ripple
                    on:click={() => del_invitation(item)}
                  >
                    <div class="menu-item-icon">
                      <Icon d={mdiTrashCanOutline} />
                    </div>
                    {$locale.pages["account/members"].actions.delete}
                  </button>
                </div>
              {/if}
            </div>
          </div>
        {/each}
      </div>
    {:else}
      <p class="invitations-empty" transition:slide|local={{ duration: 400 }}>
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
        <Email label={$locale.pages["account/members"].dialogs.invite.Email} autocomplete="username" bind:value={invite_email} /> 
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