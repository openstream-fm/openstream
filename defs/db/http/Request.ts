// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Headers } from "./Headers";
import type { Method } from "./Method";
import type { SocketAddr } from "./SocketAddr";
import type { Uri } from "./Uri";
import type { UserAgent } from "../../UserAgent";
import type { Version } from "./Version";

export interface Request {
  remote_ip: string;
  local_addr: SocketAddr;
  version: Version;
  method: Method;
  uri: Uri;
  headers: Headers;
  user_agent: UserAgent;
}
