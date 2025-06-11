-- stats_query.sql
-- Outputs totals, max concurrent num and date, and grouped by day, month, browser, os, country_code and station_id

-- 1. Totals
select
  'global' as section,
  null as key_month,
  null as key_day,
  null as key_country_code,
  null as key_browser,
  null as key_os,
  null as key_station_id,
  sessions,
  ips,
  duration_ms,
  bytes,
  max_concurrent,
  max_concurrent_date
from (
  select
    count() as sessions,
    uniq(ip) as ips,
    sum(duration_ms) as duration_ms,
    sum(bytes) as bytes
  from openstream.stream_connection_lite
  where (created_at >= toDateTime('2022-01-01 00:00:00')) and (created_at < toDateTime('2026-01-01 00:00:00'))
) s
left join (
  select
    max(connections) as max_concurrent,
    argMax(date, connections) as max_concurrent_date
  from (
    select
      date,
      sum(change) over (order by date rows between unbounded preceding and current row) as connections
    from (
      select created_at as date, 1 as change from openstream.stream_connection_lite where (created_at >= toDateTime('2022-01-01 00:00:00')) and (created_at < toDateTime('2026-01-01 00:00:00'))
      union all
      select closed_at as date, -1 as change from openstream.stream_connection_lite where (created_at >= toDateTime('2022-01-01 00:00:00')) and (created_at < toDateTime('2026-01-01 00:00:00'))
    )
  )
) c
on true

union all

-- 3. By date (with max concurrent)
select
  'by_day' as section,
  null as key_month,
  toNullable(key) as key_day,
  null as key_country_code,
  null as key_browser,
  null as key_os,
  null as key_station_id,
  
  sessions,
  ips,
  duration_ms,
  bytes,
  max_concurrent,
  max_concurrent_date
from (
  select
    toStartOfDay(created_at) as key,
    count() as sessions,
    uniq(ip) as ips,
    sum(duration_ms) as duration_ms,
    sum(bytes) as bytes
  from openstream.stream_connection_lite
  where (created_at >= toDateTime('2022-01-01 00:00:00')) and (created_at < toDateTime('2026-01-01 00:00:00'))
  group by key
) s
left join (
  select
    key,
    max(connections) as max_concurrent,
    argMax(datetime, connections) as max_concurrent_date
  from (
    select
      key,
      datetime,
      sum(change) over (partition by key order by datetime rows between unbounded preceding and current row) as connections
    from (
      select toStartOfDay(created_at) as key, created_at as datetime, 1 as change from openstream.stream_connection_lite where (created_at >= toDateTime('2022-01-01 00:00:00')) and (created_at < toDateTime('2026-01-01 00:00:00'))
      union all
      select toStartOfDay(created_at) as key, closed_at as datetime, -1 as change from openstream.stream_connection_lite where (created_at >= toDateTime('2022-01-01 00:00:00')) and (created_at < toDateTime('2026-01-01 00:00:00'))
    )
  )
  group by key
) c using key
order by key_day

union all

select
  'by_month' as section,
  toNullable(key) as key_month,
  null as key_day,
  null as key_country_code,
  null as key_browser,
  null as key_os,
  null as key_station_id,
  sessions,
  ips,
  duration_ms,
  bytes,
  max_concurrent,
  max_concurrent_date
from (
  select
    toStartOfMonth(created_at) as key,
    count() as sessions,
    uniq(ip) as ips,
    sum(duration_ms) as duration_ms,
    sum(bytes) as bytes
  from openstream.stream_connection_lite
  where (created_at >= toDateTime('2022-01-01 00:00:00')) and (created_at < toDateTime('2026-01-01 00:00:00'))
  group by key
) s
left join (
  select
    key,
    max(connections) as max_concurrent,
    argMax(datetime, connections) as max_concurrent_date
  from (
    select
      key,
      datetime,
      sum(change) over (partition by key order by datetime rows between unbounded preceding and current row) as connections
    from (
      select toStartOfMonth(created_at) as key, created_at as datetime, 1 as change from openstream.stream_connection_lite where (created_at >= toDateTime('2022-01-01 00:00:00')) and (created_at < toDateTime('2026-01-01 00:00:00'))
      union all
      select toStartOfMonth(created_at) as key, closed_at as datetime, -1 as change from openstream.stream_connection_lite where (created_at >= toDateTime('2022-01-01 00:00:00')) and (created_at < toDateTime('2026-01-01 00:00:00'))
    )
  )
  group by key
) c using key
order by key_month

union all

-- 4. By browser (with max concurrent)
select
  'by_browser' as section,
  null as key_month,
  null as key_day,
  null as key_country_code,
  toNullable(key) as key_browser,
  null as key_os,
  null as key_station_id,
  sessions,
  ips,
  duration_ms,
  bytes,
  max_concurrent,
  max_concurrent_date
