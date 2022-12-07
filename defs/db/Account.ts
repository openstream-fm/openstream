// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { AccountLimits } from "../AccountLimits";
import type { Metadata } from "./Metadata";

export interface Account {
  _id: string;
  name: string;
  ownerId: string;
  limits: AccountLimits;
  createdAt: string;
  updatedAt: string;
  userMetadata: Metadata;
  systemMetadata: Metadata;
}
