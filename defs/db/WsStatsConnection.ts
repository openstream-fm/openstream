// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { CountryCode } from "../CountryCode.js";
import type { DateTime } from "../DateTime.js";

export type WsStatsConnection = {
  _id: string;
  st: string;
  sd: string | null | undefined;
  dp: string;
  du: number | null | undefined;
  op: boolean;
  cc: CountryCode | null | undefined;
  ip: string;
  ap: string | null | undefined;
  av: number | null | undefined;
  us: string | null | undefined;
  re: number;
  ca: DateTime;
  cl: DateTime | null | undefined;
  _m: boolean;
};
