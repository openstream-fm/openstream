<script lang="ts">
	export let data: import("./$types").PageData;

  import NullEmail from "$lib/components/Form/Nullable/NullEmail.svelte";
	import NullTextField from "$lib/components/Form/Nullable/NullTextField.svelte";
	import Page from "$lib/components/Page.svelte";
	import { _patch, _post, action } from "$share/net.client";
	import { _message } from "$share/notify";
  import { invalidateAll } from "$app/navigation";

	import { ripple } from "$lib/ripple";
	
  let name: string = data.station.name
  let slogan: string | null = data.station.slogan;
  let description: string | null = data.station.description;
  let email: string | null = data.station.email;
  let phone: string | null = data.station.phone;
  let whatsapp: string | null = data.station.whatsapp;

  let website_url: string | null = data.station.website_url;
  let twitter_url: string | null = data.station.twitter_url;
  let facebook_url: string | null = data.station.facebook_url;
  let instagram_url: string | null = data.station.instagram_url;
  let youtube_url: string | null = data.station.youtube_url;
  let twitch_url: string | null = data.station.twitch_url;

  let google_play_url: string | null = data.station.google_play_url;
  let app_store_url: string | null = data.station.app_store_url;

  // TODO: send only a diff

  const send = action(async () => {
    const payload: import("$server/defs/api/stations/[station]/PATCH/Payload").Payload = {
      name,
      slogan,
      description,
      email,
      phone,
      whatsapp,
      website_url,
      twitter_url,
      facebook_url,
      instagram_url,
      youtube_url,
      twitch_url,
      google_play_url,
      app_store_url,
      
      frequencies: void 0,
    }

    await _patch<import("$server/defs/api/stations/[station]/PATCH/Output").Output>(`/api/stations/${data.station._id}`, payload);
    
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
</style>

<svelte:head>
  <title>Select station</title>
</svelte:head>

<Page>
  <div class="page">
    <div class="page-title">Station Profile</div>
    <form novalidate class="create-box" on:submit|preventDefault={send}>
      <div class="section">
        <div class="section-title">
          Profile information
        </div>
        <div class="fields">
          <div class="field">
            <NullTextField
              label="Name *"
              trim
              bind:value={name}
            />
          </div>
          <div class="field">
            <NullTextField
              label="Slogan"
              trim
              bind:value={slogan}
            />
          </div>
          <div class="field">
            <NullTextField 
              label="Description"
              multiline
              minrows={15}
              maxrows={50}
              bind:value={description}
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
              bind:value={email}
            />
          </div>
          <div class="field">
            <NullTextField
              type="tel"
              label="Full phone number"
              trim
              bind:value={phone}
            />
          </div>
          <div class="field">
            <NullTextField
              type="tel"
              label="Full WhatsApp number"
              trim
              bind:value={whatsapp}
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
              bind:value={website_url}
            />
          </div>
          <div class="field">
            <NullTextField
              type="url"
              label="Twitter URL"
              trim
              bind:value={twitter_url}
            />
          </div>
          <div class="field">
            <NullTextField
              type="url"
              label="Facebook URL"
              trim
              bind:value={facebook_url}
            />
          </div>
          <div class="field">
            <NullTextField
              type="url"
              label="Instagram URL"
              trim
              bind:value={instagram_url}
            />
          </div>
          <div class="field">
            <NullTextField
              type="url"
              label="Youtube URL"
              trim
              bind:value={youtube_url}
            />
          </div>
          <div class="field">
            <NullTextField
              type="url"
              label="Twitch URL"
              trim
              bind:value={twitch_url}
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
              bind:value={google_play_url}
            />
          </div>
          <div class="field">
            <NullTextField
              type="url"
              label="App Store URL"
              trim
              bind:value={app_store_url}
            />
          </div>
        </div>
      </div>

      <div class="submit-wrap">
        <button class="submit ripple-container" use:ripple type="submit">
          Save
        </button>
      </div>
    </form>
  </div>
</Page>