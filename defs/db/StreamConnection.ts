// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { CountryCode } from "../CountryCode.js";
import type { DateTime } from "../DateTime.js";
import type { Request } from "./http/Request.js";

export type StreamConnection = {
  _id: string;
  station_id: string;
  deployment_id: string;
  transfer_bytes: number | null;
  duration_ms: number | null;
  is_open: boolean;
  created_at: DateTime;
  country_code: CountryCode | null;
  ip: string;
  is_external_relay_redirect: boolean;
  _manually_closed: boolean;
  request: Request;
  last_transfer_at: DateTime;
  closed_at: DateTime | null;
};
