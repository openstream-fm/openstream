
import { createClient } from "@clickhouse/client";
import { getUnixTime } from "date-fns";
import { promise_map } from "./promise-map.js";

const client = createClient({ database: "openstream" });

const start = new Date("2022-01-01 00:00:00 UTC-3");
const end = new Date("2026-01-01 00:00:00 UTC-3");

// const filter = `true` 
const filter = `station_id = 'qe6fm5ev'`
const table = `openstream.stream_connection_lite`;
const where = `prewhere ${filter} where created_at >= ${getUnixTime(start)} and created_at < ${getUnixTime(end)}`;

const table_concurrent = `${table}_concurrent`;
const where_concurrent = `prewhere ${filter} where datetime >= ${getUnixTime(start)} and datetime < ${getUnixTime(end)}`;

const proc_start = Date.now();

const get_totals = async () => {
  const time_start = Date.now();

  const query = `
    select
      count(*) as sessions,
      uniq(ip) as ips,
      sum(duration_ms) as duration_ms,
      sum(bytes) as bytes
    from 
      ${table}
    ${where}
  `;

  const result = await client.query({
    query,
    format: "JSON",
  })

  const { data } = await result.json<{
    sessions: string;
    ips: string;
    duration_ms: string;
    bytes: string;
  }>();
  const totals = data[0];

  console.log(`> got totals in ${Date.now() - time_start}ms`);
  return totals;
}

const get_max_concurrent = async () => {
  // use max-conurrent.sql
  const time_start = Date.now();

  const query = `
    select
      datetime,
      connections
    from
    (
      select
        datetime,
        sum(delta) over (order by datetime rows between unbounded preceding and current row) as connections
      from
    )
    order by connections desc
    limit 1
  `;

  const result = await client.query({
    query,
    format: "JSON",
  })

  const { data } = await result.json<{
    datetime: string;
    connections: number;
  }>();

  const row = data[0];

  console.log(`> got concurrent totals in ${Date.now() - time_start}ms`);

  return row;
}

const get_totals_by = async ({
  by,
  sort = "sessions desc",
}: {
  by: string,
  sort?: string,
}) => {
  const time_start = Date.now();

  const query = `
    select
      ${by} as key,
      count(*) as sessions,
      uniq(ip) as ips,
      sum(duration_ms) as duration_ms,
      sum(bytes) as bytes
    from 
      ${table}
    where
      ${where}   
    group by 
      key
    order by
      ${sort}
  `;

  const result = await client.query({
    query,
    format: "JSON",
  })

  const { data } = await result.json<{
    key: string;
    sessions: string;
    ips: string;
    duration_ms: string;
    bytes: string;
  }[]>();

  console.log(`> got totals by ${by} in ${Date.now() - time_start}ms`);
  return data;
}

const get_max_concurrent_by = async ({
  by,
}: {
  by: string
}) => {
  const time_start = Date.now();

  const query = `
    select
      key,
      max(connections) as connections
    from
    (
      select
        ${by} as key,
        sum(delta) over (partition by key order by datetime rows between unbounded preceding and current row) as connections
      from
        ${table_concurrent}
      ${where_concurrent}
    )
    group by key
    order by connections desc
  `

  const result = await client.query({
    query,
    format: "JSON",
  })

  const { data } = await result.json<{
    browser: string | null;
    connections: string;
  }[]>();

  console.log(`> got concurrent by ${by} in ${Date.now() - time_start}ms`);
  return data;
}

const get_totals_and_concurrent = async () => {
  const time_start = Date.now();

  const query = `
    with
      totals as (
        select
          count(*) as sessions,
          uniq(ip) as ips,
          sum(duration_ms) as duration_ms,
          sum(bytes) as bytes
        from
          ${table}
        ${where}
      ),
      concurrent as (
        select
          max(connections) as max_concurrent,
          argMax(datetime, connections) as max_concurrent_date
        from (
          select
            datetime,
            sum(delta) over (order by datetime rows between unbounded preceding and current row) as connections
          from 
            ${table_concurrent}
          ${where_concurrent}
        )
      )
    select
      t.sessions,
      t.ips,
      t.duration_ms,
      t.bytes,
      c.max_concurrent,
      c.max_concurrent_date
    from totals t
    cross join concurrent c
  `;

  const result = await client.query({
    query,
    format: "JSON",
  });

  const { data } = await result.json<{
    sessions: string;
    ips: string;
    duration_ms: string;
    bytes: string;
    max_concurrent: string;
    max_concurrent_date: string;
  }[]>();

  const row = data[0];

  console.log(`> got totals and concurrent in ${Date.now() - time_start}ms`);
  return row;
}

