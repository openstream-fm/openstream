// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { AudioFile } from "../../../../../db/AudioFile";

export type Output = { kind: "none"; start_on_connect: boolean } | {
  kind: "live";
} | { kind: "playlist"; file: AudioFile };