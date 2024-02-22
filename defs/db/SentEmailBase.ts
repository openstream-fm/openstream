// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { DateTime } from "../DateTime.js";
import type { SentEmailAddress } from "./SentEmailAddress.js";
import type { SentEmailKind } from "./SentEmailKind.js";

export type SentEmailBase = {
  _id: string;
  to: SentEmailAddress;
  from: SentEmailAddress;
  subject: string;
  text: string;
  html: string;
  reply_to: SentEmailAddress | null | undefined;
  created_at: DateTime;
} & SentEmailKind;
