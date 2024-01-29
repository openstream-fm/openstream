// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { DateTime } from "../DateTime.js";
import type { StationPictureVariantFormat } from "./StationPictureVariantFormat.js";

export type StationPictureVariant = {
  _id: string;
  picture_id: string;
  format: StationPictureVariantFormat;
  size: number;
  size_bytes: number;
  content_type: string;
  data: Uint8Array;
  created_at: DateTime;
  updated_at: DateTime;
};
