<script lang="ts">
	import {
		mdiApple,
		mdiAt,
		mdiContentCut,
		mdiFacebook,
		mdiGooglePlay,
		mdiInstagram,
		mdiPhoneOutline,
		mdiSpotify,
		mdiTwitch,
		mdiTwitter,
		mdiWeb,
		mdiWhatsapp,
		mdiYoutube,
	} from '@mdi/js';
	
	const mdiThreads = "M17.692125 11.1235375C17.58875 11.0739875 17.48375 11.0263 17.377375 10.9806375C17.192125 7.567275 15.327 5.613125 12.195237499999998 5.593125C12.181050000000003 5.5930375 12.1669375 5.5930375 12.152750000000001 5.5930375C10.27955 5.5930375 8.7216375 6.3926125 7.7627500000000005 7.8475874999999995L9.485125 9.0291C10.20145 7.942287499999999 11.32565 7.710600000000001 12.153575 7.710600000000001C12.163137499999998 7.710600000000001 12.1727375 7.710600000000001 12.1822 7.7106875C13.213375 7.7172625 13.991500000000002 8.017075 14.495125 8.60175C14.861625 9.0274125 15.10675 9.615625 15.228125 10.357975C14.313875 10.2025875 13.325125 10.1548125 12.268124999999998 10.2154125C9.2905875 10.386925 7.3763875 12.1234875 7.504950000000001 14.5365C7.570187499999999 15.7605 8.1799625 16.8135 9.221875 17.501375C10.1028 18.082875 11.237375 18.36725 12.4165375 18.302875C13.97375 18.2175 15.195375 17.623375 16.047625 16.537C16.694875 15.712 17.10425 14.642875 17.285 13.295750000000002C18.027125 13.743625000000002 18.577125 14.333 18.880875 15.0415C19.397375 16.245875 19.4275 18.225 17.812625 19.8385C16.39775 21.252 14.697 21.8635 12.1266875 21.882375C9.275525 21.86125 7.119225 20.946875 5.7172625 19.164625C4.4044375 17.49575 3.7259625 15.08525 3.7006499999999996 12C3.7259625 8.914725 4.4044375 6.504200000000001 5.7172625 4.8353375C7.119225 3.0531125 9.2754875 2.13875 12.12665 2.1175625C14.9985 2.1389125 17.192375 3.057675 18.648 4.8485C19.36175 5.7267 19.899875 6.831099999999999 20.254625 8.1187875L22.273 7.580275C21.843 5.995275 21.166375 4.6294625 20.245625 3.4967500000000005C18.3795 1.200835 15.65025 0.0243935 12.1336875 0H12.1196125C8.6102 0.024308749999999997 5.9115125 1.205225 4.0985375 3.5099125000000004C2.4852375 5.5608 1.65305 8.4144625 1.6250875000000002 11.9915625L1.625 12L1.6250875000000002 12.0084375C1.65305 15.5855 2.4852375 18.43925 4.0985375 20.490125C5.9115125 22.79475 8.6102 23.97575 12.1196125 24H12.1336875C15.25375 23.978375 17.453 23.1615 19.26475 21.351375C21.635125 18.98325 21.56375 16.014875 20.7825 14.192625C20.222 12.885874999999999 19.153375 11.824525 17.692125 11.1235375ZM12.305062499999998 16.188375C11.0000625 16.261875 9.6443 15.676125000000003 9.57745 14.421499999999998C9.5279 13.49125 10.239475 12.45325 12.38515 12.3296C12.630875 12.315425 12.872 12.308500000000002 13.108875000000001 12.308500000000002C13.88825 12.308500000000002 14.617375 12.3842125 15.280250000000002 12.529125C15.033000000000001 15.616875 13.58275 16.11825 12.305062499999998 16.188375Z"
	const mdiTiktok = "M19.589 6.686a4.793 4.793 0 0 1-3.77-4.245V2h-3.445v13.672a2.896 2.896 0 0 1-5.201 1.743l-.002-.001.002.001a2.895 2.895 0 0 1 3.183-4.51v-3.5a6.329 6.329 0 0 0-5.394 10.692 6.33 6.33 0 0 0 10.857-4.424V8.687a8.182 8.182 0 0 0 4.773 1.526V6.79a4.831 4.831 0 0 1-1.003-.104z";
	const mdiAdMob = "M11.46.033h-.052A11.993 11.993 0 0 0 0 11.922v.052c0 7.475 6.563 11.928 11.447 11.928h.17a3.086 3.086 0 0 0 3.125-3.047c0-1.693-1.433-2.917-3.152-2.917h-.039a6.016 6.016 0 0 1-5.508-6.368v-.052a6.016 6.016 0 0 1 5.573-5.509c1.719 0 3.125-1.237 3.125-2.917A3.086 3.086 0 0 0 11.604.02h-.143zm2.031.026a3.516 3.516 0 0 1 1.746 3.021 3.386 3.386 0 0 1-1.928 3.047c2.865.6 4.532 3.126 4.688 5.378v7.684a3.49 3.49 0 0 1 6.003.026v-7.736A12.046 12.046 0 0 0 13.491.045zm7.475 17.932a2.995 2.995 0 1 0 .04 0z";
	const mdiAdMobViewBox = "-6 -6 36 36";

	import NullEmail from '$share/Form/Nullable/NullEmail.svelte';
	import NullTextField from '$share/Form/Nullable/NullTextField.svelte';
	import StationPictureField from '$share/Form/StationPictureField.svelte';
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
		_threads_url,
		_twitch_url,
		_twitter_url,
    _google_play_url,
		_tiktok_url,
		_spotify_url,
		_radiocut_url,
		_station_slug,
		_pattern,
	} from '$share/formy/validate';
	import { VALIDATE_STATION_DESC_MAX_LEN, VALIDATE_STATION_EMAIL_MAX_LEN, VALIDATE_STATION_NAME_MAX_LEN, VALIDATE_STATION_NAME_MIN_LEN, VALIDATE_STATION_PHONE_MAX_LEN, VALIDATE_STATION_SLOGAN_MAX_LEN, VALIDATE_STATION_SLUG_MAX_LEN, VALIDATE_STATION_URLS_MAX_LEN, VALIDATE_STATION_WHATSAPP_MAX_LEN } from "$server/defs/constants";
	import CountryField from '$share/Form/CountryField.svelte';
	import LangField from '$share/Form/LangField.svelte';
	import type { CountryCode } from '$server/defs/CountryCode';
	import TypeOfContentField from '$share/Form/TypeOfContentField.svelte';
	import type { StationTypeOfContent } from '$server/defs/db/StationTypeOfContent';
	import { locale } from '$lib/locale';
	import type { StationFrequency } from '$server/defs/StationFrequency';
	import FrequencyField from '$share/Form/FrequencyField.svelte';
	import ColorField from '$share/Form/ColorField.svelte';
	import BooleanField from '$share/Form/BooleanField.svelte';
	import { display_fly_enter } from '$share/display_transitions';
	import type { LangCode } from '$server/defs/LangCode';
	import type { GooglePlayLang } from './google-play-lang';
	import GooglePlayLangField from './GooglePlayLangField.svelte';
	import { tooltip } from '$share/tooltip';

	export let account_id: string;
	export let station_id: string | null;
	export let current: {
		picture_id: string | null | undefined;

		name: string | null | undefined;
		slug: string | null | undefined;

		slogan: string | null | undefined;
		description: string | null | undefined;
		
		country_code: CountryCode | "";
		lang_code: LangCode | "",

		type_of_content: StationTypeOfContent | "",
		frequency: StationFrequency | null | undefined,

		email: string | null | undefined;
		phone: string | null | undefined;
		whatsapp: string | null | undefined;

		website_url: string | null | undefined;
		twitter_url: string | null | undefined;
		facebook_url: string | null | undefined;
		instagram_url: string | null | undefined;
		threads_url: string | null | undefined;
		youtube_url: string | null | undefined;
		twitch_url: string | null | undefined;
		tiktok_url: string | null | undefined;
		spotify_url: string | null | undefined;
		radiocut_url: string | null | undefined,

		google_play_url: string | null | undefined;
		app_store_url: string | null | undefined;
		
		user_metadata: {
			mob_app: {
				base_color: string
				icon_bg_color: string
				icon_rounded: boolean
				
				// banners
				ads: boolean
				interstitial_ads: boolean

				admob_app_id: string | null | undefined
				admob_banner_id: string | null | undefined
				admob_interstitial_id: string | null | undefined
				
				google_play_console_id: string | null | undefined,

				google_play_title: string | null | undefined
				google_play_subtitle: string | null | undefined
				google_play_description: string | null | undefined
				google_play_lang: GooglePlayLang | ""
			}
		}
	};
	
	export let advanced_open: boolean = false;

	const _validate_admob_app_id = (value: string | null | undefined): string | null => {
		if(value == null) return null;
		if(!/^ca\-app\-pub\-[0-9]{16}\~[0-9]{10}$/.test(value)) {
			return $locale.station_profile.validation.admob_app_id_pattern;
		}
		return null;
	}

	const _validate_admob_ad_id = (value: string | null | undefined): string | null => {
		if(value == null) return null;
		if(!/^ca\-app\-pub\-[0-9]{16}\/[0-9]{10}$/.test(value)) {
			return $locale.station_profile.validation.admob_banner_id_pattern;
		}
		return null;
	}
