<script lang="ts">
	export let data: import("./$types").PageData;

  import NullEmail from "$lib/components/Form/Nullable/NullEmail.svelte";
	import NullTextField from "$lib/components/Form/Nullable/NullTextField.svelte";
	import Page from "$lib/components/Page.svelte";
	import { _post, action } from "$share/net.client";
	import { _message } from "$share/notify";

  import { goto } from "$app/navigation";

	import { ripple } from "$lib/ripple";
	import StationPictureField from "$lib/components/Form/StationPictureField.svelte";
	
  import { clone, equals } from "$server/util/collections";
	import { prevent_unload } from "$share/prevent-unload";

  let start = {
    name: "",
    slogan: null as string | null,
    description: null as string | null,
    
    email: null as string | null,
    phone: null as string | null,
    whatsapp: null as string | null,
    
    website_url: null as string | null,
    twitter_url: null as string | null,
    facebook_url: null as string | null,
    instagram_url: null as string | null,
    youtube_url: null as string | null,
    twitch_url: null as string | null,

    google_play_url: null as string | null,
    app_store_url: null as string | null,

    picture_id: null as string | null,
  }

  let current = clone(start);

  prevent_unload(() => {
    if(equals(start, current)) return null;
    else return "If you leave this page your changes will be lost. Do you want to leave anyway?"
  })

  const send = action(async () => {
    
    let picture_id: string;

    if(current.picture_id == null) throw new Error("Station logo is required");
    picture_id = current.picture_id;

    if(current.name.trim() === "") throw new Error("Station name is required");

    const payload: import("$server/defs/api/stations/POST/Payload").Payload = {
      ...current,
      account_id: data.account._id,
      picture_id,
      frequencies: null,
    }

    const {
      station
    } = await _post<import("$server/defs/api/stations/POST/Output").Output>(`/api/stations`, payload);

    _message("New station created");

    goto(`/accounts/${data.account._id}/stations/${station._id}`, { invalidateAll: true });
  });
  
</script>

<style>
  .page {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .page-title {
    margin-top: 2rem;
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
</style>

<svelte:head>
  <title>Crete new station</title>
</svelte:head>

<Page>
  <div class="page">
    <div class="page-title">Create a station</div>
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
        <button class="submit ripple-container" use:ripple type="submit">
          Create station
        </button>
      </div>
    </form>
  </div>
</Page>