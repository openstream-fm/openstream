<script lang="ts">
	import { mdiApple, mdiFacebook, mdiGooglePlay, mdiInstagram, mdiPhoneOutline, mdiTwitch, mdiTwitter, mdiWeb, mdiWhatsapp, mdiYoutube } from "@mdi/js";
	import NullEmail from "./Form/Nullable/NullEmail.svelte";
	import NullTextField from "./Form/Nullable/NullTextField.svelte";
	import StationPictureField from "./Form/StationPictureField.svelte";

  export let account_id: string;
  export let current: {
    picture_id: string | null
    
    name: string | null
    slogan: string | null
    description: string | null

    email: string | null
    phone: string | null
    whatsapp: string | null

    website_url: string | null,
    twitter_url: string | null
    facebook_url: string | null
    instagram_url: string | null
    youtube_url: string | null
    twitch_url: string | null
    
    google_play_url: string | null
    app_store_url: string | null
  };
</script>

<style>
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
</style>

<div class="section">
  <div class="section-title">
    Logo
  </div>
  <div class="fields">
    <div class="field">
      <StationPictureField {account_id} bind:picture_id={current.picture_id} />
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
        icon={mdiPhoneOutline}
        bind:value={current.phone}
      />
    </div>
    <div class="field">
      <NullTextField
        type="tel"
        label="Full WhatsApp number"
        trim
        icon={mdiWhatsapp}
        bind:value={current.whatsapp}
      />
    </div>
    <div class="field">
      <NullTextField
        type="url"
        label="Website URL"
        trim
        icon={mdiWeb}
        bind:value={current.website_url}
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
        label="Twitter URL"
        trim
        icon={mdiTwitter}
        bind:value={current.twitter_url}
      />
    </div>
    <div class="field">
      <NullTextField
        type="url"
        label="Facebook URL"
        trim
        icon={mdiFacebook}
        bind:value={current.facebook_url}
      />
    </div>
    <div class="field">
      <NullTextField
        type="url"
        label="Instagram URL"
        trim
        icon={mdiInstagram}
        bind:value={current.instagram_url}
      />
    </div>
    <div class="field">
      <NullTextField
        type="url"
        label="Youtube URL"
        trim
        icon={mdiYoutube}
        bind:value={current.youtube_url}
      />
    </div>
    <div class="field">
      <NullTextField
        type="url"
        label="Twitch URL"
        trim
        icon={mdiTwitch}
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
        icon={mdiGooglePlay}
        bind:value={current.google_play_url}
      />
    </div>
    <div class="field">
      <NullTextField
        type="url"
        label="App Store URL"
        trim
        icon={mdiApple}
        bind:value={current.app_store_url}
      />
    </div>
  </div>
</div>