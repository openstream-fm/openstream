const locale: import("./analytics.locale.js").AnalyticsLocale = {
  "Sessions": "会话",
  "Listeners": "听众",
  "Unique_IPs": "独立IP",
  "Average_listening_minutes": "平均收听分钟数",
  "Average_listening_time": "平均收听时间",
  "Total_listening_time": "总收听时间",
  "Total_listening_hours": "总收听小时数",
  "Total_transfer": "总传输",
  "Total_transfer_in_MB": "总传输量（MB）",
  "Max_concurrent_listeners": "最大并发听众数",

  "App_ID": "应用ID",
  "App_version": "应用版本",

  // @example Browser: Chrome
  "Browser": "浏览器",

  // @example Device: iPad
  // @example Device: Android
  "Device": "设备",

  // @example Device: Unknown
  // @example Browser: Unknown
  "Unknown": "未知",

  // @context Radio Station
  "Station": "电台",

  "Website": "网站",

  // @example Coutry: Argentina
  "Country": "国家",

  // @example Date: 2022/01/01
  "Date": "日期",

  // @example Hour: 2022/01/01 12:00
  "Hour": "小时",

  "Stats_by_browser": "按浏览器统计",
  "Stats_by_device": "按设备统计",
  "Stats_by_station": "按电台统计",
  "Stats_by_country": "按国家统计",
  "Stats_by_website": "按网站统计",
  "Stats_by_date": "按日期统计",
  "Stats_by_hour": "按小时统计",
  "Stats_by_app_ID": "按应用ID统计",
  "Stats_by_app_version": "按应用版本统计",

  "By_date": "按日期",
  "By_hour": "按小时",
  "By_country": "按国家",
  "By_device": "按设备",
  "By_browser": "按浏览器",
  "By_station": "按电台",
  "By_website": "按网站",
  "By_app_ID": "按应用ID",
  "By_app_version": "按应用版本",

  "Daily_stats": "每日统计",
  "Hourly_stats": "每小时统计",

  "no_data_message": "指定过滤器没有注册数据",

  "time": {
    "1_day": "1天",
    "n_days": "@n天",
    "1_hour": "1小时",
    "n_hours": "@n小时",
    "1_minute": "1分钟",
    "n_minutes": "@n分钟",
    "1_second": "1秒",
    "n_seconds": "@n秒",

    /// @example 2 días y 1 hora
    /// @example 1 hora y 2 minutos
    /// @example 10 minutos y 3 segundos
    "join": "@time1和@time2",
  },

  "filters": {
    "query_kind": {
      "now": "现在",
      "today": "今天",
      "yesterday": "昨天",
      "last-24h": "最近24小时",
      "this-week": "本周",
      "previous-week": "上周",
      "last-7d": "最近7天",
      "this-month": "本月",
      "previous-month": "上个月",
      "last-30d": "最近30天",
      "custom": "自定义",
    },

    "submit": "获取分析",

    "All_stations": "所有电台",
    "No_stations": "没有电台",

    "no_stations_message": "该账户没有电台",
  },

  "data_grid": {
    "export_as_csv": "导出为CSV"
  },

  "Search...": "搜索...",
  "No_stations_for_this_query": "此查询没有找到电台",
  "This_field_is_required": "此字段为必填项",

  "apps": "应用",
  "stream": "流",
}

export default locale;