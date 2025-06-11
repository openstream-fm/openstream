SELECT
    toStartOfDay(created_at) as date,
    max(connections)
FROM
(
    SELECT
        created_at,
        sum(change) OVER (ORDER BY created_at ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) AS connections
    FROM
    (
        SELECT created_at, 1 AS change FROM openstream.stream_connection_lite where created_at between toDateTime('2024-01-01') and toDateTime('2024-02-01')
        UNION ALL
        SELECT closed_at, -1 AS change FROM openstream.stream_connection_lite where created_at between toDateTime('2024-01-01') and toDateTime('2024-02-01')
    )
)
ORDER BY date
ORDER BY date