// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { DateTime } from "../DateTime";
import type { Metadata } from "./Metadata";

export interface User {
  _id: string;
  accountIds: Array<string>;
  firstName: string;
  lastName: string;
  email: string;
  password: string | null;
  createdAt: DateTime;
  updatedAt: DateTime;
  userMetadata: Metadata;
  systemMetadata: Metadata;
}
