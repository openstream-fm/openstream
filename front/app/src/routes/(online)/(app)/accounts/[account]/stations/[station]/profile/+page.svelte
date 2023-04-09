<script lang="ts">
	export let data: import("./$types").PageData;

  import NullEmail from "$lib/components/Form/Nullable/NullEmail.svelte";
	import NullTextField from "$lib/components/Form/Nullable/NullTextField.svelte";
	import Page from "$lib/components/Page.svelte";
	import { _patch, _post, action } from "$share/net.client";
	import { _message } from "$share/notify";
  import { invalidateAll } from "$app/navigation";

	import { ripple } from "$lib/ripple";
	import StationPictureField from "$lib/components/Form/StationPictureField.svelte";
	
  import { clone, diff, equals } from "$server/util/collections";
	import { tooltip } from "$share/tooltip";
	import { prevent_unload } from "$share/prevent-unload";

  let db = {
    name: data.station.name,
    slogan: data.station.slogan,
    description: data.station.description,
    email: data.station.email,
    phone: data.station.phone,
    whatsapp: data.station.whatsapp,
    website_url: data.station.website_url,
    twitter_url: data.station.twitter_url,
    facebook_url: data.station.facebook_url,
    instagram_url: data.station.instagram_url,
    youtube_url: data.station.youtube_url,
    twitch_url: data.station.twitch_url,
    google_play_url: data.station.google_play_url,
    app_store_url: data.station.app_store_url,
    picture_id: data.station.picture_id as string | null,
  };

  let current = clone(db);

  $: can_save = !equals(db, current);

  prevent_unload(() => {
    if(can_save) return "You have pending changes, are you sure you want to leave this page?";
    else return null;
  })

  // TODO: send only a diff

  const send = action(async () => {
    
    if(!can_save) {
      _message("No changes to save");
      return;
    }

    const dif = diff(db, current);

    let picture_id: string | undefined;
    if(dif.picture_id === null) {
      throw new Error("Logo is required");
    } else {
      picture_id = dif.picture_id;
    }

    const payload: import("$server/defs/api/stations/[station]/PATCH/Payload").Payload = {
      ...dif,
      picture_id,
      // frequencies: void 0,
    }

    await _patch<import("$server/defs/api/stations/[station]/PATCH/Output").Output>(`/api/stations/${data.station._id}`, payload);
    
    db = clone(current);

    _message("Station updated");

    invalidateAll();
  });
  
</script>

<style>
  .page {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .page-title {
    margin-top: 4rem;
    margin-bottom: 2rem;
    font-size: 2rem;
    font-weight: 600;
  }

  .create-box {
    margin-top: 3.5rem;
    width: min(100%, 600px);
    background: #fff;
    box-shadow: 0 0 15px 0 rgb(0 0 0 / 10%);
    border-radius: 0.5rem;
    display: flex;
    flex-direction: column;
    align-items: stretch;
    min-width: 0;
    padding-top: 3rem;
  }

  .section + .section {
    margin-top: 4rem;
  }

  .section-title {
    font-size: 1.25rem;
    font-weight: 600;
    text-align: center;
  }
  .fields {
    display: grid;
    gap: 2.5rem;
    padding: 2rem;
  }

  .submit-wrap {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    padding: 2rem;
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
</style>

<svelte:head>
  <title>Station Profile</title>
</svelte:head>

<Page>
  <div class="page">
    <div class="page-title">Station Profile</div>
    <form novalidate class="create-box" on:submit|preventDefault={send}>
      
      <div class="section">
        <div class="section-title">
          Logo
        </div>
        <div class="fields">
          <div class="field">
            <StationPictureField account={data.account} bind:picture_id={current.picture_id} />
          </div>
        </div>
      </div>

      <div class="section">
        <div class="section-title">
          Profile information
        </div>
        <div class="fields">
          <div class="field">
            <NullTextField
              label="Name *"
              trim
              bind:value={current.name}
            />
          </div>
          <div class="field">
            <NullTextField
              label="Slogan"
              trim
              bind:value={current.slogan}
            />
          </div>
          <div class="field">
            <NullTextField 
              label="Description"
              multiline
              minrows={15}
              maxrows={50}
              bind:value={current.description}
            />
          </div>
        </div>
      </div>
      
      <div class="section">
        <div class="section-title">
          Contact information
        </div>
        <div class="fields">
          <div class="field">
            <NullEmail
              label="Email"
              bind:value={current.email}
            />
          </div>
          <div class="field">
            <NullTextField
              type="tel"
              label="Full phone number"
              trim
              bind:value={current.phone}
            />
          </div>
          <div class="field">
            <NullTextField
              type="tel"
              label="Full WhatsApp number"
              trim
              bind:value={current.whatsapp}
            />
          </div>
        </div>
      </div>

      <div class="section">
        <div class="section-title">
          Social links
        </div>
        <div class="fields">
          <div class="field">
            <NullTextField
              type="url"
              label="Website URL"
              trim
              bind:value={current.website_url}
            />
          </div>
          <div class="field">
            <NullTextField
              type="url"
              label="Twitter URL"
              trim
              bind:value={current.twitter_url}
            />
          </div>
          <div class="field">
            <NullTextField
              type="url"
              label="Facebook URL"
              trim
              bind:value={current.facebook_url}
            />
          </div>
          <div class="field">
            <NullTextField
              type="url"
              label="Instagram URL"
              trim
              bind:value={current.instagram_url}
            />
          </div>
          <div class="field">
            <NullTextField
              type="url"
              label="Youtube URL"
              trim
              bind:value={current.youtube_url}
            />
          </div>
          <div class="field">
            <NullTextField
              type="url"
              label="Twitch URL"
              trim
              bind:value={current.twitch_url}
            />
          </div>
        </div>
      </div>

      <div class="section">
        <div class="section-title">
          App links
        </div>
        <div class="fields">
          <div class="field">
            <NullTextField
              type="url"
              label="Google Play URL"
              trim
              bind:value={current.google_play_url}
            />
          </div>
          <div class="field">
            <NullTextField
              type="url"
              label="App Store URL"
              trim
              bind:value={current.app_store_url}
            />
          </div>
        </div>
      </div>

      <div class="submit-wrap">
        <button class="submit ripple-container" class:disabled={!can_save} disabled={!can_save} use:tooltip={can_save ? null : "No changes to save"} use:ripple type="submit">
          Save
        </button>
      </div>
    </form>
  </div>
</Page>