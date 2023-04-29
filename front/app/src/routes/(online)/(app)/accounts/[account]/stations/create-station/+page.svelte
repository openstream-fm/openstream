<script lang="ts">
	export let data: import("./$types").PageData;

  import Page from "$lib/components/Page.svelte";
	import { _post, action } from "$share/net.client";
	import { _message } from "$share/notify";

  import { goto, invalidate } from "$app/navigation";

	import { ripple } from "$share/ripple";
	
  import { clone, diff, equals } from "$server/util/collections";
	import { prevent_unload } from "$share/prevent-unload";
	import StationProfile from "$lib/components/StationProfile.svelte";
	import Formy from "$share/formy/Formy.svelte";

  let start = {
    name: null as string | null,
    slogan: null as string | null,
    description: null as string | null,
    country_code: "" as import("$server/defs/CountryCode").CountryCode | "",
    type_of_content: "" as import("$server/defs/db/StationTypeOfContent").StationTypeOfContent | "",


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
    if(!equals(start, current)) {
      return "If you leave this page your changes will be lost. Do you want to leave anyway?";
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

    const payload: import("$server/defs/api/stations/POST/Payload").Payload = {
      ...current,
      name,
      type_of_content,
      country_code,
      account_id: data.account._id,
      picture_id,
      frequencies: null,
    }

    const {
      station
    } = await _post<import("$server/defs/api/stations/POST/Output").Output>(`/api/stations`, payload);

    _message("New station created");

    current = clone(start);
    
    invalidate("resource:stations");
    goto(`/accounts/${data.account._id}/stations/${station._id}`);
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
    <Formy action={send} let:submit>
      <form novalidate class="create-box" on:submit={submit}>

        <StationProfile account_id={data.account._id} bind:current />

        <div class="submit-wrap">
          <button class="submit ripple-container" use:ripple type="submit">
            Create station
          </button>
        </div>
      </form>
    </Formy>
  </div>
</Page>