const get_totals_and_concurrent_by = async ({
  by,
  by_concurrent = by,
  sort = "sessions desc",
}: {
  by: string,
  by_concurrent?: string,
  sort?: string,
}) => {
  const time_start = Date.now();

  const query = `
    with
      sessions as (
        select
          ${by} as key,
          count(*) as sessions,
          uniq(ip) as ips,
          sum(duration_ms) as duration_ms,
          sum(bytes) as bytes
        from
          ${table}
        ${where}
        group by
          key
      ),
      concurrent as (
        select
          key,
          max(connections) as max_concurrent,
          argMax(datetime, connections) as max_concurrent_date
        from (
          select
            ${by_concurrent} as key,
            datetime,
            sum(delta) over (partition by key order by datetime rows between unbounded preceding and current row) as connections
          from
            ${table_concurrent}
          ${where_concurrent}
        )
        group by key
      )
    select
      s.key,
      s.sessions,
      s.ips,
      s.duration_ms,
      s.bytes,
      c.max_concurrent,
      c.max_concurrent_date
    from sessions s
    left join concurrent c on s.key = c.key
    order by ${sort}
  `;

  const result = await client.query({
    query,
    format: "JSON",
  });

  const { data } = await result.json<{
    key: string;
    sessions: string;
    ips: string;
    duration_ms: string;
    bytes: string;
    max_concurrent: string;
    max_concurrent_date: string;
  }[]>();

  console.log(`> got totals and concurrent by ${by} in ${Date.now() - time_start}ms`);
  return data;
}

// without concurrent
// const result = await promise_map({
//   totals: get_totals(),
//   by_date: get_totals_by({
//     by: "toStartOfDay(created_at)",
//     sort: "key",
//   }),
//   by_browser: get_totals_by({ by: "browser" }),
//   by_os: get_totals_by({ by: "os" }),
//   by_country_code: get_totals_by({ by: "country_code" }),
//   by_station_id: get_totals_by({ by: "station_id" }),
// })

const result = await promise_map({
  totals: get_totals_and_concurrent(),
  by_date: get_totals_and_concurrent_by({
    by: "toStartOfDay(created_at)",
    by_concurrent: "toStartOfDay(datetime)",
    sort: "key",
  }),
  by_browser: get_totals_and_concurrent_by({ by: "browser" }),
  by_os: get_totals_and_concurrent_by({ by: "os" }),
  by_country_code: get_totals_and_concurrent_by({ by: "country_code" }),
  by_station_id: get_totals_and_concurrent_by({ by: "station_id" }),
})

// const result = await promise_map({
//   totals: get_totals(),
//   concurrent: get_max_concurrent(),
  
//   by_date: get_totals_by({
//     by: "toStartOfDay(created_at)",
//     sort: "key",
//   }),
//   by_date_concurrent: get_max_concurrent_by({
//     by: "toStartOfDay(created_at)",
//   }),

//   by_browser: get_totals_by({ by: "browser" }),
//   by_browser_concurrent: get_max_concurrent_by({ by: "browser" }),

//   by_os: get_totals_by({ by: "os" }),
//   by_os_concurrent: get_max_concurrent_by({ by: "os" }),

//   by_country_code: get_totals_by({ by: "country_code" }),
//   by_country_code_concurrent: get_max_concurrent_by({ by: "country_code" }),

//   by_station_id: get_totals_by({ by: "station_id" }),
//   by_station_id_concurrent: get_max_concurrent_by({ by: "station_id" }),
// })

// console.log(result);

console.log(result.totals);

console.log("stats got in ", Date.now() - proc_start);