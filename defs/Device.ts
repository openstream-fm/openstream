// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { DateTime } from "./DateTime";
import type { UserAgent } from "./UserAgent";

export type Device = {
  _id: string;
  is_current: boolean;
  ip: string;
  ua: UserAgent;
  created_at: DateTime;
  last_used_at: DateTime | null;
  user_id: string | null;
  admin_id: string | null;
};
