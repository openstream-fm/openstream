<svelte:options immutable={true} />

<script lang="ts">
  export let data: import("$server/defs/analytics/Analytics").Analytics;
  export let country_names: Record<string, string | undefined>;

  import { CountryCode } from "$server/defs/CountryCode";
  import Mapp from "$share/Map/Map.svelte";
  import { chart } from "$share/apexcharts/apexcharts";
  import type { ApexOptions } from "apexcharts";
  import { add, formatISO, isSameDay, startOfDay } from "date-fns";
  import { locale } from "$share/locale";
  import DataGrid, { DataGridData, DataGridField } from "./DataGrid.svelte";

  const SEC = 1000;
  const MIN = SEC * 60;
  const HOUR = MIN * 60;
  const DAY = HOUR * 24;

  const round = Math.round;
  const floor = Math.round;

  const chartHeight = 350;

  const time = (ms: number) => {
    const days = ms / DAY;
    const hours = (ms % DAY) / HOUR;
    const mins = (ms % HOUR) / MIN;
    const secs = (ms % MIN) / SEC;

    if (days >= 1) {
      return `${floor(days)} days and ${round(hours)} hours`;
    } else if (hours >= 1) {
      return `${floor(hours)} hours and ${round(mins)} minutes`;
    } else if (mins >= 1) {
      return `${floor(mins)} minutes and ${round(secs)} seconds`;
    } else {
      return `${round(secs)} seconds`;
    }
  };

  const by_day_data = (by_day: typeof data.by_day) => {
    const key = (item: { year: number, month: number, day: number }) => `${item.year}-${item.month}-${item.day}`;
    const cache = new Map<string, typeof data.by_day[number]>();
    for(const item of by_day) {
      cache.set(key(item.key), item);
    }

    const sessions: (number | null)[] = [];
    const avg: (number | null)[] = [];
    const dates: Date[] = [];

    const start = startOfDay(new Date(data.since));
    const end = add(startOfDay(new Date(data.until)), { days: 1 });

    let current = start;
        
    do {

      const k = key({ year: current.getFullYear(), month: current.getMonth() + 1, day: current.getDate() });
      const item = cache.get(k);

      dates.push(current);

      if(item == null) {
        sessions.push(null);
        avg.push(null);
      } else {
        sessions.push(item.sessions);
        avg.push(item.total_duration_ms / item.sessions);
      }

      current = add(current, { days: 1 })

    } while(!isSameDay(current, end));

    return { dates, sessions, avg }
  }

  const days_data = by_day_data(data.by_day);
  let days_options: ApexOptions = {
    series: [
      {
        name: "Sessions",
        data: days_data.sessions
      },
      {
        name: "Avg listening time",
        data: days_data.avg
      },
    ],
    chart: {
      animations: {
        enabled: false,
      },
      fontFamily: "inherit",
      height: chartHeight,
      type: "area",
      zoom: {
        enabled: false,
        // autoScaleYaxis: true,
        // type: "x",
      },
      // events: {
      //   beforeZoom: (...args) => {
      //     console.log("beforeZoom", ...args);
      //     return undefined;
      //     const start_date = days_data.dates[0];
      //     const end_date = days_data.dates[days_data.dates.length - 1];
      //     // const maindifference = Number(start_date) - Number(end_date);
      //     // const zoomdifference = xaxis.max - xaxis.min;
      //     // if( zoomdifference > maindifference ) {
      //       return  {
      //         // dont zoom out any further
      //         xaxis: {
      //           min: formatISO(start_date),
      //           max: formatISO(end_date),
      //         }
      //       }
      //     // } else {
      //     //   return {
      //     //     // keep on zooming
      //     //     xaxis: {
      //     //       min: xaxis.min,
      //     //       max: xaxis.max
      //     //     }
      //     //   }
      //     // }
      //   }
      // }
    },
    dataLabels: {
      enabled: false,
    },
    stroke: {
      curve: "smooth"
    },
    xaxis: {
      type: "datetime",
      categories: days_data.dates.map(date => formatISO(date)),
      labels: {
        rotate: -75,
        format: "dd/MM/yyyy",
      }
    },
    yaxis: [
      {
        title: {
          text: "Sessions",
          style: {
           fontSize: "1rem",
           fontWeight: 600,
          }
        },
        labels: {
          formatter: v => String(v ?? 0),
        } 
      }, {
        opposite: true,
        title: {
          text: "Avg listening time",
          style: {
           fontSize: "1rem",
           fontWeight: 600,
          }
        },
        labels: {
          formatter: v => {
            if(v == null) return "-";
            let total_secs = Math.round(v / SEC);
            let mins = Math.floor(total_secs / 60);
            let secs = total_secs % 60;
            const pad = (v: number) => String(v).padStart(2, "0");
            return `${pad(mins)}:${pad(secs)}`;
          }
        }
      }
    ],
    tooltip: {
      x: {
        format: "dd/MM/yyyy",
      },
    }
  };

  let os_options: ApexOptions = {
    chart: {
      type: "bar",
      fontFamily: "inherit",
      height: chartHeight,
      animations: {
        enabled: false,
      },
    },
    dataLabels: {
      enabled: false,
    },
    plotOptions: {
      bar: {
        distributed: true,
        columnWidth: "40%",
      }
    },
    yaxis: {
      title: {
        text: "Sessions",
        style: {
          fontSize: "1rem",
          fontWeight: 600,
        }
      },
    },
    series: [{ 
      name: "Sessions",
      data: data.by_os.map(item => {
        return {
          x: item.key == null ? "Unknown" : item.key,
          y: item.sessions,
        }
      })
    }]
  };

  let browser_options: ApexOptions = {
    chart: {
      type: "bar",
      fontFamily: "inherit",
      height: chartHeight,
      animations: {
        enabled: false,
      },
    },
    dataLabels: {
      enabled: false,
    },
    plotOptions: {
      bar: {
        distributed: true,
        columnWidth: "40%",
      }
    }, 
    yaxis: {
      title: {
        text: "Sessions",
        style: {
          fontSize: "1rem",
          fontWeight: 600,
        }
      },
    },
    series: [{
      name: "Sessions",
      data: data.by_browser.map(item => {
        return {
          x: item.key == null ? "Unknown" : item.key,
          y: item.sessions,
        }
      })
    }]
  };

  let station_options: ApexOptions = {
    chart: {
      type: "bar",
      fontFamily: "inherit",
      height: chartHeight,
      animations: {
        enabled: false,
      },
    },
    dataLabels: {
      enabled: false,
    },
    plotOptions: {
      bar: {
        distributed: true,
        columnWidth: "40%",
      }
    }, 
    yaxis: {
      title: {
        text: "Sessions",
        style: {
          fontSize: "1rem",
          fontWeight: 600,
        }
      },
    },
    series: [{
      name: "Sessions",
      data: data.by_station.map(item => {
        const station = data.stations.find(station => station._id === item.key);
        return {
          x: station?.name || item.key,
          y: item.sessions,
        }
      })
    }]
  };

  const map_data = (() => {
    
    const country_sessions: Partial<Record<CountryCode, number>> = {};
    const country_avg_listening_ms: Partial<Record<CountryCode, number>> = {};
    for(const item of data.by_country) {
      if(item.key) {
        country_sessions[item.key] = item.sessions;
        country_avg_listening_ms[item.key] = item.total_duration_ms / item.sessions;
      }
    }

    return {
      sessions: data.sessions,
      ips: data.ips,
      country_sessions,
      country_avg_listening_ms,
    }
  })()

  const compare_numbers = (a: number, b: number) => a - b;
  
  const format_mins = (ms: number) => {
    return (ms / MIN).toFixed(2)
  }

  const format_hours = (ms: number) => {
    return (ms / HOUR).toFixed(2)
  }

  const get_common_grid_options = () => {
    
    type Item = {  sessions: number, total_duration_ms: number }

    const fields = {
      "sessions": {
        name: "Sessions",
        format: item => String(item.sessions),
        sort: (a, b) => compare_numbers(a.sessions, b.sessions),
        numeric: true,
      },

      "avg_time": {
        name: "Average listening minutes",
        format: item => item.sessions === 0 ? "-" : format_mins(item.total_duration_ms / item.sessions),
        sort: (a, b) => compare_numbers((a.total_duration_ms / a.sessions) || 0,  (b.total_duration_ms / b.sessions) || 0),
        numeric: true,
      },

      "total_time": {
        name: "Total listening hours",
        format: item => format_hours(item.total_duration_ms),
        sort: (a, b) => compare_numbers(a.sessions, b.sessions),
        numeric: true,
      },

    } satisfies Record<string, DataGridField<Item>>;
    
    const sorted_by = {
      key: "sessions" as "sessions",
      direction: "desc" as "desc",
    }
    return { fields, sorted_by }
  }

  const get_by_browser_grid = () => {
    const items = data.by_browser;
    const common = get_common_grid_options();    
    const fields = {
      "key": {
        name: "Browser",
        format: item => item.key || "Unknown",
        sort: (a, b) => (a.key || "").localeCompare(b.key || ""),
      },
      ...common.fields
    } satisfies Record<string, DataGridField<typeof items[number]>>;


    return {
      ...common,
      title: "Stats by browser",
      fields,
      items,
    } satisfies DataGridData<typeof items[number], typeof fields>;
  }

  const get_by_device_grid = () => {
    const items = data.by_os;
    const common = get_common_grid_options();    
    const fields = {
      "key": {
        name: "Device",
        format: item => item.key || "Unknown",
        sort: (a, b) => (a.key || "").localeCompare(b.key || ""),
      },
      ...common.fields
    } satisfies Record<string, DataGridField<typeof items[number]>>;


    return {
      ...common,
      title: "Stats by device",
      fields,
      items,
    } satisfies DataGridData<typeof items[number], typeof fields>;
  }

  const get_by_station_grid = () => {
    const items = data.by_station;
    const common = get_common_grid_options();
    const display_name = (id: string) => {
      const station = data.stations.find(station => station._id === id);
      return station?.name || `#${id}`  
    }
    const fields = {
      "key": {
        name: "Station",
        format: item => display_name(item.key),
        sort: (a, b) => display_name(a.key).localeCompare(display_name(b.key))
      },
      ...common.fields
    } satisfies Record<string, DataGridField<typeof items[number]>>;


    return {
      ...common,
      title: "Stats by station",
      fields,
      items,
    } satisfies DataGridData<typeof items[number], typeof fields>;
  }

  const get_by_country_grid = () => {
    const items = data.by_country;
    const common = get_common_grid_options();
    const display_name = (iso: string | null) => iso == null ? "Unknown" : country_names[iso] || `#${iso}`;

    const fields = {
      "key": {
        name: "Country",
        format: item => display_name(item.key),
        sort: (a, b) => display_name(a.key || "").localeCompare(display_name(b.key || "")),
      },
      ...common.fields
    } satisfies Record<string, DataGridField<typeof items[number]>>;


    return {
      ...common,
      title: "Stats by country",
      fields,
      items,
    } satisfies DataGridData<typeof items[number], typeof fields>;
  }


  const get_by_day_items = (by_day: typeof data.by_day) => {
    const key = (item: { year: number, month: number, day: number }) => `${item.year}-${item.month}-${item.day}`;
    const cache = new Map<string, typeof data.by_day[number]>();
    for(const item of by_day) {
      cache.set(key(item.key), item);
    }

    const items: typeof by_day = [];

    const start = startOfDay(new Date(data.since));
    const end = add(startOfDay(new Date(data.until)), { days: 1 });

    let current = start;
        
    do {

      const k = key({ year: current.getFullYear(), month: current.getMonth() + 1, day: current.getDate() });
      const item = cache.get(k);

      const item_key = { year: current.getFullYear(), month: current.getMonth() + 1, day: current.getDate() }

      if(item == null) {
        items.push({
          key: item_key,
          sessions: 0,
          total_duration_ms: 0,
        })
      } else {
        items.push(item)
      }

      current = add(current, { days: 1 })

    } while(!isSameDay(current, end));

    return items;
  }

  const get_by_day_grid = () => {
    const items = get_by_day_items(data.by_day);
    const common = get_common_grid_options();

    const to_num = (key: typeof items[number]["key"]): number => {
      return (key.year * 10000) + (key.month * 100) + (key.day);
    }

    const pad = (n: number, size: number = 2, str = "0") => {
      return String(n).padStart(size, str);
    }

    const fields = {
      "key": {
        name: "Date",
        format: item => `${pad(item.key.year, 4)}/${pad(item.key.month + 1)}/${pad(item.key.day)}`,
        sort: (a, b) => compare_numbers(to_num(a.key), to_num(b.key))
      },
      ...common.fields
    } satisfies Record<string, DataGridField<typeof items[number]>>;


    return {
      ...common,
      title: "Stats by date",
      fields,
      items,
      sorted_by: {
        key: "key",
        direction: "asc",
      }
    } satisfies DataGridData<typeof items[number], typeof fields>;
  }

  const by_browser_grid_data = get_by_browser_grid();
  const by_device_grid_data = get_by_device_grid();
  const by_station_grid_data = get_by_station_grid();
  const by_country_grid_data = get_by_country_grid();
  const by_day_grid_data = get_by_day_grid();
  

