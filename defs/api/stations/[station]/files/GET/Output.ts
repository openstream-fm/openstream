// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { AudioFile } from "../../../../../db/AudioFile";
import type { Paged } from "../../../../../Paged";

export type Output = {
  files: Paged<AudioFile>;
  playlist_is_randomly_shuffled: boolean;
};
