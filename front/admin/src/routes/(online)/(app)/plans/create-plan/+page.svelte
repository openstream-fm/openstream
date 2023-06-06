<script lang="ts">
  import Page from "$lib/components/Page.svelte";
	import { clone } from "$server/util/collections";
	import Formy from "$share/formy/Formy.svelte";
	import { _patch, _post, action } from "$share/net.client";
	import { _message } from "$share/notify";
	import { ripple } from "$share/ripple";
  import { goto } from "$app/navigation";
  import PlanForm from "../PlanForm.svelte";
	import { invalidate_siblings } from "$lib/invalidate";

  let db = {
    identifier: "",
    slug: "",
    display_name: "",
    color: "",
    price: null as number | null,
    stations: null as number | null,
    listeners: null as number | null,
    transfer: null as number | null,
    storage: null as number | null,
    is_user_selectable: false,
  };

  let current = clone(db);

  let saving = false;

  const create = action(async () => {
    if(saving) return;
    saving = true;
    try {
      const {
        identifier,
        slug,
        display_name,
        price,
        color,
        stations,
        listeners,
        transfer,
        storage,
        is_user_selectable,
      } = current;

      if(price === null) throw new Error("Price is required");
      if(stations === null) throw new Error("Stations is required");
      if(listeners === null) throw new Error("Listeners is required");
      if(transfer === null) throw new Error("Transfer is required");
      if(storage === null) throw new Error("Storage is required");
      
      const payload: import("$api/plans/POST/Payload").Payload = {
        identifier,
        display_name,
        slug,
        color,
        price,
        stations,
        listeners,
        storage,
        transfer,
        is_user_selectable,
      }

      const plan = await _post(`/api/plans`, payload);
      db = clone(current);
      _message("New plan created");
      saving = false;
      goto("/plans", { invalidateAll: true });
      invalidate_siblings();
    } catch(e) {
      saving = false;
      throw e;
    }
  });
</script>

<style>
  .page {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  h1 {
    text-align: center;
    margin-top: 2rem;
  }

  .box {
    width: min(500px, 80%);
    background: #fff;
    box-shadow: var(--some-shadow);
    border-radius: 0.5rem;
    padding: 2rem 1rem;
    margin-top: 4rem;
  }

  .send-out {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
  }

  .send {
    padding: 0.75rem;
    font-weight: 600;
    color: #fff;
    background: var(--blue);
    border-radius: 0.25rem;
    box-shadow: var(--some-shadow);
    margin: 1rem 1rem 0 0;
  }
</style>


<svelte:head>
  <title>Create Plan</title>
</svelte:head>

<Page>
  <div class="page">
    <h1>Create Plan</h1>

    <Formy action={create} let:submit>
      <form novalidate class="box" on:submit={submit}>
        <div class="fields">
          <PlanForm bind:current />
        </div>
        <div class="send-out">
          <button class="send ripple-container" use:ripple type="submit">
            Create
          </button>
        </div>
      </form>
    </Formy>
  </div>
</Page>