// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { DateTime } from "../DateTime";

export interface EmailVerificationCode {
  _id: string;
  email: string;
  hash: string;
  used_at: DateTime | null;
  created_at: DateTime;
}