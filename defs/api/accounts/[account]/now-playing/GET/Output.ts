// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { AudioFile } from "../../../../../db/AudioFile";

export type Output = { kind: "none" } | { kind: "live" } | {
  kind: "playlist";
  file: AudioFile;
};
