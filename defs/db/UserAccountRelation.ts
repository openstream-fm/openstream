// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { DateTime } from "../DateTime.js";
import type { UserAccountRelationKind } from "./UserAccountRelationKind.js";

export type UserAccountRelation = {
  _id: string;
  user_id: string;
  account_id: string;
  kind: UserAccountRelationKind;
  created_at: DateTime;
};
