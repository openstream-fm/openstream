<script lang="ts">
	export let data: import("./$types").PageData;

  import Page from "$lib/components/Page.svelte";
	import { _patch, _post, action } from "$share/net.client";
	import { _message } from "$share/notify";
  import { invalidate } from "$app/navigation";
	import { ripple } from "$share/ripple";
  import { clone, diff, equals } from "$server/util/collections";
	import { prevent_unload } from "$share/prevent-unload";
	import StationProfile from "$lib/components/StationProfile.svelte";
  import Formy from "$share/formy/Formy.svelte";
	import { locale } from "$lib/locale";
	import type { GooglePlayLang } from "$lib/components/google-play-lang";

  let db = {
    name: data.station.name,
    slug: data.station.slug,
    slogan: data.station.slogan,
    description: data.station.description,
    email: data.station.email,
    phone: data.station.phone,
    whatsapp: data.station.whatsapp,
    website_url: data.station.website_url,
    twitter_url: data.station.twitter_url,
    facebook_url: data.station.facebook_url,
    instagram_url: data.station.instagram_url,
    threads_url: data.station.threads_url,
    youtube_url: data.station.youtube_url,
    twitch_url: data.station.twitch_url,
    tiktok_url: data.station.tiktok_url,
    spotify_url: data.station.spotify_url,
    radiocut_url: data.station.radiocut_url,

    google_play_url: data.station.google_play_url,
    app_store_url: data.station.app_store_url,
    picture_id: data.station.picture_id as string | null,
    country_code: data.station.country_code as typeof data.station.country_code | "",
    lang_code: data.station.lang_code as typeof data.station.lang_code | "",
    type_of_content: data.station.type_of_content as typeof data.station.type_of_content | "",
    frequency: data.station.frequency,
    user_metadata: {
      mob_app: {
        // TODO: types
        base_color: String((data.station.user_metadata.mob_app as any)?.base_color ?? ""),
        icon_bg_color: String((data.station.user_metadata.mob_app as any)?.icon_bg_color ?? ""),
        icon_rounded: !!(data.station.user_metadata.mob_app as any)?.icon_rounded ?? false,
        ads: !!(data.station.user_metadata.mob_app as any)?.ads ?? false,
        admob_app_id: String((data.station.user_metadata.mob_app as any)?.admob_app_id ?? "") || null,
        admob_banner_id: String((data.station.user_metadata.mob_app as any)?.admob_banner_id ?? "") || null,
        
        google_play_console_id: String((data.station.user_metadata.mob_app as any)?.google_play_console_id || "") || null,

        google_play_title: String((data.station.user_metadata.mob_app as any)?.google_play_title || "") || null,
        google_play_subtitle: String((data.station.user_metadata.mob_app as any)?.google_play_subtitle || "") || null,
        google_play_description: String((data.station.user_metadata.mob_app as any)?.google_play_description || "") || null,
        google_play_lang: String((data.station.user_metadata.mob_app as any)?.google_play_lang ?? "") as GooglePlayLang | "",
      }
    }
  };

  let current = clone(db);

  $: can_save = !equals(db, current);

  prevent_unload(() => {
    if(can_save) return "You have pending changes, are you sure you want to leave this page?";
    else return null;
  })

  const send = action(async () => {
    
    if(!can_save) {
      _message($locale.pages["station/profile"].notifier.no_changes);
      return;
    }

    const dif = diff(db, current);

    const picture_id = dif.picture_id;
    if(picture_id === null) throw new Error("Logo is required");

    const name = dif.name;
    if(name === null) throw new Error("Station name is required");

    const type_of_content = dif.type_of_content;
    if(type_of_content === "") throw new Error("Type of contet is required");

    const country_code = dif.country_code;
    if(country_code === "") throw new Error("Country is required");

    const lang_code = dif.lang_code;
    if(lang_code === "") throw new Error("Language is required");

    const payload: import("$api/stations/[station]/PATCH/Payload").Payload = {
      ...dif,
      name,
      type_of_content: type_of_content,
      country_code: country_code,
      lang_code: lang_code,
      picture_id,
    }

    await _patch<import("$api/stations/[station]/PATCH/Output").Output>(`/api/stations/${data.station._id}`, payload);
    
    db = clone(current);

    _message($locale.pages["station/profile"].notifier.station_updated);

    invalidate("resource:stations");
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
    transition: background-color 200ms ease;
    pointer-events: all;
    border-radius: 0.25rem;
  }

  .submit:disabled {
    background: #999;
  }
</style>

<svelte:head>
  <title>{$locale.pages["station/profile"].head.title}</title>
</svelte:head>

<Page>
  <div class="page">
    <div class="page-title">{$locale.pages["station/profile"].title}</div>
    <Formy action={send} let:submit>
      <form novalidate class="create-box" on:submit={submit}>
        
        <StationProfile account_id={data.account._id} station_id={data.station._id} bind:current />
        
        <div class="submit-wrap">
          <button class="submit ripple-container" disabled={!can_save} use:ripple type="submit">
            {$locale.pages["station/profile"].submit}
          </button>
        </div>
      </form>
    </Formy>
  </div>
</Page>