</script>

<style>
  .analytics {
    display: flex;
    flex-direction: column;
    align-items: stretch;
  }

  .totals {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(15rem, 1fr));
    gap: 1.5rem;
  }

  .total {
    background: #fff;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    border-radius: 0.5rem;
    box-shadow: var(--some-shadow);
    flex: 1;
    padding: 2rem 1rem;
  }

  .total-title {
    font-weight: 600;
    font-size: 1.25rem;
  }

  .total-value {
    font-size: 1.5rem;
    font-weight: 700;
    margin-block-start: 1rem;
  }

  .charts {
    display: flex;
    flex-direction: column;
    align-items: stretch;
    gap: 3rem;
    margin-top: 3rem;
  }

  .chart-box {
    background: #fff;
    border-radius: 0.5rem;
    overflow: hidden;
    padding: 1rem;
    box-shadow: var(--some-shadow);
  }

  .chart {
    height: calc(var(--chart-height) + 1rem);
  }
  .chart :global(.apexcharts-menu-item.exportCSV) {
    display: none;
  }

  .map {
    --map-max-width: 800px;
    margin-inline: -1rem;
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }

  .chart-title {
    font-size: 1.25rem;
    font-weight: 600;
  }

  .chart-grid-daily {
    margin-top: 1rem;
  }
