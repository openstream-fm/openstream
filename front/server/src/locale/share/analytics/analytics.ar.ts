/// file: analytics.ar.ts
const locale: import("./analytics.locale").AnalyticsLocale = {
  "Sessions": "جلسات",
  "Average_listening_minutes": "متوسط دقائق الاستماع",
  "Average_listening_time": "متوسط وقت الاستماع",
  "Total_listening_time": "إجمالي وقت الاستماع",
  "Total_listening_hours": "إجمالي ساعات الاستماع",
  "Total_transfer_in_MB": "إجمالي التحويل بالميغابايت",
  "Unique_IPs": "عناوين IP فريدة",

  // @example Browser: Chrome
  "Browser": "المتصفح",

  // @example Device: iPad
  // @example Device: Android
  "Device": "الجهاز",

  // @example Device: Unknown
  // @example Browser: Unknown
  "Unknown": "غير معروف",

  // @context Radio Station
  "Station": "محطة",

  // @example Coutry: Argentina
  "Country": "الدولة",

  // @example Date: 2022/01/01
  "Date": "التاريخ",


  "Stats_by_browser": "الإحصائيات حسب المتصفح",
  "Stats_by_device": "الإحصائيات حسب الجهاز",
  "Stats_by_station": "الإحصائيات حسب المحطة",
  "Stats_by_country": "الإحصائيات حسب الدولة",
  "Stats_by_date": "الإحصائيات حسب التاريخ",

  "By_date": "حسب التاريخ",
  "By_country": "حسب الدولة",
  "By_device": "حسب الجهاز",
  "By_browser": "حسب المتصفح",
  "By_station": "حسب المحطة",
  "Daily_stats": "الإحصائيات اليومية",

  "no_data_message": "لا توجد بيانات مسجلة للمرشحين المحددين",

  "time": {
    "1_day": "1 يوم",
    "n_days": "@n أيام",
    "1_hour": "1 ساعة",
    "n_hours": "@n ساعات",
    "1_minute": "1 دقيقة",
    "n_minutes": "@n دقائق",
    "1_second": "1 ثانية",
    "n_seconds": "@n ثواني",

    /// @example 2 أيام و 1 ساعة
    /// @example 1 ساعة و 2 دقائق
    /// @example 10 دقائق و 3 ثواني
    "join": "@time1 و @time2",
  },

  "filters": {
    "query_kind": {
      "today": "اليوم",
      "yesterday": "أمس",
      "last-24h": "آخر 24 ساعة",
      "this-week": "هذا الأسبوع",
      "previous-week": "الأسبوع السابق",
      "last-7d": "آخر 7 أيام",
      "this-month": "هذا الشهر",
      "previous-month": "الشهر السابق",
      "last-30d": "آخر 30 يوم",
      "custom": "مخصص",
    },

    "submit": "الحصول على التحليلات",

    "All_stations": "جميع المحطات",
    "No_stations": "بدون محطات",

    "no_stations_message": "لا توجد محطات لهذا الحساب",
  },

  "data_grid": {
    "export_as_csv": "تصدير كملف CSV"
  }
}

export default locale;