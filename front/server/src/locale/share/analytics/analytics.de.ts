/// file: analytics.de.ts
const locale: import("./analytics.locale").AnalyticsLocale = {
  "Sessions": "Sitzungen",
  "Listeners": "Zuhörer",
  "Unique_IPs": "Einzigartige IPs",
  "Average_listening_minutes": "Durchschnittliche Zuhörminuten",
  "Average_listening_time": "Durchschnittliche Zuhörzeit",
  "Total_listening_time": "Gesamtzuhörzeit",
  "Total_listening_hours": "Gesamtzuhörstunden",
  "Total_transfer": "Gesamtübertragung",
  "Total_transfer_in_MB": "Gesamtübertragung in MB",
  "Max_concurrent_listeners": "Max. gleichzeitige Zuhörer",

  // @example Browser: Chrome
  "Browser": "Browser",

  // @example Device: iPad
  // @example Device: Android
  "Device": "Gerät",

  // @example Device: Unknown
  // @example Browser: Unknown
  "Unknown": "Unbekannt",

  // @context Radio Station
  "Station": "Station",

  "Website": "Webseite",

  // @example Coutry: Argentina
  "Country": "Land",

  // @example Date: 2022/01/01
  "Date": "Datum",

  // @example Hour: 2022/01/01 12:00
  "Hour": "Stunde",

  "Stats_by_browser": "Statistiken nach Browser",
  "Stats_by_device": "Statistiken nach Gerät",
  "Stats_by_station": "Statistiken nach Station",
  "Stats_by_country": "Statistiken nach Land",
  "Stats_by_website": "Statistiken nach Webseite",
  "Stats_by_date": "Statistiken nach Datum",
  "Stats_by_hour": "Statistiken nach Stunde",

  "By_date": "Nach Datum",
  "By_hour": "Nach Stunde",
  "By_country": "Nach Land",
  "By_device": "Nach Gerät",
  "By_browser": "Nach Browser",
  "By_station": "Nach Station",
  "By_website": "Nach Webseite",

  "Daily_stats": "Tägliche Statistiken",
  "Hourly_stats": "Stündliche Statistiken",

  "no_data_message": "Es sind keine Daten für die angegebenen Filter vorhanden",

  "time": {
    "1_day": "1 Tag",
    "n_days": "@n Tage",
    "1_hour": "1 Stunde",
    "n_hours": "@n Stunden",
    "1_minute": "1 Minute",
    "n_minutes": "@n Minuten",
    "1_second": "1 Sekunde",
    "n_seconds": "@n Sekunden",

    /// @example 2 Tage und 1 Stunde
    /// @example 1 Stunde und 2 Minuten
    /// @example 10 Minuten und 3 Sekunden
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

    "All_stations": "Alle Stationen",
    "No_stations": "Keine Stationen",

    "no_stations_message": "Dieses Konto hat keine Stationen",
  },

  "data_grid": {
    "export_as_csv": "Als CSV exportieren"
  },

  "Search...": "Suchen...",
  "No_stations_for_this_query": "Für diese Abfrage wurden keine Stationen gefunden",
  "This_field_is_required": "Dieses Feld ist erforderlich",
}

export default locale;