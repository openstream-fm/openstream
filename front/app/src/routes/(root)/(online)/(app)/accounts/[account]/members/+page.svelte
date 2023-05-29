<script lang="ts">
  export let data: import("./$types").PageData;
	import Email from "$lib/components/Form/Email.svelte";
  import Page from "$lib/components/Page.svelte";
	import Dialog from "$share/Dialog.svelte";
	import Icon from "$share/Icon.svelte";
	import Formy from "$share/formy/Formy.svelte";
	import Validator from "$share/formy/Validator.svelte";
	import { _email } from "$share/formy/validate";
	import { action } from "$share/net.client";
	import { _message } from "$share/notify";
	import { ripple } from "$share/ripple";
	import { mdiAccountPlusOutline } from "@mdi/js";

  type Relation = typeof data.members[number]["relation"];
  const relation_names: Record<Relation, string> = {
    "owner": "Owner",
    "staff": "Staff",
  };

  let invite_open = false;

  let invite_email = "";
  let invite = action(async () => {
    invite_email = "";
    invite_open = false;
    _message("Invitation sent")
  })
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
    flex-direction: row;
    box-shadow: var(--some-shadow);
  }

  .invite-dialog-send-icon {
    display: flex;
    font-size: 1.25rem;
    margin-inline-end: 0.5rem;
  }
</style>

<svelte:head>
  <title>Members</title>
</svelte:head>

<Page>
  <h1>Members</h1>
  
  <div class="list">
    {#each data.members as item (item._id)}
      {@const relation_name = relation_names[item.relation]}
      <div class="item" class:me={data.user._id === item._id}>
        <div class="item-name">{item.first_name} {item.last_name}</div>
        <div class="item-email">{item.email}</div>
        <div class="item-relation">{relation_name}</div>
      </div>
    {/each}
  </div>

  <h2>Pending invitations</h2>

  <p>There are no pending invitations</p>

  <div class="invite-out">
    <button class="invite ripple-container" use:ripple on:click={() => invite_open = true}>
      <div class="invite-icon">
        <Icon d={mdiAccountPlusOutline} />
      </div>
      Invite people
    </button>
  </div>

</Page>

{#if invite_open}
  <Dialog
    width="500px"
    title="Invite people to manage this account with {relation_names["staff"]} role"
    on_close={() => invite_open = false}
  >
    <Formy action={invite} let:submit>
      <form novalidate class="invite-dialog" on:submit={submit}>
        <Email label="Email" bind:value={invite_email} /> 
        <div class="invite-dialog-validate">
          <Validator value={invite_email} fn={_email({ required: true })} /> 
        </div>
        <div class="invite-dialog-send-out">
          <button type="submit" class="invite-dialog-send">
            <div class="invite-dialog-send-icon">
              <Icon d={mdiAccountPlusOutline} />
            </div>
            Invite
          </button>
        </div>
      </form>
    </Formy>
  </Dialog>
{/if}