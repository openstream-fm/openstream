// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { UserAccountRelationKind } from "../../../../../db/UserAccountRelationKind";

export type Member = {
  _id: string;
  email: string;
  first_name: string;
  last_name: string;
  relation: UserAccountRelationKind;
};
