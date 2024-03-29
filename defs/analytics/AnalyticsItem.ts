// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { DateTime } from "../DateTime.js";

export type AnalyticsItem<K> = {
  key: K;
  sessions: number;
  ips: number;
  total_duration_ms: number;
  total_transfer_bytes: number;
  max_concurrent_listeners?: number;
  max_concurrent_listeners_date?: DateTime;
};
