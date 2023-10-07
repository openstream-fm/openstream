<svelte:options immutable={true} />

<script lang="ts" context="module">
    export type ClickEvent = 
    | { kind: "country_code", value: CountryCode | null | undefined }
    | { kind: "os", value: string | null | undefined }
    | { kind: "browser", value: string | null | undefined }
    | { kind: "domain", value: string | null | undefined }
    | { kind: "station", value: string };
</script>

<script lang="ts">

  export let data: import("$server/defs/analytics/Analytics").Analytics;
  export let country_names: Record<string, string | undefined>;
  export let locale: import("$server/locale/share/analytics/analytics.locale").AnalyticsLocale;
  export let stats_map_locale: import("$server/locale/share/stats-map/stats-map.locale").StatsMapLocale;
  export let lang: string;

  export let os: string | null | undefined;
  export let country_code: CountryCode | null | undefined; 
  export let browser: string | null | undefined;
  export let domain: string | null | undefined;
  export let selected_stations: StationItem[] | "all";

  let LocaleSessions = data.is_now ? locale.Listeners : locale.Sessions;
  $: LocaleSessions = data.is_now ? locale.Listeners : locale.Sessions;

  const with_max_concurrent = data.max_concurrent_listeners != null;

  let selected_by_hour_chart = (() => {
    if(!data.by_hour) return false;
    if((new Date(data.until).getTime() - new Date(data.since).getTime()) < (1000 * 60 * 60 * 24 + 1000)) {
      return true
    } else {
      return false;
    } 
  })()

  let selected_by_hour_grid = selected_by_hour_chart;

  const df = new Intl.DateTimeFormat(lang, {
    day: "2-digit",
    month: "2-digit",
    year: "2-digit",
    weekday: "long",
    hour: "2-digit",
    minute: "2-digit",
  });

  const total_date = (s: Date | string) => {
    return df.format(new Date(s));
  } 

  const is_station_selected = (id: string) => {
    return Array.isArray(selected_stations) && selected_stations.length === 1 && selected_stations[0]._id === id;
  }
  
  export let on_click: (event: ClickEvent) => void;

  import Mapp from "$share/Map/Map.svelte";
  import { chart } from "$share/apexcharts/apexcharts";
  import type { ApexOptions } from "apexcharts";
  import { add, formatISO, isSameDay, isSameHour, startOfDay, startOfHour } from "date-fns";
  import DataGrid from "./DataGrid.svelte";
  
  import type { DataGridData, DataGridField } from "./types";
  import type { CountryCode } from "$server/defs/CountryCode";
  import type { StationItem } from "./AnalyticsFilters.svelte";
  import { ripple } from "$share/ripple";
  
  const SEC = 1000;
  const MIN = SEC * 60;
  const HOUR = MIN * 60;
  const DAY = HOUR * 24;

  const { round, floor, min } = Math;

  const chartHeight = 350;

  const to_fixed = (n: number, min_digits: number, max_digits = min_digits) => {
    return new Intl.NumberFormat(lang, {
      maximumFractionDigits: min_digits,
      minimumFractionDigits: max_digits,
    }).format(n || 0)
  }

  const n_time = (n: number, unit: "day" | "hour" | "minute" | "second") => {
    if(n === 1) {
      return locale.time[`1_${unit}`];
    } else {
      return locale.time[`n_${unit}s`].replace("@n", String(n));
    }
  }

  const join_time = (n1: number, unit1: "day" | "hour" | "minute" | "second", n2: number, unit2: "day" | "hour" | "minute" | "second") => {
    return locale.time.join
      .replace("@time1", n_time(n1, unit1))
      .replace("@time2", n_time(n2, unit2))
  }


  const time = (ms: number): string => {
    const days = ms / DAY;
    const hours = (ms % DAY) / HOUR;
    const mins = (ms % HOUR) / MIN;
    const secs = (ms % MIN) / SEC;

    if (days >= 1) {
      return join_time(floor(days), "day", min(23, round(hours)), "hour");
    } else if (hours >= 1) {
      return join_time(floor(hours), "hour", min(59, round(mins)), "minute");
    } else if (mins >= 1) {
      return join_time(floor(mins), "minute", min(59, round(secs)), "second");
    } else {
      return n_time(min(59, round(secs)), "second");
    }
  };

  const by_day_data = (by_day: typeof data.by_day) => {
    const key = (item: { year: number, month: number, day: number }) => `${item.year}-${item.month}-${item.day}`;
    const cache = new Map<string, typeof data.by_day[number]>();
    for(const item of by_day) {
      cache.set(key(item.key), item);
    }

    const ips: (number | null)[] = [];
    const total_hours: (number | null)[] = [];
    const dates: Date[] = [];

    const start = startOfDay(new Date(data.since));
    const end = add(startOfDay(new Date(data.until)), { days: 1 });

    let current = start;
        
    do {

      const k = key({ year: current.getFullYear(), month: current.getMonth() + 1, day: current.getDate() });
      const item = cache.get(k);

      dates.push(current);

      if(item == null) {
        ips.push(null);
        total_hours.push(null);
      } else {
        ips.push(item.ips);
        total_hours.push(item.total_duration_ms / 1000 / 60 / 60);
      }

      current = add(current, { days: 1 })

    } while(!isSameDay(current, end));

    return { dates, ips, total_hours }
  }

  const days_data = by_day_data(data.by_day);
  const days_options: ApexOptions = {
    series: [
      {
        name: locale.Unique_IPs,
        data: days_data.ips
      },
      {
        name: locale.Total_listening_hours,
        data: days_data.total_hours
      },
    ],

    markers: {
      showNullDataPoints: true,
      size: 4,
      hover: {
        size: 8,
      }
    },

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
        formatter: (v) => {
          return new Date(v).toLocaleDateString(lang, {
            year: "numeric",
            day: "2-digit",
            month: "2-digit",
          })
        }
      }
    },
    yaxis: [
      {
        title: {
          text: locale.Unique_IPs,
          style: {
           fontSize: "1rem",
           fontWeight: 600,
          }
        },
        labels: {
          formatter: v => (v % 1 === 0 || v == null) ? String(v ?? 0) : to_fixed(v, 2),
        } 
      }, {
        opposite: true,
        title: {
          text: locale.Total_listening_hours,
          style: {
           fontSize: "1rem",
           fontWeight: 600,
          }
        },
        labels: {
          formatter: v => {
            if(v == null) return "-";
            return to_fixed(v, 1);
            // if(v == null) return "-";
            // let total_secs = Math.round(v / SEC);
            // let mins = Math.floor(total_secs / 60);
            // let secs = total_secs % 60;
            // const pad = (v: number) => String(v).padStart(2, "0");
            // return `${pad(mins)}:${pad(secs)}`;
          }
        }
      }
    ],
    tooltip: {
      x: {
        formatter: v => {
          return new Date(v).toLocaleString(lang, {
            year: "numeric",
            month: "2-digit",
            day: "2-digit",
          })
        }
      },
    }
  };

  const by_hour_data = (by_hour: Exclude<typeof data.by_hour, null>) => {
    const key = (item: { year: number, month: number, day: number, hour: number }) => `${item.year}-${item.month}-${item.day}-${item.hour}`;
    const cache = new Map<string, Exclude<typeof data.by_hour, null>[number]>();
    for(const item of by_hour) {
      cache.set(key(item.key), item);
    }

    const ips: (number | null)[] = [];
    const total_hours: (number | null)[] = [];
    const dates: Date[] = [];

    const start = startOfHour(new Date(data.since));
    const end = add(startOfHour(new Date(data.until)), { hours: 1 });

    let current = start;
        
    do {

      const k = key({ year: current.getFullYear(), month: current.getMonth() + 1, day: current.getDate(), hour: current.getHours() });
      const item = cache.get(k);

      dates.push(current);

      if(item == null) {
        ips.push(null);
        total_hours.push(null);
      } else {
        ips.push(item.ips);
        total_hours.push(item.total_duration_ms / 1000 / 60 / 60);
      }

      current = add(current, { hours: 1 })

    } while(!isSameHour(current, end));

    return { dates, ips, total_hours }
  }

  const hours_data = data.by_hour && by_hour_data(data.by_hour);
  const hours_options: ApexOptions | null = hours_data && {
    series: [
      {
        name: locale.Unique_IPs,
        data: hours_data.ips
      },
      {
        name: locale.Total_listening_hours,
        data: hours_data.total_hours
      },
    ],

    markers: {
      showNullDataPoints: true,
      size: 2,
      hover: {
        size: 4,
      }
    },

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
      width: 1,
      curve: "smooth"
    },
    xaxis: {
      type: "datetime",
      categories: (() => {
        const temp = hours_data.dates.map(date => formatISO(date))
        console.log("hours_categories", temp);
        return temp;
      })(),
      
      labels: {
        formatter: (v) => {
          return new Date(v).toLocaleDateString(lang, {
            year: "numeric",
            day: "2-digit",
            month: "2-digit",
            hour: "2-digit"
          })
        }
      }
    },
    yaxis: [
      {
        title: {
          text: locale.Unique_IPs,
          style: {
           fontSize: "1rem",
           fontWeight: 600,
          }
        },
        labels: {
          formatter: v => (v % 1 === 0 || v == null) ? String(v ?? 0) : to_fixed(v, 2),
        } 
      }, {
        opposite: true,
        title: {
          text: locale.Total_listening_hours,
          style: {
           fontSize: "1rem",
           fontWeight: 600,
          }
        },
        labels: {
          formatter: v => {
            if(v == null) return "-";
            return to_fixed(v, 1);
            // if(v == null) return "-";
            // let total_secs = Math.round(v / SEC);
            // let mins = Math.floor(total_secs / 60);
            // let secs = total_secs % 60;
            // const pad = (v: number) => String(v).padStart(2, "0");
            // return `${pad(mins)}:${pad(secs)}`;
          }
        }
      }
    ],
    tooltip: {
      x: {
        formatter: v => {
          return new Date(v).toLocaleString(lang, {
            year: "numeric",
            month: "2-digit",
            day: "2-digit",
            hour: "2-digit",
          })
        }
      },
    }
  };

  // const colors = [
  //   // BLUE
  //   "#0074D9",
  //   // GREEN
  //   "#2ECC40",
  //   // RED
  //   "#FF4136",
  //   // PURPLE
  //   "#B10DC9",
  //   // ORANGE
  //   "#FF851B",
  //   // YELLOW
  //   "#FFDC00",
  //   // AQUA
  //   "#7FDBFF",
  //   // TEAL
  //   "#39CCCC",
  //   // NAVY
  //   "#001f3f",
  //   // FUCHSIA
  //   "#F012BE",
  //   // MAROON
  //   "#85144b",
  // ]

  // const os_options: ApexOptions = {
  //   chart: {
  //     type: "pie",
  //     fontFamily: "inherit",
  //     height: chartHeight,
  //   },
  //   plotOptions: {
  //     pie: {
  //       donut: {
  //         labels: {
  //           show: true,
  //         }
  //       }
  //     },
  //   },
  //   colors,
  //   series: data.by_os.map(item => item.sessions),
  //   labels: data.by_os.map(item => item.key || locale.Unknown),
  // }

  const os_options: ApexOptions = {
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
        text: LocaleSessions,
        style: {
          fontSize: "1rem",
          fontWeight: 600,
        }
      },
    },
    series: [{ 
      name: LocaleSessions,
      data: data.by_os.map(item => {
        return {
          x: item.key == null ? locale.Unknown : item.key,
          y: item.sessions,
        }
      })
    }]
  };


  const browser_options: ApexOptions = {
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
        text: LocaleSessions,
        style: {
          fontSize: "1rem",
          fontWeight: 600,
        }
      },
    },
    series: [{
      name: LocaleSessions,
      data: data.by_browser.map(item => {
        return {
          x: item.key == null ? locale.Unknown : item.key,
          y: item.sessions,
        }
      })
    }]
  };


  const domain_options: ApexOptions = {
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
        text: LocaleSessions,
        style: {
          fontSize: "1rem",
          fontWeight: 600,
        }
      },
    },
    series: [{ 
      name: LocaleSessions,
      data: data.by_domain.map(item => {
        return {
          x: item.key == null ? locale.Unknown : item.key,
          y: item.sessions,
        }
      })
    }]
  };

  const station_options: ApexOptions = {
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
        text: LocaleSessions,
        style: {
          fontSize: "1rem",
          fontWeight: 600,
        }
      },
    },
    series: [{
      name: LocaleSessions,
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
    for(const item of data.by_country) {
      if(item.key) {
        country_sessions[item.key] = item.sessions;
      }
    }

    return {
      sessions: data.sessions,
      ips: data.ips,
      country_sessions,
    }
  })()

  const map_country_has_data = (code: CountryCode) => {
    const item = data.by_country.find(item => item.key === code);
    return item && item?.sessions !== 0;
  }

  const map_country_sessions = (code: CountryCode) => {
    const item = data.by_country.find(item => item.key === code);
    return item?.sessions || 0;
  }

  const map_country_avg_minutes = (code: CountryCode) => {
    const item = data.by_country.find(item => item.key === code);
    if(item) {
      return to_fixed(item.total_duration_ms / item.sessions / MIN, 1);
    } else {
      return "-"
    }
  }

  const map_country_total_hours = (code: CountryCode) => {
    const item = data.by_country.find(item => item.key === code);
    if(item) {
      return to_fixed(item.total_duration_ms / HOUR, 1)
    } else {
      return "0"
    }
  }

  const compare_numbers = (a: number, b: number) => a - b;
  
  const format_mins = (ms: number) => {
    // const mins = floor(ms / MIN);
    // const secs = min(59, floor((ms % MIN) / SEC));
    // return `${String(mins).padStart(2, "0")}:${String(secs).padStart(2, "0")}`
    return to_fixed(ms / MIN, 1)
  }

  const format_hours = (ms: number) => {
    //const hours = floor(ms / HOUR);
    //const mins = min(59, round((ms % HOUR) / MIN));
    //const secs = min(59, round((ms % MIN) / SEC));
    //return `${String(hours).padStart(2, "0")}:${String(mins).padStart(2, "0")}:${String(secs).padStart(2, "0")}`
    return to_fixed(ms / HOUR, 1)
  }

  const get_common_grid_options = () => {
    
    type Item = {
      sessions: number,
      ips: number,
      total_duration_ms: number,
      total_transfer_bytes: number,
      max_concurrent_listeners?: number
    }

    const max_concurrent_field = (!data.is_now && with_max_concurrent ? {
      "max_concurrent": {
        name: locale.Max_concurrent_listeners,
        format: item => to_fixed(item.max_concurrent_listeners!, 0),
        sort: (a, b) => compare_numbers(a.max_concurrent_listeners!, b.max_concurrent_listeners!),
        numeric: true,
      }
    } : {} as Record<string, never>) satisfies Record<string, DataGridField<Item>>;

    const not_now_fields = (!data.is_now ? {
      "avg_time": {
        name: locale.Average_listening_minutes,
        format: item => item.sessions === 0 ? "-" : format_mins(item.total_duration_ms / item.sessions),
        sort: (a, b) => compare_numbers((a.total_duration_ms / a.sessions) || 0,  (b.total_duration_ms / b.sessions) || 0),
        numeric: true,
      },

      "total_time": {
        name: locale.Total_listening_hours,
        format: item => format_hours(item.total_duration_ms),
        sort: (a, b) => compare_numbers(a.total_duration_ms, b.total_duration_ms),
        numeric: true,
      },

      "total_transfer": {
        name: locale.Total_transfer_in_MB,
        format: item => to_fixed(item.total_transfer_bytes / 1_000_000, 1),
        sort: (a, b) => compare_numbers(a.total_transfer_bytes, b.total_transfer_bytes),
        numeric: true,
      }
    } : {} as Record<string, never>) satisfies Record<string, DataGridField<Item>>;

    const fields = {
      "sessions": {
        name: LocaleSessions,
        format: item => String(item.sessions || 0),
        sort: (a, b) => compare_numbers(a.sessions, b.sessions),
        numeric: true,
      },

      "ips": {
        name: locale.Unique_IPs,
        format: item => String(item.ips || 0),
        sort: (a, b) => compare_numbers(a.ips, b.ips),
        numeric: true,
      },

      ...max_concurrent_field,
      
      ...not_now_fields,

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
        name: locale.Browser,
        format: item => item.key || locale.Unknown,
        sort: (a, b) => (a.key || "").localeCompare(b.key || ""),
        is_selected: item => browser === item.key,
        on_click: item => on_click({ kind: "browser", value: item.key })
      },
      ...common.fields
    } satisfies Record<string, DataGridField<typeof items[number]>>;


    return {
      ...common,
      title: locale.Stats_by_browser,
      fields,
      items,
    } satisfies DataGridData<typeof items[number], typeof fields>;
  }

  const get_by_device_grid = () => {
    const items = data.by_os;
    const common = get_common_grid_options();    
    const fields = {
      "key": {
        name: locale.Device,
        format: item => item.key || locale.Unknown,
        sort: (a, b) => (a.key || "").localeCompare(b.key || ""),
        is_selected: item => os === item.key,
        on_click: item => on_click({ kind: "os", value: item.key })
      },
      ...common.fields
    } satisfies Record<string, DataGridField<typeof items[number]>>;


    return {
      ...common,
      title: locale.Stats_by_device,
      fields,
      items,
    } satisfies DataGridData<typeof items[number], typeof fields>;
  }

  const get_by_domain_grid = () => {
    const items = data.by_domain;
    const common = get_common_grid_options();    
    const fields = {
      "key": {
        name: locale.Website,
        format: item => item.key || locale.Unknown,
        sort: (a, b) => (a.key || "").localeCompare(b.key || ""),
        is_selected: item => domain === item.key,
        on_click: item => on_click({ kind: "domain", value: item.key })
      },
      ...common.fields
    } satisfies Record<string, DataGridField<typeof items[number]>>;


    return {
      ...common,
      title: locale.Stats_by_device,
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
        name: locale.Station,
        format: item => display_name(item.key),
        sort: (a, b) => display_name(a.key).localeCompare(display_name(b.key)),
        is_selected: item => is_station_selected(item.key),
        on_click: item => on_click({ kind: "station", value: item.key })
      },
      ...common.fields
    } satisfies Record<string, DataGridField<typeof items[number]>>;


    return {
      ...common,
      title: locale.Stats_by_station,
      fields,
      items,
    } satisfies DataGridData<typeof items[number], typeof fields>;
  }

  const get_by_country_grid = () => {
    const items = data.by_country;
    const common = get_common_grid_options();
    const display_name = (iso: string | null) => iso == null ? locale.Unknown : country_names[iso] || `#${iso}`;

    const fields = {
      "iso": {
        name: "ISO",
        /// ZZ is the "unspecified" value for ISO country codes
        /// The definition of CountryCode does not include ZZ, instead it use the null value, Eg: Option<CountryCode>
        format: item => item.key ?? "ZZ",
        csv_only: true,
      },
      "key": {
        name: locale.Country,
        format: item => display_name(item.key),
        sort: (a, b) => display_name(a.key || "").localeCompare(display_name(b.key || "")),
        is_selected: item => country_code === item.key,
        on_click: item => on_click({ kind: "country_code", value: item.key })
      },
      ...common.fields
    } satisfies Record<string, DataGridField<typeof items[number]>>;


    return {
      ...common,
      title: locale.Stats_by_country,
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
          ips: 0,
          total_duration_ms: 0,
          total_transfer_bytes: 0,
          // max_concurrent_listeners: undefined,
          // max_concurrent_listeners_date: undefined,
        })
      } else {
        items.push(item)
      }

      current = add(current, { days: 1 })

    } while(!isSameDay(current, end));

    return items;
  }

  const get_by_hour_items = (by_hour: Exclude<typeof data.by_hour, null>) => {
    const key = (item: { year: number, month: number, day: number, hour: number }) => `${item.year}-${item.month}-${item.day}-${item.hour}`;
    const cache = new Map<string, Exclude<typeof data.by_hour, null>[number]>();
    for(const item of by_hour) {
      cache.set(key(item.key), item);
    }

    const items: typeof by_hour = [];

    const start = startOfHour(new Date(data.since));
    const end = add(startOfHour(new Date(data.until)), { hours: 1 });

    let current = start;
        
    do {

      const k = key({ year: current.getFullYear(), month: current.getMonth() + 1, day: current.getDate(), hour: current.getHours() });
      const item = cache.get(k);

      const item_key = { year: current.getFullYear(), month: current.getMonth() + 1, day: current.getDate(), hour: current.getHours() }

      if(item == null) {
        items.push({
          key: item_key,
          sessions: 0,
          ips: 0,
          total_duration_ms: 0,
          total_transfer_bytes: 0,
          // max_concurrent_listeners: undefined,
          // max_concurrent_listeners_date: undefined,
        })
      } else {
        items.push(item)
      }

      current = add(current, { hours: 1 })

    } while(!isSameHour(current, end));

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
        name: locale.Date,
        format: item => `${pad(item.key.year, 4)}/${pad(item.key.month)}/${pad(item.key.day)}`,
        sort: (a, b) => compare_numbers(to_num(a.key), to_num(b.key))
      },
      ...common.fields
    } satisfies Record<string, DataGridField<typeof items[number]>>;


    return {
      ...common,
      title: locale.Stats_by_date,
      fields,
      items,
      sorted_by: {
        key: "key",
        direction: "asc",
      }
    } satisfies DataGridData<typeof items[number], typeof fields>;
  }

  const get_by_hour_grid = () => {
    if(!data.by_hour) return null;
    const items = get_by_hour_items(data.by_hour);
    const common = get_common_grid_options();

    const to_num = (key: typeof items[number]["key"]): number => {
      return (key.year * 1000000) + (key.month * 10000) + (key.day * 100) + key.hour;
    }

    const pad = (n: number, size: number = 2, str = "0") => {
      return String(n).padStart(size, str);
    }

    const fields = {
      "key": {
        name: locale.Hour,
        // non breaking space and non breaking hyphen
        format: item => `${pad(item.key.year, 4)}/${pad(item.key.month)}/${pad(item.key.day)}\u0020\u2011\u0020${pad(item.key.hour)}`,
        sort: (a, b) => compare_numbers(to_num(a.key), to_num(b.key))
      },
      ...common.fields
    } satisfies Record<string, DataGridField<typeof items[number]>>;


    return {
      ...common,
      title: locale.Stats_by_hour,
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
  const by_domain_grid_data = get_by_domain_grid();
  const by_station_grid_data = get_by_station_grid();
  const by_country_grid_data = get_by_country_grid();
  const by_day_grid_data = get_by_day_grid();
  const by_hour_grid_data = get_by_hour_grid();

  const units = ["byte", "kilobyte", "megabyte", "gigabyte", "terabyte"]
  const bytes = (n: number) => {
    const unit_i = Math.min(units.length - 1, Math.floor(Math.max(0, Math.log(n)) / Math.log(1000)));
    const unit_n = n / (1000 ** unit_i);

    const f = new Intl.NumberFormat(lang, {
      style: "unit",
      unit: units[unit_i],
      minimumFractionDigits: 1,
      maximumFractionDigits: 1,
    })
    
    return f.format(unit_n || 0);
  }
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
    font-size: 1rem;
  }

  .total-value {
    font-size: 1.6rem;
    font-weight: 700;
    margin-block-start: 0.75rem;
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

  .chart-grid-daily, .chart-grid-hourly {
    margin-top: 1rem;
  }

  .empty {
    text-align: center;
    justify-content: center;
    align-items: center;
  }

  .map-tooltip-name {
    font-weight: 600;
    font-size: 1rem;  
    margin-block-end: 0.35rem;
  }

  .map-tooltip-stat {
    font-size: 0.85rem; 
  }

  .map-tooltip-stat + .map-tooltip-stat {
    margin-top: 0.2rem;
  }

  .map-tooltip-stat-value {
    font-weight: 700;
  }

  .total-max-concurrent-date {
    text-align: center;
    font-weight: 400;
    font-size: 0.9rem;
    margin-top: 0.75rem;
  }

  .chart-title.with-btn {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
  }

  .day-hour-btn {
    padding: 0.5rem;
    margin: -0.5rem;
    border-radius: 0.25rem;
    transition: background-color 250ms ease;
    cursor: pointer;
  }

  .day-hour-btn:hover {
    background: rgba(0,0,0,0.05);
  }
</style>

<div class="analytics">
  {#if data.sessions === 0}
    <div class="empty">
      {locale.no_data_message}
    </div>
  {:else}
    <div class="totals">
      <div class="total">
        <div class="total-title">{LocaleSessions}</div>
        <div class="total-value">
          {data.sessions}
        </div>
      </div>

      <div class="total">
        <div class="total-title">{locale.Unique_IPs}</div>
        <div class="total-value">
          {data.ips}
        </div>
      </div>

      {#if !data.is_now}
        {#if with_max_concurrent}
          <div class="total">
            <div class="total-title">{locale.Max_concurrent_listeners}</div>
            <div class="total-value">
              {data.max_concurrent_listeners}
              {#if data.max_concurrent_listeners_date}
                <div class="total-max-concurrent-date">
                  {total_date(data.max_concurrent_listeners_date)}
                </div>
              {/if}
            </div>
          </div>
        {/if}
      {/if}

      {#if !data.is_now}
        <div class="total">
          <div class="total-title">{locale.Average_listening_time}</div>
          <div class="total-value">
            {time(data.total_duration_ms / data.sessions)}
          </div>
        </div>
      {/if}

      {#if !data.is_now}
        <div class="total">
          <div class="total-title">{locale.Total_listening_time}</div>
          <div class="total-value">
            {time(data.total_duration_ms)}
          </div>
        </div>
      {/if}

      {#if !data.is_now}
        <div class="total">
          <div class="total-title">{locale.Total_transfer}</div>
          <div class="total-value">
            {bytes(data.total_transfer_bytes)}
          </div>
        </div>
      {/if}
    </div>
    
    <div class="charts" style:--chart-height="{chartHeight}px">

      {#if !data.is_now}
        {#if hours_options}
          <div class="chart-box">
            <div class="chart-title with-btn">
              <button class="day-hour-btn ripple-container" use:ripple on:click={() => selected_by_hour_chart = !selected_by_hour_chart}>
                {#if selected_by_hour_chart}
                  {locale.By_hour}
                {:else}
                  {locale.By_date}
                {/if}
                {#if hours_options}
                  ►
                {/if}
              </button>
            </div>
            {#if hours_options && selected_by_hour_chart}
              <div class="chart" use:chart={hours_options} />
            {:else}
              <div class="chart" use:chart={days_options} />
            {/if}
          </div>
        {:else}
          <div class="chart-box">
            <div class="chart-title with-btn">{locale.By_date}</div>
            <div class="chart" use:chart={days_options} />
          </div>
        {/if}
      {/if}

      <div class="chart-box chart-box-map">
        <div class="chart-title">{locale.By_country}</div>
        <div class="map">
          <Mapp stats={map_data} {country_names} locale={stats_map_locale}>
            <div slot="tooltip" class="map-tooltip" let:country_name let:country_code>
              <div class="map-tooltip-name">{country_name}</div>
              <div class="map-tooltip-stat">
                <span class="map-tooltip-stat-name">
                  {LocaleSessions}:
                </span>
                <span class="map-tooltip-stat-value">
                  {map_country_sessions(country_code)}
                </span>
              </div>
              
              {#if !data.is_now}
                {#if map_country_has_data(country_code)}
                  <div class="map-tooltip-stat">
                    <span class="map-tooltip-stat-name">
                      {locale.Average_listening_minutes}:
                    </span>
                    <span class="map-tooltip-stat-value">
                      {map_country_avg_minutes(country_code)}
                    </span>
                  </div>
                  <div class="map-tooltip-stat">
                    <span class="map-tooltip-stat-name">
                      {locale.Total_listening_hours}:
                    </span>
                    <span class="map-tooltip-stat-value">
                      {map_country_total_hours(country_code)}
                    </span>
                  </div>
                {/if}
              {/if}
            </div>
          </Mapp>
        </div>
        <div class="chart-grid">
          <DataGrid data={by_country_grid_data} locale={locale.data_grid} />
        </div>
      </div>

      <div class="chart-box">
        <div class="chart-title">{locale.By_website}</div>
        <div class="chart" use:chart={domain_options} />
        <div class="chart-grid">
          <DataGrid data={by_domain_grid_data} locale={locale.data_grid} />
        </div>
      </div>

      <div class="chart-box">
        <div class="chart-title">{locale.By_device}</div>
        <div class="chart" use:chart={os_options} />
        <div class="chart-grid">
          <DataGrid data={by_device_grid_data} locale={locale.data_grid} />
        </div>
      </div>

      <div class="chart-box">
        <div class="chart-title">{locale.By_browser}</div>
        <div class="chart" use:chart={browser_options} />
        <div class="chart-grid">
          <DataGrid data={by_browser_grid_data} locale={locale.data_grid} />
        </div>
      </div>

      <div class="chart-box">
        <div class="chart-title">{locale.By_station}</div>
        <div class="chart" use:chart={station_options} />
        <div class="chart-grid">
          <DataGrid data={by_station_grid_data} locale={locale.data_grid} />
        </div>
      </div>

      {#if !data.is_now}
        {#if by_hour_grid_data}
          <div class="chart-box">
            <div class="chart-title with-btn">
              <button class="day-hour-btn ripple-container" use:ripple on:click={() => selected_by_hour_grid = !selected_by_hour_grid}>
                {#if selected_by_hour_grid}
                  {locale.Hourly_stats}
                {:else}
                  {locale.Daily_stats}
                {/if}
                {#if by_hour_grid_data}
                  ►
                {/if}
              </button>
            </div>
            {#if selected_by_hour_grid}
              <div class="chart-grid chart-grid-hourly">
                <DataGrid data={by_hour_grid_data} locale={locale.data_grid} />
              </div>
            {:else}
              <div class="chart-grid chart-grid-daily">
                <DataGrid data={by_day_grid_data} locale={locale.data_grid} />
              </div>
            {/if}
          </div>
        {:else}
          <div class="chart-box">
            <div class="chart-title">{locale.Daily_stats}</div>
            <div class="chart-grid chart-grid-daily">
              <DataGrid data={by_day_grid_data} locale={locale.data_grid} />
            </div>
          </div>
        {/if}
      {/if}
    </div>
  {/if}

</div>
