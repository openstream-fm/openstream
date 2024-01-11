const locale: import("./analytics.locale").AnalyticsLocale = {
  "Sessions": "Sitzungen",
  "Listeners": "Zuhörer",
  "Unique_IPs": "Eindeutige IPs",
  "Average_listening_minutes": "Durchschnittliche Hördauer in Minuten",
  "Average_listening_time": "Durchschnittliche Hördauer",
  "Total_listening_time": "Gesamte Hördauer",
  "Total_listening_hours": "Gesamte Hördauer in Stunden",
  "Total_transfer": "Gesamtübertragung",
  "Total_transfer_in_MB": "Gesamtübertragung in MB",
  "Max_concurrent_listeners": "Max. gleichzeitige Zuhörer",

  "App_ID": "App-ID",
  "App_version": "App-Version",

  // @example Browser: Chrome
  "Browser": "Browser",

  // @example Device: iPad
  // @example Device: Android
  "Device": "Gerät",

  // @example Device: Unknown
  // @example Browser: Unknown
  "Unknown": "Unbekannt",

  // @context Radio Station
  "Station": "Sender",

  "Website": "Webseite",

  // @example Coutry: Argentina
  "Country": "Land",

  // @example Date: 2022/01/01
  "Date": "Datum",

  // @example Hour: 2022/01/01 12:00
  "Hour": "Uhr",

  "Stats_by_browser": "Statistiken nach Browser",
  "Stats_by_device": "Statistiken nach Gerät",
  "Stats_by_station": "Statistiken nach Sender",
  "Stats_by_country": "Statistiken nach Land",
  "Stats_by_website": "Statistiken nach Webseite",
  "Stats_by_date": "Statistiken nach Datum",
  "Stats_by_hour": "Statistiken nach Stunde",
  "Stats_by_app_ID": "Statistiken nach App-ID",
  "Stats_by_app_version": "Statistiken nach App-Version",

  "By_date": "Nach Datum",
  "By_hour": "Nach Stunde",
  "By_country": "Nach Land",
  "By_device": "Nach Gerät",
  "By_browser": "Nach Browser",
  "By_station": "Nach Sender",
  "By_website": "Nach Webseite",
  "By_app_ID": "Nach App-ID",
  "By_app_version": "Nach App-Version",

  "Daily_stats": "Tägliche Statistiken",
  "Hourly_stats": "Stündliche Statistiken",

  "no_data_message": "Keine Daten für die angegebenen Filter vorhanden",

  "time": {
    "1_day": "1 Tag",
    "n_days": "@n Tage",
    "1_hour": "1 Stunde",
    "n_hours": "@n Stunden",
    "1_minute": "1 Minute",
    "n_minutes": "@n Minuten",
    "1_second": "1 Sekunde",
    "n_seconds": "@n Sekunden",

    /// @example 2 días y 1 hora
    /// @example 1 hora y 2 minutos
    /// @example 10 minutos y 3 segundos
    "join": "@time1 und @time2",
  },

  "filters": {
    "query_kind": {
      "now": "Jetzt",
      "today": "Heute",
      "yesterday": "Gestern",
      "last-24h": "Letzte 24 Stunden",
      "this-week": "Diese Woche",
      "previous-week": "Vorherige Woche",
      "last-7d": "Letzte 7 Tage",
      "this-month": "Dieser Monat",
      "previous-month": "Vorheriger Monat",
      "last-30d": "Letzte 30 Tage",
      "custom": "Benutzerdefiniert",
    },

    "submit": "Analysen abrufen",

    "All_stations": "Alle Sender",
    "No_stations": "Keine Sender",

    "no_stations_message": "Dieses Konto hat keine Sender",
  },

  "data_grid": {
    "export_as_csv": "Als CSV exportieren"
  },

  "Search...": "Suchen...",
  "No_stations_for_this_query": "Keine Sender für diese Anfrage gefunden",
  "This_field_is_required": "Dieses Feld ist erforderlich",

  "apps": "Apps",
  "stream": "Stream",
}

export default locale;