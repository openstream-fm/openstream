// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { ShowQuery } from "./ShowQuery";
import type { SortQuery } from "./SortQuery";

export type Query = {
  show?: ShowQuery;
  sort?: SortQuery;
  limit?: number;
  skip?: number;
  stations?: Array<string>;
};
