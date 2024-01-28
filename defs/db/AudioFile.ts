// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { AudioMetadata } from "./AudioMetadata.js";
import type { DateTime } from "../DateTime.js";

export type AudioFile = {
  _id: string;
  station_id: string;
  sha_256: string;
  len: number;
  duration_ms: number;
  bytes_sec: number;
  chunk_count: number;
  chunk_len: number;
  chunk_duration_ms: number;
  filename: string;
  metadata: AudioMetadata;
  order: number;
  created_at: DateTime;
};
