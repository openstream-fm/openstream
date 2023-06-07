/// file: analytics.zh.ts
const locale: import("./analytics.locale").AnalyticsLocale = {
  "Sessions": "会话",
  "Average_listening_minutes": "平均收听分钟数",
  "Average_listening_time": "平均收听时间",
  "Total_listening_time": "总收听时间",
  "Total_listening_hours": "总收听小时数",
  "Total_transfer_in_MB": "总传输量（MB）",
  "Unique_IPs": "唯一IP",

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

  // @example Coutry: Argentina
  "Country": "国家",

  // @example Date: 2022/01/01
  "Date": "日期",

  "Stats_by_browser": "按浏览器统计",
  "Stats_by_device": "按设备统计",
  "Stats_by_station": "按电台统计",
  "Stats_by_country": "按国家统计",
  "Stats_by_date": "按日期统计",

  "By_date": "按日期",
  "By_country": "按国家",
  "By_device": "按设备",
  "By_browser": "按浏览器",
  "By_station": "按电台",
  "Daily_stats": "每日统计",

  "no_data_message": "没有符合指定筛选条件的数据",

  "time": {
    "1_day": "1天",
    "n_days": "@n天",
    "1_hour": "1小时",
    "n_hours": "@n小时",
    "1_minute": "1分钟",
    "n_minutes": "@n分钟",
    "1_second": "1秒",
    "n_seconds": "@n秒",

    /// @example 2天和1小时
    /// @example 1小时和2分钟
    /// @example 10分钟和3秒
    "join": "@time1和@time2",
  },

  "filters": {
    "query_kind": {
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

    "submit": "获取分析数据",

    "All_stations": "所有电台",
    "No_stations": "无电台",

    "no_stations_message": "此帐户没有电台",
  },

  "data_grid": {
    "export_as_csv": "导出为CSV"
  }
}

export default locale;