// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { AnalyticsItem } from "./AnalyticsItem";
import type { AnalyticsStation } from "./AnalyticsStation";
import type { CountryCode } from "../CountryCode";
import type { YearMonth } from "./YearMonth";
import type { YearMonthDay } from "./YearMonthDay";

export type Analytics = {
  stations: Array<AnalyticsStation>;
  since: /** time::DateTime */ string;
  until: /** time::DateTime
     */
    string;
  utc_offset_minutes: number;
  sessions: number;
  ips: number;
  total_duration_ms: number;
  by_month: Array<AnalyticsItem<YearMonth>>;
  by_day: Array<AnalyticsItem<YearMonthDay>>;
  by_hour: Array<AnalyticsItem<number>>;
  by_browser: Array<AnalyticsItem<string | null>>;
  by_os: Array<AnalyticsItem<string | null>>;
  by_country: Array<AnalyticsItem<CountryCode | null>>;
  by_station: Array<AnalyticsItem<string>>;
};