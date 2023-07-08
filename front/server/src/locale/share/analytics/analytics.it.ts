/// file: analytics.it.ts
const locale: import("./analytics.locale").AnalyticsLocale = {
  "Sessions": "Sessioni",
  "Average_listening_minutes": "Minuti medi di ascolto",
  "Average_listening_time": "Tempo medio di ascolto",
  "Total_listening_time": "Tempo totale di ascolto",
  "Total_listening_hours": "Ore totali di ascolto",
  "Total_transfer": "Trasferimento totale",
  "Total_transfer_in_MB": "Trasferimento totale in MB",
  "Unique_IPs": "IP unici",
  "Max_concurrent_listeners": "Max. ascoltatori simultanei",
  
  // @example Browser: Chrome
  "Browser": "Browser",

  // @example Device: iPad
  // @example Device: Android
  "Device": "Dispositivo",

  // @example Device: Unknown
  // @example Browser: Unknown
  "Unknown": "Sconosciuto",

  // @context Radio Station
  "Station": "Stazione",

  "Website": "Sito web",

  // @example Country: Italy
  "Country": "Paese",

  // @example Date: 2022/01/01
  "Date": "Data",


  "Stats_by_browser": "Statistiche per browser",
  "Stats_by_device": "Statistiche per dispositivo",
  "Stats_by_station": "Statistiche per stazione",
  "Stats_by_country": "Statistiche per paese",
  "Stats_by_website": "Statistiche per sito web",
  "Stats_by_date": "Statistiche per data",

  "By_date": "Per data",
  "By_country": "Per paese",
  "By_device": "Per dispositivo",
  "By_browser": "Per browser",
  "By_station": "Per stazione",
  "By_website": "Per sito web",
  "Daily_stats": "Statistiche giornaliere",

  "no_data_message": "Non ci sono dati registrati per i filtri specificati",

  "time": {
    "1_day": "1 giorno",
    "n_days": "@n giorni",
    "1_hour": "1 ora",
    "n_hours": "@n ore",
    "1_minute": "1 minuto",
    "n_minutes": "@n minuti",
    "1_second": "1 secondo",
    "n_seconds": "@n secondi",

    /// @example 2 giorni e 1 ora
    /// @example 1 ora e 2 minuti
    /// @example 10 minuti e 3 secondi
    "join": "@time1 e @time2",
  },

  "filters": {
    "query_kind": {
      "today": "Oggi",
      "yesterday": "Ieri",
      "last-24h": "Ultime 24 ore",
      "this-week": "Questa settimana",
      "previous-week": "Settimana precedente",
      "last-7d": "Ultimi 7 giorni",
      "this-month": "Questo mese",
      "previous-month": "Mese precedente",
      "last-30d": "Ultimi 30 giorni",
      "custom": "Personalizzato",
    },

    "submit": "Ottieni analitiche",

    "All_stations": "Tutte le stazioni",
    "No_stations": "Nessuna stazione",

    "no_stations_message": "Questo account non ha stazioni",
  },

  "data_grid": {
    "export_as_csv": "Esporta come CSV"
  }
}

export default locale;