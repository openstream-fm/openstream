// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { DateTime } from "../DateTime";
import type { Request } from "./http/Request";

export type MediaSessionKind = {
  kind: "playlist";
  last_audio_file_id: string | null;
  last_audio_chunk_i: number;
  last_audio_chunk_skip_parts: number;
  last_audio_chunk_date: DateTime;
} | { kind: "live"; request: Request };
