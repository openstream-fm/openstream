// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { CountryCode } from "../CountryCode";
import type { DateTime } from "../DateTime";

export interface StreamConnectionLite {
  _id: string;
  s: string;
  o: boolean;
  i: string;
  c: CountryCode | null;
  d: DateTime;
}