const locale: import("./analytics.locale").AnalyticsLocale = {
  "Sessions": "جلسات",
  "Listeners": "المستمعين",
  "Unique_IPs": "عناوين IP فريدة",
  "Average_listening_minutes": "متوسط دقائق الاستماع",
  "Average_listening_time": "متوسط وقت الاستماع",
  "Total_listening_time": "إجمالي وقت الاستماع",
  "Total_listening_hours": "إجمالي ساعات الاستماع",
  "Total_transfer": "إجمالي النقل",
  "Total_transfer_in_MB": "إجمالي النقل بالميجابايت",
  "Max_concurrent_listeners": "الحد الأقصى للمستمعين المتزامنين",

  "App_ID": "معرف التطبيق",
  "App_version": "إصدار التطبيق",

  // @example Browser: Chrome
  "Browser": "المتصفح",

  // @example Device: iPad
  // @example Device: Android
  "Device": "الجهاز",

  // @example Device: Unknown
  // @example Browser: Unknown
  "Unknown": "غير معروف",

  // @context Radio Station
  "Station": "المحطة",

  "Website": "الموقع الإلكتروني",

  // @example Coutry: Argentina
  "Country": "البلد",

  // @example Date: 2022/01/01
  "Date": "التاريخ",

  // @example Hour: 2022/01/01 12:00
  "Hour": "الساعة",

  "Stats_by_browser": "الإحصائيات حسب المتصفح",
  "Stats_by_device": "الإحصائيات حسب الجهاز",
  "Stats_by_station": "الإحصائيات حسب المحطة",
  "Stats_by_country": "الإحصائيات حسب البلد",
  "Stats_by_website": "الإحصائيات حسب الموقع الإلكتروني",
  "Stats_by_date": "الإحصائيات حسب التاريخ",
  "Stats_by_hour": "الإحصائيات حسب الساعة",
  "Stats_by_app_ID": "الإحصائيات حسب معرف التطبيق",
  "Stats_by_app_version": "الإحصائيات حسب إصدار التطبيق",

  "By_date": "حسب التاريخ",
  "By_hour": "حسب الساعة",
  "By_country": "حسب البلد",
  "By_device": "حسب الجهاز",
  "By_browser": "حسب المتصفح",
  "By_station": "حسب المحطة",
  "By_website": "حسب الموقع الإلكتروني",
  "By_app_ID": "حسب معرف التطبيق",
  "By_app_version": "حسب إصدار التطبيق",

  "Daily_stats": "الإحصائيات اليومية",
  "Hourly_stats": "الإحصائيات الساعية",

  "no_data_message": "لا توجد بيانات مسجلة للفلاتر المحددة",

  "time": {
    "1_day": "يوم واحد",
    "n_days": "@n أيام",
    "1_hour": "ساعة واحدة",
    "n_hours": "@n ساعات",
    "1_minute": "دقيقة واحدة",
    "n_minutes": "@n دقائق",
    "1_second": "ثانية واحدة",
    "n_seconds": "@n ثواني",

    /// @example 2 días y 1 hora
    /// @example 1 hora y 2 minutos
    /// @example 10 minutos y 3 segundos
    "join": "@time1 و @time2",
  },

  "filters": {
    "query_kind": {
      "now": "الآن",
      "today": "اليوم",
      "yesterday": "الأمس",
      "last-24h": "آخر 24 ساعة",
      "this-week": "هذا الأسبوع",
      "previous-week": "الأسبوع الماضي",
      "last-7d": "آخر 7 أيام",
      "this-month": "هذا الشهر",
      "previous-month": "الشهر الماضي",
      "last-30d": "آخر 30 يومًا",
      "custom": "مخصص",
    },

    "submit": "الحصول على التحليلات",

    "All_stations": "جميع المحطات",
    "No_stations": "لا توجد محطات",

    "no_stations_message": "هذا الحساب لا يملك محطات",
  },

  "data_grid": {
    "export_as_csv": "تصدير كملف CSV"
  },

  "Search...": "بحث...",
  "No_stations_for_this_query": "لم يتم العثور على محطات لهذا البحث",
  "This_field_is_required": "هذا الحقل مطلوب",

  "apps": "تطبيقات",
  "stream": "بث",
}

export default locale;