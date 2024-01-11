const locale: import("./analytics.locale").AnalyticsLocale = {
  "Sessions": "Sessões",
  "Listeners": "Ouvintes",
  "Unique_IPs": "IPs únicos",
  "Average_listening_minutes": "Minutos médios de escuta",
  "Average_listening_time": "Tempo médio de escuta",
  "Total_listening_time": "Tempo total de escuta",
  "Total_listening_hours": "Horas totais de escuta",
  "Total_transfer": "Transferência total",
  "Total_transfer_in_MB": "Transferência total em MB",
  "Max_concurrent_listeners": "Máx. ouvintes simultâneos",

  "App_ID": "ID do aplicativo",
  "App_version": "Versão do aplicativo",

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

  "Website": "Site",

  // @example Coutry: Argentina
  "Country": "País",

  // @example Date: 2022/01/01
  "Date": "Data",

  // @example Hour: 2022/01/01 12:00
  "Hour": "Hora",

  "Stats_by_browser": "Estatísticas por navegador",
  "Stats_by_device": "Estatísticas por dispositivo",
  "Stats_by_station": "Estatísticas por estação",
  "Stats_by_country": "Estatísticas por país",
  "Stats_by_website": "Estatísticas por site",
  "Stats_by_date": "Estatísticas por data",
  "Stats_by_hour": "Estatísticas por hora",
  "Stats_by_app_ID": "Estatísticas por ID do aplicativo",
  "Stats_by_app_version": "Estatísticas por versão do aplicativo",

  "By_date": "Por data",
  "By_hour": "Por hora",
  "By_country": "Por país",
  "By_device": "Por dispositivo",
  "By_browser": "Por navegador",
  "By_station": "Por estação",
  "By_website": "Por site",
  "By_app_ID": "Por ID do aplicativo",
  "By_app_version": "Por versão do aplicativo",

  "Daily_stats": "Estatísticas diárias",
  "Hourly_stats": "Estatísticas horárias",

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
      "now": "Agora",
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

    "no_stations_message": "Esta conta não tem estações",
  },

  "data_grid": {
    "export_as_csv": "Exportar como CSV"
  },

  "Search...": "Buscar...",
  "No_stations_for_this_query": "Não foram encontradas estações para esta busca",
  "This_field_is_required": "Este campo é obrigatório",

  "apps": "aplicativos",
  "stream": "transmissão",
}

export default locale;