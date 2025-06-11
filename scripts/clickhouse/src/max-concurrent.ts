import { MongoClient } from "mongodb";
import { createClient } from "@clickhouse/client";
import { Readable } from "stream";

const mongo = await MongoClient.connect("mongodb://localhost:27017/openstream");
const clickhouse = createClient({ database: "openstream" });

const start = Date.now();

const created_at_p = clickhouse.query({
  query: `
  select
    toUnixTimestamp(created_at) as created_at
  from
    stream_connection_lite
  order by
    created_at
  `,
  format: "JSONEachRow",
})

const closed_at_p = clickhouse.query({
  query: `
  select
    toUnixTimestamp(closed_at) as closed_at
  from
    stream_connection_lite
  order by
    closed_at
  `,
  format: "JSONEachRow",
})

const created_at_cursor = await created_at_p;
const closed_at_cursor = await closed_at_p;

const created_at = created_at_cursor.stream<{ created_at: number }>();
const closed_at = closed_at_cursor.stream<{ closed_at: number }>();

const { promise, reject, resolve } = Promise.withResolvers<void>();

let elapsed = 0;

let created_end = false;
let closed_end = false;

created_at.on("data", data => {
  elapsed += data.length;
})

closed_at.on("data", data => {
  elapsed += data.length;
})

setInterval(() => {
  console.log(`> ${elapsed} items processed`);
}, 1_000)

created_at.on("end", () => {
  created_end = true;
  if(closed_end) resolve();
})

created_at.on("error", reject);

closed_at.on("end", () => {
  closed_end = true;
  if(created_end) resolve();
})

closed_at.on("error", reject);

await promise;
console.log(`> processes ${elapsed} items in ${Date.now() - start}ms`);
process.exit()