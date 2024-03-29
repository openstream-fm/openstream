// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { PaginationQs } from "../../../qs/PaginationQs.js";
import type { ShowQuery } from "./ShowQuery.js";
import type { SortQuery } from "./SortQuery.js";

export type Query = {
  show?: ShowQuery;
  sort?: SortQuery;
  stations?: Array<string>;
} & PaginationQs;
