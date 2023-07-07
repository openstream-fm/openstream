// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { CountryCode } from "../CountryCode";
import type { Metadata } from "../db/Metadata";
import type { StationFrequency } from "../StationFrequency";
import type { StationTypeOfContent } from "../db/StationTypeOfContent";

export type StationPatch = {
  name?: string;
  picture_id?: string;
  slogan?: string | null;
  description?: string | null;
  type_of_content?: StationTypeOfContent;
  country_code?: CountryCode;
  frequencies?: Array<StationFrequency>;
  email?: string | null;
  phone?: string | null;
  whatsapp?: string | null;
  website_url?: string | null;
  twitter_url?: string | null;
  facebook_url?: string | null;
  instagram_url?: string | null;
  threads_url?: string | null;
  youtube_url?: string | null;
  twitch_url?: string | null;
  tiktok_url?: string | null;
  google_play_url?: string | null;
  app_store_url?: string | null;
  external_relay_url?: string | null;
  user_metadata?: Metadata;
  system_metadata?: Metadata;
};
