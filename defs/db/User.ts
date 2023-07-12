// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { DateTime } from "../DateTime";
import type { Metadata } from "./Metadata";

export type User = {
  _id: string;
  first_name: string;
  last_name: string;
  email: string;
  phone: string | null;
  language: string | null;
  password: string | null;
  user_metadata: Metadata;
  system_metadata: Metadata;
  created_at: DateTime;
  updated_at: DateTime;
  deleted_at: DateTime | null;
};
