// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { AccountLimits } from "./AccountLimits.js";
import type { DateTime } from "./DateTime.js";
import type { Metadata } from "./db/Metadata.js";

export type UserPublicAccount = {
  _id: string;
  plan_id: string;
  payment_method_id: string | null;
  name: string;
  limits: AccountLimits;
  created_at: DateTime;
  updated_at: DateTime;
  user_metadata: Metadata;
  deleted_at: DateTime | null;
};
