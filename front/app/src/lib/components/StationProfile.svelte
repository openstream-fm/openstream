<script lang="ts">
	import {
		mdiApple,
		mdiFacebook,
		mdiGooglePlay,
		mdiInstagram,
		mdiPhoneOutline,
		mdiTwitch,
		mdiTwitter,
		mdiWeb,
		mdiWhatsapp,
		mdiYoutube
	} from '@mdi/js';
	import NullEmail from './Form/Nullable/NullEmail.svelte';
	import NullTextField from './Form/Nullable/NullTextField.svelte';
	import StationPictureField from './Form/StationPictureField.svelte';
	import Validator from '$share/formy/Validator.svelte';
	import {
		_email,
		_string,
		_number,
		_phone,
		_url,
		_youtube_url,
		_app_store_url,
		_facebook_url,
		_instagram_url,
		_twitch_url,
		_twitter_url,
    _google_play_url
	} from '$share/formy/validate';
	import CountryField from './Form/CountryField.svelte';
	import type { CountryCode } from '$server/defs/CountryCode';
	import TypeOfContentField from './Form/TypeOfContentField.svelte';
	import type { StationTypeOfContent } from '$server/defs/db/StationTypeOfContent';
	import { locale } from '$lib/locale';

	export let account_id: string;
	export let current: {
		picture_id: string | null;

		name: string | null;
		slogan: string | null;
		description: string | null;
		country_code: CountryCode | "";
		type_of_content: StationTypeOfContent | "",

		email: string | null;
		phone: string | null;
		whatsapp: string | null;

		website_url: string | null;
		twitter_url: string | null;
		facebook_url: string | null;
		instagram_url: string | null;
		youtube_url: string | null;
		twitch_url: string | null;

		google_play_url: string | null;
		app_store_url: string | null;
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
  
  .section-logo {
    --validator-message-font-size: 1em;
    --validator-message-margin: 0;
  }
</style>


<div class="section section-logo">
	<div class="section-title">{$locale.station_profile.titles.logo} *</div>
	<div class="fields">
		<div class="field">
			<StationPictureField required {account_id} bind:picture_id={current.picture_id} />
		</div>
	</div>
</div>

<div class="section">
	<div class="section-title">{$locale.station_profile.titles.profile_info}</div>
	<div class="fields">
		<div class="field">
			<NullTextField label="{$locale.station_profile.labels.name} *" trim bind:value={current.name} />
			<Validator value={current.name} fn={_string({ required: true, minlen: 3, maxlen: 40 })} />
		</div>
		<div class="field">
			<NullTextField label={$locale.station_profile.labels.slogan} trim bind:value={current.slogan} />
			<Validator value={current.slogan} fn={_string({ maxlen: 50 })} />
		</div>
		<div class="field">
			<NullTextField
				label={$locale.station_profile.labels.description}
				multiline
				minrows={15}
				maxrows={50}
				maxlength={2000}
				bind:value={current.description}
			/>
			<Validator value={current.description} fn={_string({ maxlen: 2000 })} />
		</div>
		<div class="field">
			<CountryField
				label="{$locale.station_profile.labels.country} *"
				country_names={$locale.countries}
				bind:value={current.country_code}
			/>
			<Validator value={current.country_code} fn={_string({ required: true })} />
		</div>
		<div class="field">
			<TypeOfContentField
				label="{$locale.station_profile.labels.type_of_content} *"
				locale_names={$locale.station_type_of_content}
				bind:value={current.type_of_content}
			/>
			<Validator value={current.type_of_content} fn={_string({ required: true })} />
		</div>

	</div>

</div>

<div class="section">
	<div class="section-title">{$locale.station_profile.titles.contact_info}</div>
	<div class="fields">
		<div class="field">
			<NullEmail label={$locale.station_profile.labels.email} maxlength={40} bind:value={current.email} />
			<Validator value={current.email} fn={_email()} />
		</div>
		<div class="field">
			<NullTextField
				type="tel"
				label={$locale.station_profile.labels.phone}
				trim
				icon={mdiPhoneOutline}
				maxlength={40}
				bind:value={current.phone}
			/>
			<Validator value={current.phone} fn={_phone()} />
		</div>
		<div class="field">
			<NullTextField
				type="tel"
				label={$locale.station_profile.labels.whatsapp}
				trim
				icon={mdiWhatsapp}
				maxlength={40}
				bind:value={current.whatsapp}
			/>
			<Validator value={current.whatsapp} fn={_phone({ whatsapp: true })} />
		</div>
		<div class="field">
			<NullTextField
				type="url"
				label={$locale.station_profile.labels.website}
				trim
				icon={mdiWeb}
				maxlength={150}
				bind:value={current.website_url}
			/>
			<Validator value={current.website_url} fn={_url({ maxlen: 150 })} />
		</div>
	</div>
</div>

<div class="section">
	<div class="section-title">{$locale.station_profile.titles.social}</div>
	<div class="fields">
		<div class="field">
			<NullTextField
				type="url"
				label={$locale.station_profile.labels.twitter}
				maxlength={150}
				trim
				icon={mdiTwitter}
				bind:value={current.twitter_url}
			/>
			<Validator value={current.twitter_url} fn={_twitter_url({ maxlen: 150 })} />
		</div>
		<div class="field">
			<NullTextField
				type="url"
				label={$locale.station_profile.labels.facebook}
				trim
				icon={mdiFacebook}
				maxlength={150}
				bind:value={current.facebook_url}
			/>
			<Validator value={current.facebook_url} fn={_facebook_url({ maxlen: 150 })} />
		</div>
		<div class="field">
			<NullTextField
				type="url"
				label={$locale.station_profile.labels.instagram}
				trim
				icon={mdiInstagram}
				maxlength={150}
				bind:value={current.instagram_url}
			/>
			<Validator value={current.instagram_url} fn={_instagram_url({ maxlen: 150 })} />
		</div>
		<div class="field">
			<NullTextField
				type="url"
				label={$locale.station_profile.labels.youtube}
				trim
				maxlength={150}
				icon={mdiYoutube}
				bind:value={current.youtube_url}
			/>
			<Validator value={current.youtube_url} fn={_youtube_url({ maxlen: 150 })} />
		</div>
		<div class="field">
			<NullTextField
				type="url"
				label={$locale.station_profile.labels.twitch}
				trim
				maxlength={150}
				icon={mdiTwitch}
				bind:value={current.twitch_url}
			/>
			<Validator value={current.twitch_url} fn={_twitch_url({ maxlen: 150 })} />
		</div>
	</div>
</div>

<div class="section">
	<div class="section-title">{$locale.station_profile.titles.apps}</div>
	<div class="fields">
		<div class="field">
			<NullTextField
				type="url"
				label={$locale.station_profile.labels.google_play}
				trim
				icon={mdiGooglePlay}
        maxlength={150}
        bind:value={current.google_play_url}
			/>
      <Validator value={current.google_play_url} fn={_google_play_url({ maxlen: 150 })} />
		</div>
		<div class="field">
			<NullTextField
				type="url"
				label={$locale.station_profile.labels.app_store}
				trim
				icon={mdiApple}
				maxlength={150}
				bind:value={current.app_store_url}
			/>
      <Validator value={current.app_store_url} fn={_app_store_url({ maxlen: 150 })} />
		</div>
	</div>
</div>