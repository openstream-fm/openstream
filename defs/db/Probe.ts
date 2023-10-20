// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { DateTime } from "../DateTime";
import type { ProbeResult } from "./ProbeResult";

export type Probe = {
  _id: string;
  station_id: string;
  url: string;
  duration_ms: number;
  created_at: DateTime;
  updated_at: DateTime;
  deleted_at: DateTime | null;
} & ProbeResult;