// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { UserAgent } from "./UserAgent";

export interface Device {
  _id: string;
  is_current: boolean;
  ip: string;
  ua: UserAgent;
  user_id: string | null;
  admin_id: string | null;
}
