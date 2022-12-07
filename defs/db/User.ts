// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Metadata } from "./Metadata";

export interface User {
  _id: string;
  accountIds: Array<string>;
  firstName: string;
  lastName: string;
  email: string;
  password: string | null;
  createdAt: string;
  updatedAt: string;
  userMetadata: Metadata;
  systemMetadata: Metadata;
}
