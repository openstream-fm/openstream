// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { DateTime } from "./DateTime.js";
import type { Metadata } from "./db/Metadata.js";

export type PublicAdmin = {
  _id: string;
  first_name: string;
  last_name: string;
  email: string;
  language: string | null | undefined;
  system_metadata: Metadata;
  created_at: DateTime;
  updated_at: DateTime;
  deleted_at: DateTime | null | undefined;
};
