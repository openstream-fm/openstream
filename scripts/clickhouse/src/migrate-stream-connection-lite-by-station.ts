import { MongoClient } from "mongodb";
import { createClient } from "@clickhouse/client";
import { Readable } from "stream";

const mongo = await MongoClient.connect("mongodb://localhost:27017/openstream");
const clickhouse = createClient({ database: "openstream" });

const table = `stream_connection_lite`

await clickhouse.query({
   query: `drop table if exists ${table}`,
})

await clickhouse.query({
  query: `
    create table if not exists ${table} (
      id String,
      ip IPv4,
      bytes UInt64,
      country_code FixedString(2),
      station_id String,
      is_open Boolean,
      browser Nullable(String),
      os Nullable(String),
      domain Nullable(String),
      duration_ms UInt32,
      created_at DateTime,
      closed_at DateTime,
      index idx_created_at created_at type minmax granularity 1,
      index idx_closed_at closed_at type minmax granularity 1,
    )
    engine = MergeTree()
    order by (station_id, created_at)
    partition by toYYYYMM(created_at)
  `
});

await clickhouse.query({
  query: `drop table if exists ${table}_concurrent`,
});

await clickhouse.query({
  query: `
    create table if not exists ${table}_concurrent
    (
      station_id String,
      country_code FixedString(2),
      domain Nullable(String),
      os Nullable(String),
      browser Nullable(String),
      datetime DateTime,
      delta Int8,
      index datetime_idx datetime type minmax granularity 1
    )
    engine = MergeTree()
    order by (station_id, datetime)
    partition by toYYYYMM(datetime)
  `
});

await clickhouse.query({
  query: `drop view if exists mv_${table}_concurrent_created`,
});
await clickhouse.query({
  query: `
    create materialized view if not exists
      mv_${table}_concurrent_created
    to
      ${table}_concurrent
    as
      select
        station_id,
        country_code,
        domain,
        os,
        browser,
        created_at as datetime,
        1 as delta
      from
        ${table}
    `,
});

await clickhouse.query({
  query: `drop view if exists mv_${table}_concurrent_closed`,
});

await clickhouse.query({
  query: `
    create materialized view if not exists 
      mv_${table}_concurrent_closed
    to
      ${table}_concurrent
    as
      select
        station_id,
        country_code,
        domain,
        os,
        browser,
        closed_at as datetime,
        -1 as delta
      from
        ${table}
    `,
});

const cl = mongo.db("openstream").collection("stream_connections_lite");
const total = await cl.estimatedDocumentCount();
console.log(`> migrating ${total} documents`);

let elapsed = 0;

type Doc = {
  _id: string,
  cc: string,
  st: string,
  ip: string,
  op: boolean,
  du: number,
  by: number,
  os?: string | null,
  do?: string | null,
  br?: string | null,
  ca: Date,
  cl: Date,
}

const cursor = cl.find<Doc>({
  ca: { $ne: null },
  cl: { $ne: null },
  op: false,
  by: { $ne: null },
  du: { $ne: null },
  cc: { $ne: null },
  ip: /([0-9]{1,3}).([0-9]{1,3}).([0-9]{1,3}).([0-9]{1,3})/,
});

type InsertDoc = {
  id: string,
  ip: string,
  country_code: string,
  station_id: string,
  is_open: boolean,
  duration_ms: number,
  bytes: number,
  browser: string | null,
  os: string | null,
  domain: string | null,
  created_at: number,
  closed_at: number,
}

const BATCH_SIZE = 10_000;
let batch: InsertDoc[] = [];

for await (const doc of cursor) {
  batch.push({
    id: doc._id,
    ip: doc.ip,
    country_code: doc.cc,
    station_id: doc.st,
    is_open: doc.op,
    duration_ms: doc.du,
    bytes: doc.by,
    browser: doc.br ?? null,
    os: doc.os ?? null,
    domain: doc.do ?? null,
    created_at: Math.floor(doc.ca.getTime() / 1000),
    closed_at: Math.floor(doc.cl.getTime() / 1000),
  })

  if(batch.length == BATCH_SIZE) {
    await clickhouse.insert<InsertDoc>({
      table,
      format: "JSONEachRow",
      values: Readable.from(batch),
    });

    elapsed += batch.length;
    batch = [];
    console.log(`> ${elapsed} of ${total} (${(elapsed / total * 100).toFixed(2)}%) docs migrated`); 
    continue;
  }
}

if(batch.length !== 0) {
  await clickhouse.insert<InsertDoc>({
    table,
    format: "JSONEachRow",
    values: batch,
  });

  elapsed += batch.length;
  batch = [];
  console.log(`> ${elapsed} of ${total} docs migrated`);
}

await clickhouse.query({
  query: `optimize table ${table} final`,
});

await clickhouse.query({
  query: `optimize table ${table}_concurrent final`,
});


console.log("> Bye!");
process.exit();

