/// file: analytics.fr.ts
const locale: import("./analytics.locale").AnalyticsLocale = {
  "Sessions": "Sessions",
  "Average_listening_minutes": "Minutes moyennes d'écoute",
  "Average_listening_time": "Temps moyen d'écoute",
  "Total_listening_time": "Temps total d'écoute",
  "Total_listening_hours": "Heures totales d'écoute",
  "Total_transfer": "Transfert total",
  "Total_transfer_in_MB": "Transfert total en MB",
  "Unique_IPs": "IPs uniques",
  "Max_concurrent_listeners": "Max. auditeurs simultanés",

  // @example Browser: Chrome
  "Browser": "Navigateur",

  // @example Device: iPad
  // @example Device: Android
  "Device": "Appareil",

  // @example Device: Unknown
  // @example Browser: Unknown
  "Unknown": "Inconnu",

  // @context Radio Station
  "Station": "Station",

  "Website": "Site web",

  // @example Country: France
  "Country": "Pays",

  // @example Date: 2022/01/01
  "Date": "Date",


  "Stats_by_browser": "Statistiques par navigateur",
  "Stats_by_device": "Statistiques par appareil",
  "Stats_by_station": "Statistiques par station",
  "Stats_by_country": "Statistiques par pays",
  "Stats_by_website": "Statistiques par site web",
  "Stats_by_date": "Statistiques par date",

  "By_date": "Par date",
  "By_country": "Par pays",
  "By_device": "Par appareil",
  "By_browser": "Par navigateur",
  "By_station": "Par station",
  "By_website": "Par site web",
  "Daily_stats": "Statistiques quotidiennes",

  "no_data_message": "Aucune donnée enregistrée pour les filtres spécifiés",

  "time": {
    "1_day": "1 jour",
    "n_days": "@n jours",
    "1_hour": "1 heure",
    "n_hours": "@n heures",
    "1_minute": "1 minute",
    "n_minutes": "@n minutes",
    "1_second": "1 seconde",
    "n_seconds": "@n secondes",

    /// @example 2 jours et 1 heure
    /// @example 1 heure et 2 minutes
    /// @example 10 minutes et 3 secondes
    "join": "@time1 et @time2",
  },

  "filters": {
    "query_kind": {
      "today": "Aujourd'hui",
      "yesterday": "Hier",
      "last-24h": "Dernières 24 heures",
      "this-week": "Cette semaine",
      "previous-week": "Semaine précédente",
      "last-7d": "7 derniers jours",
      "this-month": "Ce mois",
      "previous-month": "Mois précédent",
      "last-30d": "30 derniers jours",
      "custom": "Personnalisé",
    },

    "submit": "Obtenir des analyses",

    "All_stations": "Toutes les stations",
    "No_stations": "Sans stations",

    "no_stations_message": "Ce compte n'a pas de stations",
  },

  "data_grid": {
    "export_as_csv": "Exporter en CSV"
  }
}

export default locale;