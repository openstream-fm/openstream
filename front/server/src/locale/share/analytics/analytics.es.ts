const locale: import("./analytics.locale").AnalyticsLocale = {
  "Sessions": "Sesiones",
  "Average_listening_minutes": "Minutos promedio de escucha",
  "Average_listening_time": "Tiempo promedio de escucha",
  "Total_listening_time": "Tiempo total de escucha",
  "Total_listening_hours": "Horas totales de escucha",
  "Unique_IPs": "IPs únicas",

  // @example Browser: Chrome
  "Browser": "Navegador",

  // @example Device: iPad
  // @example Device: Android
  "Device": "Dispositivo",

  // @example Device: Unknown
  // @example Browser: Unknown
  "Unknown": "Desconocido",

  // @context Radio Station
  "Station": "Estación",

  // @example Coutry: Argentina
  "Country": "País",

  // @example Date: 2022/01/01
  "Date": "Fecha",


  "Stats_by_browser": "Estadísticas por navegador",
  "Stats_by_device": "Estadísticas por dispositivo",
  "Stats_by_station": "Estadísticas por estación",
  "Stats_by_country": "Estadísticas por país",
  "Stats_by_date": "Estadísticas por fecha",

  "By_date": "Por fecha",
  "By_country": "Por país",
  "By_device": "Por dispositivo",
  "By_browser": "Por navegador",
  "By_station": "Por estación",
  "Daily_stats": "Estadísticas diarias",

  "no_data_message": "No hay datos registrados para los filtros especificados",

  "time": {
    "1_day": "1 día",
    "n_days": "@n días",
    "1_hour": "1 hora",
    "n_hours": "@n horas",
    "1_minute": "1 minuto",
    "n_minutes": "@n minutos",
    "1_second": "1 segundo",
    "n_seconds": "@n segundos",

    /// @example 2 días y 1 hora
    /// @example 1 hora y 2 minutos
    /// @example 10 minutos y 3 segundos
    "join": "@time1 y @time2",
  },

  "filters": {
    "query_kind": {
      "today": "Hoy",
      "yesterday": "Ayer",
      "last-24h": "Últimas 24 horas",
      "this-week": "Esta semana",
      "previous-week": "Semana anterior",
      "last-7d": "Últimos 7 días",
      "this-month": "Este mes",
      "previous-month": "Mes anterior",
      "last-30d": "Últimos 30 días",
      "custom": "Personalizado",
    },

    "submit": "Obtener analíticas",

    "All_stations": "Todas las estaciones",
    "No_stations": "Sin estaciones",

    "no_stations_message": "Esta cuenta no tiene estaciones",
  },

  "data_grid": {
    "export_as_csv": "Exportar como CSV"
  }
}

export default locale;