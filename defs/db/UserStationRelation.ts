// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { DateTime } from "../DateTime";
import type { UserStationRelationKind } from "./UserStationRelationKind";

export interface UserStationRelation {
  _id: string;
  user_id: string;
  station_id: string;
  kind: UserStationRelationKind;
  created_at: DateTime;
}