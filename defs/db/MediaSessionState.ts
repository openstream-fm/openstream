// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { DateTime } from "../DateTime";

export type MediaSessionState = { state: "open" } | {
  state: "closed";
  closed_at: DateTime;
  duration_ms: number;
};