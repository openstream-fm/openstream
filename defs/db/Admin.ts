// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Metadata } from "./Metadata";

export interface Admin {
  _id: string;
  name: string;
  email: string;
  password: string | null;
  createdAt: string;
  updatedAt: string;
  systemMetadata: Metadata;
}