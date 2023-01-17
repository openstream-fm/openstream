// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { DateTime } from "../DateTime";
import type { Request } from "./http/Request";
import type { StreamConnectionState } from "./StreamConnectionState";

export interface StreamConnection {
  _id: string;
  station_id: string;
  request: Request;
  connected_at: DateTime;
  transfer_bytes: number;
  last_transfer_at: DateTime;
  state: StreamConnectionState;
}
