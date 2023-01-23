// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { DateTime } from "./DateTime";
import type { Metadata } from "./db/Metadata";
import type { StationLimits } from "./StationLimits";

export interface UserPublicStation {
  _id: string;
  name: string;
  limits: StationLimits;
  playlist_is_randomly_shuffled: boolean;
  created_at: DateTime;
  updated_at: DateTime;
  user_metadata: Metadata;
  source_password: string;
}
