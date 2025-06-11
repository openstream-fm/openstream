import { MongoClient } from "mongodb";
import { createClient } from "@clickhouse/client";
import { Readable } from "stream";

const mongo = await MongoClient.connect("mongodb://localhost:27017/openstream");
const clickhouse = createClient({ database: "openstream" });

const table = `ws_stats_connection`

console.log(`> creating ${table}`)
await clickhouse.query({
  query: `
    create table if not exists ${table} (
      id String,
      station_id String,
      user_id String,
      ip IPv4,
      app_id String,
      app_version UInt32,
      country_code FixedString(2),
      reconnections UInt32,
      is_open Boolean,
      duration_ms UInt32,
      created_at DateTime,
      closed_at DateTime,
    )
    engine = MergeTree()
    order by (station_id, created_at)
    partition by toYYYYMM(created_at)
  `
});

const cl = mongo.db("openstream").collection("ws_stats_connection");
const total = await cl.estimatedDocumentCount();
console.log(`> migrating ${total} documents`);

let elapsed = 0;

type Doc = {
  _id: string,
  av: number,
  ap: string,
  cc: string,
  st: string,
  ip: string,
  re: number,
  us: string,
  op: boolean,
  du: number,
  ca: Date,
  cl: Date,
}

const cursor = cl.find<Doc>({
  ca: { $ne: null },
  cl: { $ne: null },
  op: false,
  du: { $ne: null },
  av: { $ne: null },
  ap: { $ne: null },
  cc: { $ne: null },
  ip: /([0-9]{1,3}).([0-9]{1,3}).([0-9]{1,3}).([0-9]{1,3})/,
  re: { $ne: null },
  us: { $ne: null },
});

type InsertDoc = {
  id: string,
  user_id: string,
  ip: string,
  app_id: string,
  app_version: number,
  country_code: string,
  station_id: string,
  reconnections: number,
  is_open: boolean,
  duration_ms: number,
  created_at: number,
  closed_at: number,
}

const BATCH_SIZE = 10_000;
let batch: InsertDoc[] = [];

for await (const doc of cursor) {
  batch.push({
    id: doc._id,
    user_id: doc.us,
    ip: doc.ip,
    app_id: doc.ap,
    app_version: doc.av,
    country_code: doc.cc,
    station_id: doc.st,
    reconnections: doc.re,
    is_open: doc.op,
    duration_ms: doc.du,
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
    console.log(`> ${elapsed} of ${total} (${Math.floor(elapsed / total * 100).toFixed(2)}%) docs migrated`); 
    continue;
  }
}

if(batch.length !== 0) {
  await clickhouse.insert<InsertDoc>({
    table: "${table}",
    format: "JSONEachRow",
    values: Readable.from(batch),
  });

  elapsed += batch.length;
  batch = [];
  console.log(`> ${elapsed} of ${total} docs migrated`);
}

await clickhouse.query({
  query: `optimize table ${table} final`,
});

console.log("> Bye!");
process.exit();