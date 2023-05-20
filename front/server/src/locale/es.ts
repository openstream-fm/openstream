// es.ts
const locale = {

  // @notranslate
  "logo_text": "openstream",

  // @notranslate
  "app_name": "Openstream Studio",

  "drawer": {
    "account_selector": {
      "see_all_accounts": "Ver todas las cuentas",
    },
    "dashboard": "Tablero",
    "stations": "Estaciones",
    "members": "Miembros",
    "analytics": "Analítica",
  },

  "stats_map": {
    "now": "Ahora",
    "24_hours": "24 horas",
    "7_days": "7 días",
    "30_days": "30 días",
    "listener": "oyente",
    "listeners": "oyentes",
    "country": "país",
    "countries": "países",  
  },

  "limits": {
    "of": "de",
    "stations": "Estaciones",
    "listeners": "Oyentes",
    "transfer": "Transferencia",
    "storage": "Almacenamiento",
  },

  "dialogs": {
    "delete": {
      "default_message": "Esta acción es permanente.",
      "cancel": "Cancelar",
      "delete": "Eliminar"
    }
  },

  "station_nav": {
    "dashboard": "Tablero",
    "profile": "Perfil",
    "playlist": "Lista de reproducción",
    "broadcast": "Transmisión",
    "settings": "Configuración",
  },

  "station_profile": {
    "titles": {
      "logo": "Logo",
      "profile_info": "Información del perfil",
      "contact_info": "Información de contacto",
      "social": "Enlaces sociales",
      "apps": "Enlaces de aplicaciones",
    },
    "upload_image": "Subir imagen",
    "picture_requirement_labels": {
      "format": "Formatos de imagen aceptados:",
      "size": "Tamaño mínimo de imagen:",
      "file_size": "Tamaño máximo de archivo:",
      "square": "La imagen debe ser cuadrada",
    },

    "labels": {
      "name": "Nombre",
      "slogan": "Eslogan",
      "description": "Descripción",
      "country": "País",
      "type_of_content": "Tipo de contenido",
      "email": "Correo electrónico",
      "phone": "Número de teléfono completo",
      "whatsapp": "Número completo de WhatsApp",
      "website": "URL del sitio web",
      "twitter": "URL de Twitter",
      "facebook": "URL de Facebook",
      "instagram": "URL de Instagram",
      "youtube": "URL de Youtube",
      "twitch": "URL de Twitch",
      "google_play": "URL de Google Play",
      "app_store": "URL de App Store"
    }
  },

  "pages": {
    
    "error": {
      "retry": "Reintentar",
      "home": "Llévame a casa",
      "default_message": "Ocurrió un error",
      "offline": {
        "head": {
          "title": "Sin conexión",
        },
        "title": "Parece que estás sin conexión",
        "text": "Necesitas acceso a internet para usar @app_name",
      }
    },

    "accounts": {
      "head": {
        "title": "Cuentas",
      },
      "title": "Seleccionar una cuenta",
      "create_new_account": "crear una nueva cuenta",
      "or": "o",
      "no_items_message_html": "Aún no tienes una cuenta de emisor.<br/>Para comenzar a transmitir, regístrate para obtener una cuenta de emisor.",
      "no_items_create": "Crear mi cuenta de emisor",
    },

    "account.dashboard": {
      "edit": {
        "tooltip": "Editar",
        "dialog": {
          "field_label": "Nombre de la cuenta",
          "title": "Editar el nombre de tu cuenta",
          "save": "Guardar",
        }
      },

      "stats_map": {
        "all_stations": "Todas las estaciones",
      },

      "station_item": {
        "on_air": "AL AIRE",
        "off_air": "FUERA DEL AIRE",
        "playlist": "Lista de reproducción",
        "live": "En vivo",
      }
    },

    "stations": {
      "head": {
        "title": "Estaciones",
      },
      "title": "Seleccionar una estación",
      "create_new_station": "crear una nueva estación",
      "or": "o",
      "no_items_message_html": "Esta cuenta aún no tiene estaciones.<br />Para comenzar a transmitir, crea una nueva estación.",
      "no_items_create": "Crear una estación",
    },

    "stations.create_station": {
      "create_a_stations": "Crear una estación",
    },

    "station.dashboard": {
      "on_air": "AL AIRE",
      "off_air": "FUERA DEL AIRE",
      "playlist": "Lista de reproducción",
      "live": "En vivo",
      "preview": "Vista previa",
      "broadcast": "Transmisión",
      "aria_pause": "Pausar",
      "aria_play": "Reproducir",
    },

    "station.profile": {
      "head": {
        "title": "Perfil de la estación",
      },
      "title": "Perfil",
      "submit": "Guardar",
    },

    "station.playlist": {
      "head": {
        "title": "Lista de reproducción",
      },
      "title": "Lista de reproducción",
      "explain_html": "Crea una lista de reproducción de música o episodios antiguos para mantener tu estación activa las 24 horas, los 7 días de la semana.<br /> Cuando se pierde la conexión o no estás transmitiendo, <b>Lista de reproducción</b> tomará el control automáticamente.",
      "upload": "Subir",
      "browse": "Examinar",
      "upload_files": "Subir archivos",
      "tracks_title": "Pistas",
      "track": "pista",
      "tracks": "pistas",
      "actions": {
        "restart_playlist": "Reiniciar lista de reproducción",
        "shuffle_playlist": "Mezclar lista de reproducción",
        "unshuffle_playlist": "Desordenar lista de reproducción",
        "drag_to_rearrange": "Arrastrar para reorganizar",
        "edit": "Editar",
        "delete": "Eliminar",
      },
      "columns": {
        "title": "Título",
        "artist": "Artista",
        "album": "Álbum",
        "duration": "Duración",
      },
      "selection": {
        "one_track_selected": "1 pista seleccionada",
        "n_tracks_selected": "@n pistas seleccionadas",
        "delete_selected": "Eliminar seleccionados",
        "select_all": "Seleccionar todo",
        "unselect_all": "Deseleccionar todo",
      },
      "uploading": {
        "success": "Subido exitosamente",
        "waiting": "Esperando",
        "in_progress": "En progreso...",
        "retry": "Reintentar",
        "clear_done": "Borrar elementos terminados",
      },
      "dialogs": {
        "delete_track": {
          "title": "Eliminar pista @name" 
        },
        "delete_tracks": {
          "title": "Eliminar @n pistas",
        },
        "edit_track": {
          "title": "Editar pista @name",
          "fields": {
            "title": "Título",
            "artist": "Artista",
            "album": "Álbum",
          },
          "cancel": "Cancelar",
          "save": "Guardar",
        },
        "shuffle_playlist": {
          "title": "Mezclar lista de reproducción",
          "message": "¿Estás seguro de que deseas mezclar aleatoriamente toda la lista de reproducción?",
          "cancel": "Cancelar",
          "submit": "Mezclar",
        },
        "unshuffle_playlist": {
          "title": "Desordenar lista de reproducción",
          "message": "¿Estás seguro de que deseas desordenar toda la lista de reproducción?",
          "cancel": "Cancelar",
          "submit": "Desordenar",
        },
        "restart_playlist": {
          "title": "Reiniciar lista de reproducción",
          "message": "¿Estás seguro de que deseas reiniciar la lista de reproducción?",
          "cancel": "Cancelar",
          "submit": "Reiniciar",
        }
      }
    }
  },

  "user_menu": {
    "profile": "Perfil",
    "accounts": "Cuentas",
    "stations": "Estaciones",
    "sign_out": "Cerrar sesión",
  }
}

export default locale