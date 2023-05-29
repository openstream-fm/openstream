// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { DateTime } from "../DateTime";
import type { RelaySessionState } from "./RelaySessionState";

export type RelaySessionBase = {
  _id: string;
  station_id: string;
  deployment_id: string;
  target_deployment_id: string;
  state: RelaySessionState;
  transfer_bytes: number;
  closed_at: DateTime | null;
  duration_ms: number | null;
  created_at: DateTime;
  updated_at: DateTime;
};
