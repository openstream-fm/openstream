// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { DateTime } from "../DateTime";
import type { UserAccountRelationKind } from "./UserAccountRelationKind";

export interface UserAccountRelation {
  _id: string;
  user_id: string;
  account_id: string;
  kind: UserAccountRelationKind;
  created_at: DateTime;
}
