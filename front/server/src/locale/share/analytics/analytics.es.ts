const locale: import("./analytics.locale").AnalyticsLocale = {
  "Sessions": "Sesiones",
  "Listeners": "Oyentes",
  "Unique_IPs": "IPs únicas",
  "Average_listening_minutes": "Minutos promedio de escucha",
  "Average_listening_time": "Tiempo promedio de escucha",
  "Total_listening_time": "Tiempo total de escucha",
  "Total_listening_hours": "Horas totales de escucha",
  "Total_transfer": "Transferencia total",
  "Total_transfer_in_MB": "Transferencia total en MB",
  "Max_concurrent_listeners": "Máx. oyentes simultáneos",

  "App_ID": "ID de aplicación",
  "App_version": "Version de aplicación",

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

  "Website": "Sitio web",

  // @example Coutry: Argentina
  "Country": "País",

  // @example Date: 2022/01/01
  "Date": "Fecha",

  // @example Hour: 2022/01/01 12:00
  "Hour": "Hora",

  "Stats_by_browser": "Estadísticas por navegador",
  "Stats_by_device": "Estadísticas por dispositivo",
  "Stats_by_station": "Estadísticas por estación",
  "Stats_by_country": "Estadísticas por país",
  "Stats_by_website": "Estadisticas por sitio web",
  "Stats_by_date": "Estadísticas por fecha",
  "Stats_by_hour": "Estadísticas por hora",
  "Stats_by_app_ID": "Estadísticas por ID de aplicación",
  "Stats_by_app_version": "Estadísticas por versión de aplicación",

  "By_date": "Por fecha",
  "By_hour": "Por hora",
  "By_country": "Por país",
  "By_device": "Por dispositivo",
  "By_browser": "Por navegador",
  "By_station": "Por estación",
  "By_website": "Por sitio web",
  "By_app_ID": "Por ID de aplicación",
  "By_app_version": "Por versión de aplicación",

  "Daily_stats": "Estadísticas diarias",
  "Hourly_stats": "Estadísticas por hora",

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
      "now": "Ahora",
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
  },

  "Search...": "Buscar...",
  "No_stations_for_this_query": "No se encontraron estaciones para esta búsqueda",
  "This_field_is_required": "Este campo es requerido",

  "apps": "apps",
  "stream": "stream",
}

export default locale;