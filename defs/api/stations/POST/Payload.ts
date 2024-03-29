// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { CountryCode } from "../../../CountryCode.js";
import type { LangCode } from "../../../LangCode.js";
import type { Metadata } from "../../../db/Metadata.js";
import type { StationFrequency } from "../../../StationFrequency.js";
import type { StationTypeOfContent } from "../../../db/StationTypeOfContent.js";

export type Payload = {
  account_id: string;
  picture_id: string;
  name: string;
  slug: string | null | undefined;
  slogan: string | null | undefined;
  description: string | null | undefined;
  type_of_content: StationTypeOfContent;
  country_code: CountryCode;
  lang_code: LangCode;
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
  radiocut_url: string | null | undefined;
  google_play_url: string | null | undefined;
  app_store_url: string | null | undefined;
  frequency: StationFrequency | null | undefined;
  external_relay_url: string | null | undefined;
  user_metadata?: Metadata;
  system_metadata?: Metadata;
};