from (
  select
    browser as key,
    count() as sessions,
    uniq(ip) as ips,
    sum(duration_ms) as duration_ms,
    sum(bytes) as bytes
  from openstream.stream_connection_lite
  where (created_at >= toDateTime('2022-01-01 00:00:00')) and (created_at < toDateTime('2026-01-01 00:00:00'))
  group by key
) s
left join (
  select
    key,
    max(connections) as max_concurrent,
    argMax(datetime, connections) as max_concurrent_date
  from (
    select
      key,
      datetime,
      sum(change) over (partition by key order by datetime rows between unbounded preceding and current row) as connections
    from (
      select browser as key, created_at as datetime, 1 as change from openstream.stream_connection_lite where (created_at >= toDateTime('2022-01-01 00:00:00')) and (created_at < toDateTime('2026-01-01 00:00:00'))
      union all
      select browser as key, closed_at as datetime, -1 as change from openstream.stream_connection_lite where (created_at >= toDateTime('2022-01-01 00:00:00')) and (created_at < toDateTime('2026-01-01 00:00:00'))
    )
  )
  group by key
) c using key
order by sessions desc

union all

-- 5. By os (with max concurrent)
select
  'by_os' as section,
  null as key_month,
  null as key_day,
  null as key_country_code,
  null as key_browser,
  toNullable(key) as key_os,
  null as key_station_id,
  sessions,
  ips,
  duration_ms,
  bytes,
  max_concurrent,
  max_concurrent_date
from (
  select
    os as key,
    count() as sessions,
    uniq(ip) as ips,
    sum(duration_ms) as duration_ms,
    sum(bytes) as bytes
  from openstream.stream_connection_lite
  where (created_at >= toDateTime('2022-01-01 00:00:00')) and (created_at < toDateTime('2026-01-01 00:00:00'))
  group by key
) s
left join (
  select
    key,
    max(connections) as max_concurrent,
    argMax(datetime, connections) as max_concurrent_date
  from (
    select
      key,
      datetime,
      sum(change) over (partition by key order by datetime rows between unbounded preceding and current row) as connections
    from (
      select os as key, created_at as datetime, 1 as change from openstream.stream_connection_lite where (created_at >= toDateTime('2022-01-01 00:00:00')) and (created_at < toDateTime('2026-01-01 00:00:00'))
      union all
      select os as key, closed_at as datetime, -1 as change from openstream.stream_connection_lite where (created_at >= toDateTime('2022-01-01 00:00:00')) and (created_at < toDateTime('2026-01-01 00:00:00'))
    )
  )
  group by key
) c using key
order by sessions desc

union all

-- 6. By country_code (with max concurrent)
select
  'by_country_code' as section,
  null as key_month,
  null as key_day,
  toNullable(key) as key_country_code,
  null as key_browser,
  null as key_os,
  null as key_station_id,
  sessions,
  ips,
  duration_ms,
  bytes,
  max_concurrent,
  max_concurrent_date
from (
  select
    country_code as key,
    count() as sessions,
    uniq(ip) as ips,
    sum(duration_ms) as duration_ms,
    sum(bytes) as bytes
  from openstream.stream_connection_lite
  where (created_at >= toDateTime('2022-01-01 00:00:00')) and (created_at < toDateTime('2026-01-01 00:00:00'))
  group by key
) s
left join (
  select
    key,
    max(connections) as max_concurrent,
    argMax(datetime, connections) as max_concurrent_date
  from (
    select
      key,
      datetime,
      sum(change) over (partition by key order by datetime rows between unbounded preceding and current row) as connections
    from (
      select country_code as key, created_at as datetime, 1 as change from openstream.stream_connection_lite where (created_at >= toDateTime('2022-01-01 00:00:00')) and (created_at < toDateTime('2026-01-01 00:00:00'))
      union all
      select country_code as key, closed_at as datetime, -1 as change from openstream.stream_connection_lite where (created_at >= toDateTime('2022-01-01 00:00:00')) and (created_at < toDateTime('2026-01-01 00:00:00'))
    )
  )
  group by key
) c using key
order by sessions desc

union all

-- 7. By station_id (with max concurrent)
select
  'by_station' as section,
  null as key_month,
  null as key_day,
  null as key_country_code,
  null as key_browser,
  null as key_os,
  toNullable(key) as key_station_id,
  sessions,
  ips,
  duration_ms,
  bytes,
  max_concurrent,
  max_concurrent_date
from (
  select
    station_id as key,
    count() as sessions,
    uniq(ip) as ips,
    sum(duration_ms) as duration_ms,
    sum(bytes) as bytes
  from openstream.stream_connection_lite
  where (created_at >= toDateTime('2022-01-01 00:00:00')) and (created_at < toDateTime('2026-01-01 00:00:00'))
  group by key
) s
left join (
  select
    key,
    max(connections) as max_concurrent,
    argMax(datetime, connections) as max_concurrent_date
  from (
    select
      key,
      datetime,
      sum(change) over (partition by key order by datetime rows between unbounded preceding and current row) as connections
    from (
      select station_id as key, created_at as datetime, 1 as change from openstream.stream_connection_lite where (created_at >= toDateTime('2022-01-01 00:00:00')) and (created_at < toDateTime('2026-01-01 00:00:00'))
      union all
      select station_id as key, closed_at as datetime, -1 as change from openstream.stream_connection_lite where (created_at >= toDateTime('2022-01-01 00:00:00')) and (created_at < toDateTime('2026-01-01 00:00:00'))
    )
  )
  group by key
) c using key
order by sessions desc