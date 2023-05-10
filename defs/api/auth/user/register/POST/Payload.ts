// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Metadata } from "../../../../../db/Metadata";

export interface Payload {
  plan_id: string;
  email: string;
  password: string;
  phone: string | null;
  first_name: string;
  last_name: string;
  account_name: string;
  user_user_metadata?: Metadata;
  user_system_metadata?: Metadata;
  account_user_metadata?: Metadata;
  account_system_metadata?: Metadata;
  device_id: string;
}
