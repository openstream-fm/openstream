SELECT
    time,
    active_events
FROM
(
    SELECT
        time,
        sum(change) OVER (ORDER BY time ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) AS active_events
    FROM
    (
        SELECT created_at AS time, 1 AS change FROM openstream.stream_connection_lite where created_at between toDateTime('2024-01-01') and toDateTime('2024-02-01')
        UNION ALL
        SELECT closed_at AS time, -1 AS change FROM openstream.stream_connection_lite where created_at between toDateTime('2024-01-01') and toDateTime('2024-02-01')
    )
)
ORDER BY active_events desc
LIMIT 20