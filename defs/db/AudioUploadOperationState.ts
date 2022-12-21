// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { DateTime } from "../DateTime";

export type AudioUploadOperationState = { state: "pending" } | {
  state: "success";
  commited_at: DateTime;
} | {
  state: "error";
  cancelled_at: DateTime;
  error_display: string;
  error_debug: string;
};