</style>

<div class="analytics">
  <div class="totals">
    <div class="total">
      <div class="total-title">Total sessions completed</div>
      <div class="total-value">
        {data.sessions}
      </div>
    </div>

    <div class="total">
      <div class="total-title">Total unique IPs</div>
      <div class="total-value">
        {data.ips}
      </div>
    </div>

    <div class="total">
      <div class="total-title">Average listening time</div>
      <div class="total-value">
        {time(data.total_duration_ms / data.sessions)}
      </div>
    </div>

    <div class="total">
      <div class="total-title">Total listening time</div>
      <div class="total-value">
        {time(data.total_duration_ms)}
      </div>
    </div>
  </div>

  <div class="charts" style:--chart-height="{chartHeight}px">
    <div class="chart-box">
      <div class="chart-title">By date</div>
      <div class="chart" use:chart={days_options} />
    </div>

    <div class="chart-box chart-box-map">
      <div class="chart-title">By country</div>
      <div class="map">
        <Mapp stats={map_data} {country_names} locale={$locale.stats_map} />
      </div>
      <div class="chart-grid">
        <DataGrid data={by_country_grid_data} />
      </div>
    </div>

    <div class="chart-box">
      <div class="chart-title">By device</div>
      <div class="chart" use:chart={os_options} />
      <div class="chart-grid">
        <DataGrid data={by_device_grid_data} />
      </div>
    </div>

    <div class="chart-box">
      <div class="chart-title">By browser</div>
      <div class="chart" use:chart={browser_options} />
      <div class="chart-grid">
        <DataGrid data={by_browser_grid_data} />
      </div>
    </div>

    <div class="chart-box">
      <div class="chart-title">By station</div>
      <div class="chart" use:chart={station_options} />
      <div class="chart-grid">
        <DataGrid data={by_station_grid_data} />
      </div>
    </div>

    <div class="chart-box">
      <div class="chart-title">Daily stats</div>
      <div class="chart-grid chart-grid-daily">
        <DataGrid data={by_day_grid_data} />
      </div>
    </div>
  </div>
</div>
