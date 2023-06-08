// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { CountryCode } from "../CountryCode";
import type { DateTime } from "../DateTime";

export type StreamConnectionLite = {
  _id: string;
  st: string;
  op: boolean;
  ip: string;
  cc: CountryCode | null;
  du: number | null;
  by: number | null;
  br: string | null;
  do: string | null;
  os: string | null;
  ca: DateTime;
};
