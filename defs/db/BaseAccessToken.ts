// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { DateTime } from "../DateTime";

export interface BaseAccessToken {
  _id: string;
  key: string;
  media_key: string;
  last_used_at: DateTime | null;
  hits: number;
  created_at: DateTime;
  deleted_at: DateTime | null;
}
