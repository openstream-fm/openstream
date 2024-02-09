// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Metadata } from "../../../db/Metadata.js";

export type Payload = {
  email: string;
  phone: string | null | undefined;
  password: string;
  first_name: string;
  last_name: string;
  language?: string;
  user_metadata?: Metadata;
  system_metadata?: Metadata;
};
