<script lang="ts">
	export let data: import("./$types").PageData;

  import Page from "$lib/components/Page.svelte";
	import { _post, action } from "$share/net.client";
	import { _message } from "$share/notify";

  import { goto } from "$app/navigation";

	import { ripple } from "$share/ripple";
	
  import { clone, equals } from "$server/util/collections";
	import { prevent_unload } from "$share/prevent-unload";
	import StationProfile from "$lib/components/StationProfile.svelte";
	import Formy from "$share/formy/Formy.svelte";
  import { locale } from "$lib/locale";
	import { invalidate_siblings } from "$lib/invalidate";
	import type { StationFrequency } from "$server/defs/StationFrequency";
	import type { GooglePlayLang } from "$lib/components/google-play-lang";
	import { POST, unwrap } from "$lib/client";

  let start = {
    name: null as string | null,
    slug: null  as string | null,
    slogan: null as string | null,
    description: null as string | null,
    country_code: "" as import("$server/defs/CountryCode").CountryCode | "",
    lang_code: "" as import("$server/defs/LangCode").LangCode | "",
    type_of_content: "" as import("$server/defs/db/StationTypeOfContent").StationTypeOfContent | "",


    email: null as string | null,
    phone: null as string | null,
    whatsapp: null as string | null,
    
    website_url: null as string | null,
    twitter_url: null as string | null,
    facebook_url: null as string | null,
    instagram_url: null as string | null,
    threads_url: null as string | null,
    youtube_url: null as string | null,
    twitch_url: null as string | null,
    tiktok_url: null as string | null,
    spotify_url: null as string | null,
    radiocut_url: null  as string | null,

    google_play_url: null as string | null,
    app_store_url: null as string | null,

    picture_id: null as string | null,
    frequency: null as StationFrequency | null,

    user_metadata: {
      mob_app: {
        base_color: "",
        icon_bg_color: "",
        icon_rounded: false,
        ads: false,
        admob_app_id: null,
        admob_banner_id: null,

        google_play_title: null,
        google_play_subtitle: null,
        google_play_description: null,
        google_play_lang: "" as GooglePlayLang | "",
      }
    }
  }

  let current = clone(start);

  prevent_unload(() => {
    if(!equals(start, current)) {
      return $locale.prevent_unload_message;
    } else return null;
  })

  const send = action(async () => {
    
    const picture_id = current.picture_id;
    if(picture_id == null) throw new Error("Station logo is required");

    const name = current.name;
    if(name == null) throw new Error("Station name is required");

    const type_of_content = current.type_of_content;
    if(type_of_content === "") throw new Error("Type of content is required");

    const country_code = current.country_code;
    if(country_code === "") throw new Error("Country is required");

    const lang_code = current.lang_code;
    if(lang_code === "") throw new Error("Language is required");

    const {
      station
    } = unwrap(await POST("/stations", {
      body: {
        ...current,
        name,
        type_of_content,
        country_code,
        lang_code,
        account_id: data.account._id,
        picture_id,
        external_relay_url: null,  
      }
    })); 

    _message($locale.pages["stations/create_station"].notifier.station_created);

    current = clone(start);
    
    goto(`/accounts/${data.account._id}/stations/${station._id}`, { invalidateAll: true });
    invalidate_siblings();
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
    font-weight: var(--font-bold);
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

  .submit-wrap {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    padding: 2rem;
    position: sticky;
    bottom: -1rem;
    border-radius: 0 0 0.5rem 0.5rem;
    background: linear-gradient(to top, #fff 5%, rgba(255,255,255,0.75) 65%, transparent);
    pointer-events: none;
  }

  .submit {
    color: #fff;
    background: var(--blue);
    box-shadow: 0 4px 20px 0 rgb(0 0 0 / 16%);
    padding: 0.75rem;
    appearance: none;
    border: 0;
    margin: 0;
    cursor: pointer;
    user-select: none;
    align-self: flex-end;
    font-weight: var(--font-bold);
    pointer-events: all;
    border-radius: 0.25rem;
  }

  .submit:disabled {
    background: #999;
  }
</style>

<svelte:head>
  <title>
    {$locale.pages["stations/create_station"].head.title}
  </title>
</svelte:head>

<Page>
  <div class="page">
    <div class="page-title">
      {$locale.pages["stations/create_station"].title}
    </div>
    <Formy action={send} let:submit>
      <form novalidate class="create-box" on:submit={submit}>

        <StationProfile account_id={data.account._id} station_id={null} bind:current />

        <div class="submit-wrap">
          <button class="submit ripple-container" disabled={false} use:ripple type="submit">
            {$locale.pages["stations/create_station"].submit}
          </button>
        </div>
      </form>
    </Formy>
  </div>
</Page>