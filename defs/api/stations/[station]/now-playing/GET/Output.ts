// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.

export type Output = { kind: "none"; start_on_connect: boolean } | {
  kind: "live";
  title: string | null;
  artist: string | null;
} | {
  kind: "playlist";
  file_id: string;
  filename: string;
  title: string | null;
  artist: string | null;
};
