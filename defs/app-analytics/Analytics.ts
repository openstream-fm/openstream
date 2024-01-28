// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { AnalyticsItem } from "./AnalyticsItem.js";
import type { AnalyticsQueryKind } from "./AnalyticsQueryKind.js";
import type { AnalyticsStation } from "./AnalyticsStation.js";
import type { AppKindVersion } from "./AppKindVersion.js";
import type { CountryCode } from "../CountryCode.js";
import type { DateTime } from "../DateTime.js";
import type { TimezoneDateTime } from "../TimezoneDateTime.js";
import type { YearMonthDay } from "./YearMonthDay.js";
import type { YearMonthDayHour } from "./YearMonthDayHour.js";

export type Analytics = {
  is_now: boolean;
  kind: AnalyticsQueryKind;
  stations: Array<AnalyticsStation>;
  since: TimezoneDateTime;
  until: TimezoneDateTime;
  utc_offset_minutes: number;
  sessions: number;
  ips: number;
  total_duration_ms: number;
  max_concurrent_listeners?: number;
  max_concurrent_listeners_date?: DateTime;
  by_day: Array<AnalyticsItem<YearMonthDay>>;
  by_hour: Array<AnalyticsItem<YearMonthDayHour>> | null;
  by_country: Array<AnalyticsItem<CountryCode | null>>;
  by_station: Array<AnalyticsItem<string>>;
  by_app_kind: Array<AnalyticsItem<string | null>>;
  by_app_version: Array<AnalyticsItem<AppKindVersion>>;
};
