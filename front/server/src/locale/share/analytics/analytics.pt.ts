/// file: analytics.pt.ts
const locale: import("./analytics.locale").AnalyticsLocale = {
  "Sessions": "Sessões",
  "Average_listening_minutes": "Minutos médios de escuta",
  "Average_listening_time": "Tempo médio de escuta",
  "Total_listening_time": "Tempo total de escuta",
  "Total_listening_hours": "Horas totais de escuta",
  "Unique_IPs": "IPs únicos",

  // @example Browser: Chrome
  "Browser": "Navegador",

  // @example Device: iPad
  // @example Device: Android
  "Device": "Dispositivo",

  // @example Device: Unknown
  // @example Browser: Unknown
  "Unknown": "Desconhecido",

  // @context Radio Station
  "Station": "Estação",

  // @example Coutry: Argentina
  "Country": "País",

  // @example Date: 2022/01/01
  "Date": "Data",


  "Stats_by_browser": "Estatísticas por navegador",
  "Stats_by_device": "Estatísticas por dispositivo",
  "Stats_by_station": "Estatísticas por estação",
  "Stats_by_country": "Estatísticas por país",
  "Stats_by_date": "Estatísticas por data",

  "By_date": "Por data",
  "By_country": "Por país",
  "By_device": "Por dispositivo",
  "By_browser": "Por navegador",
  "By_station": "Por estação",
  "Daily_stats": "Estatísticas diárias",

  "no_data_message": "Não há dados registrados para os filtros especificados",

  "time": {
    "1_day": "1 dia",
    "n_days": "@n dias",
    "1_hour": "1 hora",
    "n_hours": "@n horas",
    "1_minute": "1 minuto",
    "n_minutes": "@n minutos",
    "1_second": "1 segundo",
    "n_seconds": "@n segundos",

    /// @example 2 dias e 1 hora
    /// @example 1 hora e 2 minutos
    /// @example 10 minutos e 3 segundos
    "join": "@time1 e @time2",
  },

  "filters": {
    "query_kind": {
      "today": "Hoje",
      "yesterday": "Ontem",
      "last-24h": "Últimas 24 horas",
      "this-week": "Esta semana",
      "previous-week": "Semana anterior",
      "last-7d": "Últimos 7 dias",
      "this-month": "Este mês",
      "previous-month": "Mês anterior",
      "last-30d": "Últimos 30 dias",
      "custom": "Personalizado",
    },

    "submit": "Obter análises",

    "All_stations": "Todas as estações",
    "No_stations": "Sem estações",

    "no_stations_message": "Esta conta não possui estações",
  },

  "data_grid": {
    "export_as_csv": "Exportar como CSV"
  }
}

export default locale;