</script>


<style>
	.section + .section {
		margin-top: 4rem;
	}

	.section-title {
		font-size: 1.25rem;
		font-weight: var(--font-bold);
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

	.section-advanced {
		display: flex;
		flex-direction: column;
	}

	.section-advanced-title {
		outline: none;
		appearance: none;
		border: 0;
		background: none;
		cursor: default;
		align-self: center;
	}

	.section-advanced-title:not(.open) {
		cursor: pointer;
		text-decoration: underline;
	}

	.section-advanced-fields {
		margin-top: 2rem;	
	}

	.section-advanced-fields:not(.open) {
		display: none;
	}

	.advanced-sep {
		height: 1px;
		width: 80%;
		margin: 0 auto;
		background: #ccc;
	}

	.sep-one {
		margin: 2rem auto;
	}

	.field-explain {
		color: #999;
		font-size: 0.8rem;
		margin: 0.5rem 0.25rem;
	}

	.field.with-toggle {
		position: relative;
		padding-inline-end: 2.5rem;
	}

	.field-toggle {
		position: absolute;
		right: 0;
		top: 50%;
		transform: translateY(-50%);
		display: flex;
		flex: none;
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
	<!-- TODO: locale -->
	<div class="section-title">Identifiers</div>
	<div class="fields">
		<div class="field">
			<!-- TODO: locale -->
			<NullTextField
				label="Openstream @name"
				trim
				maxlength={VALIDATE_STATION_SLUG_MAX_LEN}
				bind:value={current.slug}
				icon={mdiAt}
			/>
			<div class="field-explain">
				<!-- TODO: locale -->
				This will be a unique identifier for this station, it can contain letters, numbers, dashes, underscores and dots.
				It can be changed at any time but it will be linked to the station even after it's changed.
			</div>
			<Validator value={current.slug} fn={_station_slug({ station_id })} />
		</div>
	</div>
</div>

<div class="section">
	<div class="section-title">{$locale.station_profile.titles.profile_info}</div>
	<div class="fields">
		<div class="field">
			<NullTextField
				label="{$locale.station_profile.labels.name} *"
				trim
				maxlength={VALIDATE_STATION_NAME_MAX_LEN}
				bind:value={current.name}
			/>
			<Validator value={current.name} fn={_string({
					required: true,
					minlen: VALIDATE_STATION_NAME_MIN_LEN,
					maxlen: VALIDATE_STATION_NAME_MAX_LEN,
				})}
			/>
		</div>
		<div class="field">
			<NullTextField
				label={$locale.station_profile.labels.slogan}
				trim
				maxlength={VALIDATE_STATION_SLOGAN_MAX_LEN}
				bind:value={current.slogan}
			/>
			<Validator value={current.slogan} fn={_string({
					maxlen: VALIDATE_STATION_SLOGAN_MAX_LEN
				})}
			/>
		</div>
		<div class="field">
			<NullTextField
				label={$locale.station_profile.labels.description}
				multiline
				minrows={15}
				maxrows={50}
				maxlength={VALIDATE_STATION_DESC_MAX_LEN}
				bind:value={current.description}
			/>
			<Validator value={current.description} fn={_string({
				maxlen: VALIDATE_STATION_DESC_MAX_LEN,
			})}
			/>
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
			<LangField
				label="{$locale.station_profile.labels.language} *"
				lang_names={$locale.langs}
				bind:value={current.lang_code}
			/>
			<Validator value={current.lang_code} fn={_string({ required: true })} />
		</div>


		<div class="field">
			<TypeOfContentField
				label="{$locale.station_profile.labels.type_of_content} *"
				locale_names={$locale.type_of_content}
				bind:value={current.type_of_content}
			/>
			<Validator value={current.type_of_content} fn={_string({ required: true })} />
		</div>
		<div class="field">
			<FrequencyField bind:value={current.frequency} />
		</div>
	</div>

</div>

<div class="section">
	<div class="section-title">{$locale.station_profile.titles.contact_info}</div>
	<div class="fields">
		<div class="field">
			<NullEmail
				label={$locale.station_profile.labels.email}
				maxlength={VALIDATE_STATION_EMAIL_MAX_LEN}
				bind:value={current.email}
			/>
			<Validator value={current.email} fn={_email({
					maxlen: VALIDATE_STATION_EMAIL_MAX_LEN
				})}
			/>
		</div>
		<div class="field">
			<NullTextField
				type="tel"
				label={$locale.station_profile.labels.phone}
				trim
				icon={mdiPhoneOutline}
				maxlength={VALIDATE_STATION_PHONE_MAX_LEN}
				bind:value={current.phone}
			/>
			<Validator value={current.phone} fn={_phone({
				maxlen: VALIDATE_STATION_PHONE_MAX_LEN
			})} />
		</div>
		<div class="field">
			<NullTextField
				type="tel"
				label={$locale.station_profile.labels.whatsapp}
				trim
				icon={mdiWhatsapp}
				maxlength={VALIDATE_STATION_WHATSAPP_MAX_LEN}
				bind:value={current.whatsapp}
			/>
			<Validator value={current.whatsapp} fn={_phone({
					whatsapp: true,
					maxlen: VALIDATE_STATION_WHATSAPP_MAX_LEN
				})}
			/>
		</div>
		<div class="field">
			<NullTextField
				type="url"
				label={$locale.station_profile.labels.website}
				trim
				icon={mdiWeb}
				maxlength={VALIDATE_STATION_URLS_MAX_LEN}
				bind:value={current.website_url}
			/>
			<Validator value={current.website_url} fn={_url({ maxlen: VALIDATE_STATION_URLS_MAX_LEN })} />
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
				maxlength={VALIDATE_STATION_URLS_MAX_LEN}
				trim
				icon={mdiTwitter}
				bind:value={current.twitter_url}
			/>
			<Validator value={current.twitter_url} fn={_twitter_url({ maxlen: VALIDATE_STATION_URLS_MAX_LEN })} />
		</div>
		<div class="field">
			<NullTextField
				type="url"
				label={$locale.station_profile.labels.facebook}
				trim
				icon={mdiFacebook}
				maxlength={VALIDATE_STATION_URLS_MAX_LEN}
				bind:value={current.facebook_url}
			/>
			<Validator value={current.facebook_url} fn={_facebook_url({ maxlen: VALIDATE_STATION_URLS_MAX_LEN })} />
		</div>
		<div class="field">
			<NullTextField
				type="url"
				label={$locale.station_profile.labels.instagram}
				trim
				icon={mdiInstagram}
				maxlength={VALIDATE_STATION_URLS_MAX_LEN}
				bind:value={current.instagram_url}
			/>
			<Validator value={current.instagram_url} fn={_instagram_url({ maxlen: VALIDATE_STATION_URLS_MAX_LEN })} />
		</div>
		<div class="field">
			<NullTextField
				type="url"
				label={$locale.station_profile.labels.threads}
				trim
				icon={mdiThreads}
				maxlength={VALIDATE_STATION_URLS_MAX_LEN}
				bind:value={current.threads_url}
			/>
			<Validator value={current.threads_url} fn={_threads_url({ maxlen: VALIDATE_STATION_URLS_MAX_LEN })} />
		</div>
		<div class="field">
			<NullTextField
				type="url"
				label={$locale.station_profile.labels.youtube}
				trim
				maxlength={VALIDATE_STATION_URLS_MAX_LEN}
				icon={mdiYoutube}
				bind:value={current.youtube_url}
			/>
			<Validator value={current.youtube_url} fn={_youtube_url({ maxlen: VALIDATE_STATION_URLS_MAX_LEN })} />
		</div>
		<div class="field">
			<NullTextField
				type="url"
				label={$locale.station_profile.labels.twitch}
				trim
				maxlength={VALIDATE_STATION_URLS_MAX_LEN}
				icon={mdiTwitch}
				bind:value={current.twitch_url}
			/>
			<Validator value={current.twitch_url} fn={_twitch_url({ maxlen: VALIDATE_STATION_URLS_MAX_LEN })} />
		</div>

		<div class="field">
			<NullTextField
				type="url"
				label={$locale.station_profile.labels.tiktok}
				trim
				maxlength={VALIDATE_STATION_URLS_MAX_LEN}
				icon={mdiTiktok}
				bind:value={current.tiktok_url}
			/>
			<Validator value={current.tiktok_url} fn={_tiktok_url({ maxlen: VALIDATE_STATION_URLS_MAX_LEN })} />
		</div>

		<div class="field">
			<NullTextField
				type="url"
				label={$locale.station_profile.labels.spotify}
				trim
				maxlength={VALIDATE_STATION_URLS_MAX_LEN}
				icon={mdiSpotify}
				bind:value={current.spotify_url}
			/>
			<Validator value={current.spotify_url} fn={_spotify_url({ maxlen: VALIDATE_STATION_URLS_MAX_LEN })} />
		</div>

		<div class="field">
			<NullTextField
				type="url"
				label={$locale.station_profile.labels.radiocut}
				trim
				maxlength={VALIDATE_STATION_URLS_MAX_LEN}
				icon={mdiContentCut}
				bind:value={current.radiocut_url}
			/>
			<Validator value={current.radiocut_url} fn={_radiocut_url({ maxlen: VALIDATE_STATION_URLS_MAX_LEN })} />
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
        maxlength={VALIDATE_STATION_URLS_MAX_LEN}
        bind:value={current.google_play_url}
			/>
      <Validator value={current.google_play_url} fn={_google_play_url({ maxlen: VALIDATE_STATION_URLS_MAX_LEN })} />
		</div>
		<div class="field">
			<NullTextField
				type="url"
				label={$locale.station_profile.labels.app_store}
				trim
				icon={mdiApple}
				maxlength={VALIDATE_STATION_URLS_MAX_LEN}
				bind:value={current.app_store_url}
			/>
      <Validator value={current.app_store_url} fn={_app_store_url({ maxlen: VALIDATE_STATION_URLS_MAX_LEN })} />
		</div>
	</div>
</div>

<div class="section section-advanced">
	<button class="section-title section-advanced-title" class:open={advanced_open} on:click|preventDefault={() => advanced_open = true}>
		{$locale.station_profile.titles.advanced}
	</button>

	<div class="fields section-advanced-fields" class:open={advanced_open} use:display_fly_enter={{ show: advanced_open, start: false, duration: 300, y: -25 }}>

		<div class="field">
			<!-- TODO: locale -->
			<GooglePlayLangField
				label={"App language for Google Play"}
				bind:value={current.user_metadata.mob_app.google_play_lang}
			/>
		</div>

		<div class="field">
			<!-- TODO: locale -->
			<NullTextField
				icon={mdiGooglePlay}
				label={"App title for Google Play"}
				bind:value={current.user_metadata.mob_app.google_play_title}
			/>
			<Validator value={current.user_metadata.mob_app.google_play_title} fn={_string({ maxlen: 30 })} />
		</div>

		<div class="field">
			<!-- TODO: locale -->
			<NullTextField
				icon={mdiGooglePlay}	
				label={"App subtitle for Google Play"}
				bind:value={current.user_metadata.mob_app.google_play_subtitle}
			/>
			<Validator value={current.user_metadata.mob_app.google_play_subtitle} fn={_string({ maxlen: 80 })} />
		</div>

		<div class="field">
			<NullTextField
				icon={mdiGooglePlay}
				label={"App description for Google Play"}
				multiline
				minrows={5}
				bind:value={current.user_metadata.mob_app.google_play_description}
			/>
			<Validator value={current.user_metadata.mob_app.google_play_description} fn={_string({ maxlen: 4000 })} />
		</div>

		<div class="advanced-sep sep-one" />

		<div class="field">
			<ColorField
				label={$locale.station_profile.labels.mob_app_base_color}
				bind:value={current.user_metadata.mob_app.base_color}
			/>
		</div>

		<div class="field">
			<ColorField
				label={$locale.station_profile.labels.mob_app_icon_bg_color}
				bind:value={current.user_metadata.mob_app.icon_bg_color}
			/>
		</div>

		<div class="field">
			<BooleanField
				label={$locale.station_profile.labels.mob_app_icon_round}
				bind:value={current.user_metadata.mob_app.icon_rounded}
			/>
		</div>

		<div class="advanced-sep" />

		<div class="field">
			<!-- TODO: locale -->
			<NullTextField
				icon={mdiGooglePlay}
				trim
				label={"Google Play Console Id"}
				bind:value={current.user_metadata.mob_app.google_play_console_id}
			/>
			<!-- TODO: locale -->
			<Validator value={current.user_metadata.mob_app.google_play_console_id} fn={_pattern({
				maxlen: 50,
				regex: /^[0-9]+$/,
				message: () => "Play Console Id must be a number",
			})} />
		</div>

		<div class="field">
			<NullTextField
				icon={mdiAdMob}
				icon_viewbox={mdiAdMobViewBox}
				trim
				label={$locale.station_profile.labels.mob_app_admob_app_id}
				bind:value={current.user_metadata.mob_app.admob_app_id}
			/>
			<Validator value={current.user_metadata.mob_app.admob_app_id} fn={_validate_admob_app_id} />
		</div>

		<div class="field with-toggle">
			<NullTextField
				icon={mdiAdMob}
				icon_viewbox={mdiAdMobViewBox}
				trim
				label={$locale.station_profile.labels.mob_app_admob_banner_id}
				bind:value={current.user_metadata.mob_app.admob_banner_id}
			/>
			<Validator value={current.user_metadata.mob_app.admob_banner_id} fn={_validate_admob_ad_id} />

			<div
				class="field-toggle"
				use:tooltip={
				// TODO: locale
				"Enable banner ads"
				}
			>
				<BooleanField bind:value={current.user_metadata.mob_app.ads} />
			</div>
		</div>


		<div class="field with-toggle">
			<NullTextField
				icon={mdiAdMob}
				icon_viewbox={mdiAdMobViewBox}
				trim
				label={
					// TODO: locale
					"Admob Interstitial Id"
				}
				bind:value={current.user_metadata.mob_app.admob_interstitial_id}
			/>
			<Validator value={current.user_metadata.mob_app.admob_interstitial_id} fn={_validate_admob_ad_id} />

			<div
				class="field-toggle"
				use:tooltip={
					// TODO: locale
					"Enable interstitial ads"
				}
			>
				<BooleanField bind:value={current.user_metadata.mob_app.interstitial_ads} />
			</div>
		</div>
	</div>
</div>