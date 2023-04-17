// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { DateTime } from "../DateTime";
import type { Metadata } from "./Metadata";

export interface Account {
  _id: string;
  name: string;
  created_at: DateTime;
  updated_at: DateTime;
  user_metadata: Metadata;
  system_metadata: Metadata;
}
