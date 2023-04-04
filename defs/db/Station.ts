// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { DateTime } from "../DateTime";
import type { Metadata } from "./Metadata";
import type { StationLimits } from "../StationLimits";

export interface Station {
  _id: string;
  account_id: string;
  name: string;
  limits: StationLimits;
  created_at: DateTime;
  updated_at: DateTime;
  playlist_is_randomly_shuffled: boolean;
  user_metadata: Metadata;
  system_metadata: Metadata;
  source_password: string;
}
