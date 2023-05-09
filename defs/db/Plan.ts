// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { DateTime } from "../DateTime";
import type { PlanLimits } from "./PlanLimits";

export interface Plan {
  _id: string;
  identifier: string;
  display_name: string;
  price: number;
  limits: PlanLimits;
  color: string;
  order: number;
  is_user_selectable: boolean;
  created_at: DateTime;
  updated_at: DateTime;
  deleted_at: DateTime | null;
}
