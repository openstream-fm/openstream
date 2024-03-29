import stats_map from "../share/stats-map/stats-map.es.js";
import validate from "../share/validate/validate.es.js";
import countries from "../share/countries/countries.es.js";
import langs from "../share/langs/langs.es.js";
import type_of_content from "../share/type-of-content/type-of-content.es.js";
import analytics from "../share/analytics/analytics.es.js";
import payments from "../share/payments/payments.es.js";
import station_profile from "../share/station-profile/station-profile.es.js";
import misc from "../misc/misc.es.js";
import language from "../share/language/language.es.js";

const locale: import("./studio.locale.js").StudioLocale = {

  "lang": "es",
  "region": null,

  // @notranslate
  "logo_text": "openstream",

  // @notranslate
  "brand_name": "Openstream",

  // @notranslate
  "app_name": "Openstream Studio",
  
  type_of_content,
  countries,
  langs,
  validate,
  stats_map,
  analytics,
  payments,
  misc,
  station_profile,
  language,

  "copy_to_clipboard": "Copiar al portapapeles",
  "show_password": "Mostrar contraseña",
  "hide_password": "Ocultar contraseña",

  "prevent_unload_message":  "Si abandonas esta página los cambios que realizaste se perderán. ¿Quieres abandonar la página de todas maneras?",

  "drawer": {
    "account_selector": {
      "see_all_accounts": "Ver todas las cuentas",
    },
    "dashboard": "Panel",
    "stations": "Estaciones",
    "members": "Miembros",
    "analytics": "Analítica",
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
    "dashboard": "Panel",
    "profile": "Perfil",
    "playlist": "Playlist",
    "broadcast": "Transmitir",
    "settings": "Configuración",
  },

  "plan_selector": {
    "price": {
      "per_month": "por mes",
      "n_per_month": "@n / mes",
    },

    "unlimited": "Ilimitados",

    "trial": {
      "30_day": "30 días",
      "free_trial": "de prueba gratis",
      "tooltip": "No se te cobrará nada hasta que tu prueba termine, y puedes cancelar en cualquier momento"
    },

    "features": {
      "station": "Estación",
      "stations": "Estaciones",
      "listeners": "Oyentes",
      "transfer": "Transferencia",
      "storage": "Almacenamiento",
      "staff": "usuarios",
      "auto_dj": "Auto DJ",
      "stats": "Estadísticas avanzadas",
      "android_app": "Aplicación para Android",
    },

    "tooltips": {
      "one_station": "Solo puedes crear una estación en este plan",
      "n_stations": "Hasta @n estaciones diferentes",
      "listeners": "Hasta @n oyentes simultáneos",
      "transfer": "Con @tb TB de transferencia mensual tendrás capacidad de transmitir alrededor de @hours horas de audio",
      "storage": "@gb GB de almacenamiento para música o episodios antiguos",
      "staff": "Puedes agregar usuarios para todo tu equipo sin límite",
      "auto_dj": "Emite desde una playlist cuando estés sin conexión",
      "stats": "Estadísticas históricas y en vivo, sabé quien está escuchando tus estaciones",
      "android_app": "Una aplicación para Android con tu marca y tus estaciones, disponible en todo el mundo a través de Google Play",
    }
  },

  "pages": {

    "error": {
      "retry": "Reintentar",
      "home": "Ir al inicio",
      "default_message": "Ocurrió un error",
      "offline": {
        "head": {
          "title": "Sin conexión",
        },
        "title": "Parece que estás sin conexión",
        "text": "Se necesita acceso a internet para usar @app_name",
      }
    },

    "login": {
      "head": {
        "title": "Ingresar",
      },
      "title": "Ingresar",
      "fields": {
        "email": "Email",
        "password": "Contraseña",
      },
      "links": {
        "forgot": "Olvidaste tu contraseña?",
        "new_user": "Usuario nuevo?",
        "sign_up": "Regístrate",
      },
      "submit": "Ingresar"
    },

    "recover": {
      "head": {
        "title": "Recupera tu cuenta",
      },
      "title": "Recuperar",
      "comment": "Te enviaremos un email para que recuperes tu cuenta",
      "sent_message_html": "Te enviamos un email a <b>@email</b> con instrucciones para continuar",
      "links": {
        "login": "Voler a Ingresar",
      },
      "submit": "Enviar",
    },

    "plans": {
      "head": {
        "title": "Planes y Precios",
      },
      "title_1": "En vivo en 3... 2... 1...",
      "title_2": "Empieza tu emisora en menos de 60 segundos.",
      "title_3": "No se te cobrará hasta que tu prueba termine. Y puedes cancelar en cualquier momento.",
      "plan_selector": {
        "select_btn_label": "Empezar Prueba",
      }
    },

    "register": {
      "head": {
        "title": "Regístrarme",
      },
      "title": "Empieza tu prueba gratis",
      "plan": {
        "selected_plan": "Plan seleccionado",
        "n_per_month": "@n / mes",
        "limits": {
          "station": "Estación",
          "stations": "Estaciones",
          "listeners": "Oyentes",
          "transfer": "Transferencia",
          "storage": "Almacenamiento",
        },
        "links": {
          "plans": "Volver a planes y precios"
        }
      },
      "form": {
        "title": "Cuéntanos sobre ti",
        "account_name_comment": "Si estás creando una cuenta para una organización, puedes llenar este campo con el nombre de la organización",
        "fields": {
          "first_name": "Tu nombre",
          "last_name": "Tu apellido",
          "account_name": "Un nombre para tu cuenta",
          "phone": "Tu teléfono",
          "email": "Tu email",
          "password": "Tu contraseña",
          "confirm_password": "Confirma tu contraseña",
        },
        "next": "Siguiente",
      },
      "pay": {
        "title": "Detalles de pago",
        "message": "No se te cobrará hasta que tu prueba gratis de 30 días termine y puedes cancelar en cualquier momento."
      },

      "back": "Volver al paso anterior",

      "verification": {
        "title": "Ingresa el código de verificación",
        "message_html": "Te enviamos un código de verificación a <b>@email</b>",
        "submit": "Enviar",
      },
      "links": {
        "login_comment": "Ya tienes una cuenta?",
        "login_link": "Ingresar",
      }
    },

    "user_recovery": {
      "head_page_title": {
        "expired": "El link ha caducado",
        "used": "El link ya fue usado",
        "not_found": "No se encontró el link",
        "ok": "Reinicia tu contraseña",
      },
      "fields": {
        "email": "Email",
        "password": "Nueva contraseña",
        "confirm_password": "Confirma la contraseña",
      },
      "error": {
        "used_message_html": "El link que usaste para acceder a esta página ya ha sido utilizado.<br /> Crea un nuevo link desde la @user_recovery_page",
        "expired_message_html": "El link que usaste para acceder a esta página ha caducado.<br /> Crea un nuevo link desde la @user_recovery_page",
        "not_found_message_html": "El link que usaste para acceder a esta página no existe.<br /> Crea un nuevo link desde la @user_recovery_page",
        "user_recovery_page": "página de recuperación",
      },
      "submit": "Enviar",
      "notifier": {
        "password_updated": "Contraseña actualizada",
      }
    },

    "accounts": {
      "head": {
        "title": "Cuentas",
      },
      "title": "Elige una cuenta",
      "create_new_account": "crea una cuenta nueva",
      "or": "o",
      "no_items_message_html": "Todavía no tienes una cuenta de emisor.<br/>Para empezar a transmitir, crea tu cuenta de emisor.",
      "no_items_create": "Crear mi cuenta de emisor",
    },

    "accounts/create_account": {
      "head": {
        "title": "Elige un plan",
      },
      "title": "Elige un plan para tu nueva cuenta",
      "select": "Seleccionar",
    },

    "accounts/create_account/plan": {
      "head": {
        "title": "Crear una cuenta de emisor",
      },
      "title": "Crear una cuenta de emisor",
      "plan": {
        "title": "Plan seleccionado",
        "n_per_month": "@n / month",
        "station": "Estación",
        "stations": "Estaciones",
        "listeners": "Oyentes",
        "transfer": "Transferencia",
        "storage": "Almacenamiento",
        "back": "Volver a planes y precios",
      },
      "form": {
        "title": "Cuentanos de la nueva cuenta",
        "fields": {
          "account_name":"Un nombre para tu cuenta",
          "account_name_message": "Si estás creando una cuenta para una organización, puedes llenar este campo con el nombre de la organización",
        },
        "submit": "Crear",
        "next": "Siguiente",
        "pay": {
          "title": "Detalles de pago",
        },
        "back": "Volver al paso anterior",
      }
    },

    "account/dashboard": {
      "edit": {
        "tooltip": "Editar",
        "dialog": {
          "field_label": "Nombre de la cuenta",
          "title": "Edita el nombre de tu cuenta",
          "save": "Guardar",
        }
      },

      "stats_map": {
        "all_stations": "Todas las estaciones",
      },

      "station_item": {
        "on_air": "ON AIR",
        "off_air": "OFF AIR",
        "playlist": "Playlist",
        "live": "En vivo",
      },
    },

    "account/analytics": {
      "head": {
        "title": "Analítica",
      },
      "title": "Analítica",
    },

    "stations": {
      "head": {
        "title": "Estaciones",
      },
      "title": "Elige una estación",
      "create_new_station": "crea una nueva estación",
      "or": "o",
      "no_items_message_html": "Esta cuenta todavía no tiene estaciones.<br />Para empezar a transmitir, crea una nueva estación.",
      "no_items_create": "Crear una estación",
    },

    "stations/create_station": {
      "head": {
        "title": "Crear una estación"
      },
      "title": "Crear una estación",
      "submit": "Crear estación",
      "notifier": {
        "station_created": "Nueva estación creada",
      }
    },

    "station/dashboard": {
      "on_air": "ON AIR",
      "off_air": "OFF AIR",
      "playlist": "Playlist",
      "live": "En vivo",
      "preview": "Vista previa",
      "broadcast": "Transmitir",
      "aria_pause": "Pausar",
      "aria_play": "Reproducir",
    },

    "station/profile": {
      "head": {
        "title": "Perfil",
      },
      "title": "Perfil",
      "submit": "Guardar",
      "notifier": {
        "no_changes": "Sin cambios para guardar",
        "station_updated": "Estación actualizada",

      }
    },

    "station/playlist": {
      "head": {
        "title": "Playlist",
      },
      "title": "Playlist",
      "explain_html": "Crea una lista de música o viejos episodios para mantener tu estación activa 24x7<br /> Cuendo no tengas conexión o no estés transmitiendo en vivo, <b>Playlist</b> tomará el control automáticamente.",
      "upload": "Subir",
      "browse": "Explorar",
      "upload_files": "Subir archivos",
      "tracks_title": "Pistas",
      "track": "pista",
      "tracks": "pistas",
      "actions": {
        "restart_playlist": "Reiniciar playlist",
        "shuffle_playlist": "Mezclar playlist",
        "unshuffle_playlist": "Des-mezclar playlist",
        "drag_to_rearrange": "Arrastra para ordenar",
        "edit": "Editar",
        "delete": "Eliminar",
      },
      "columns": {
        "title": "Título",
        "artist": "Artísta",
        "album": "Álbum",
        "duration": "Duración",
      },
      "selection": {
        "one_track_selected": "1 pista seleccionada",
        "n_tracks_selected": "@n pistas seleccionadas",
        "delete_selected": "Eliminar seleccionados",
        "select_all": "Seleccionar todos",
        "unselect_all": "Des-seleccionar todos",
      },
      "uploading": {
        "success": "Subido con éxito",
        "waiting": "Esperando",
        "in_progress": "En proceso...",
        "retry": "Reintentar",
        "clear_done": "Ocultar items terminados",
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
          "title": "Mezclar playlist",
          "message": "¿Estás seguro que quieres mezclar aleatoriamente la lista de reproducción?",
          "cancel": "Cancelar",
          "submit": "Mezclar",
        },
        "unshuffle_playlist": {
          "title": "Unshuffle playlist",
          "message": "¿Estás seguro que quieres des-mezclar la lista de reproducción?",
          "cancel": "Cancelar",
          "submit": "Des-mezclar",
        },
        "restart_playlist": {
          "title": "Reiniciar playlist",
          "message": "¿Estás seguro que quieres reiniciar la lista de reproducción?",
          "cancel": "Cancelar",
          "submit": "Reiniciar",
        }
      },
      "upload_prevent_unload_message": "Salir de esta página cancelará las subidas pendientes. ¿Quieres salir de todas maneras?",
      "notifier": {
        "playlist_restarted": "Playlist reiniciada",
        "track_deleted": "Pista eliminada",
        "deleting_n_tracks": "Eliminando @n pistas",
        "n_tracks_deleted": "@n pistas eliminadas",
        "playlist_unshuffled": "Playlist des-mezclada",
        "playlist_shuffled": "Playlist mezclada",
      }
    },

    "station/broadcast": {
      "head": {
        "title": "Transmitir",
      },
      "title": "Transmitir",
      "icecast_settings": "Configuración de Icecast",
      "fields": {
        "address": "Dirección",
        "port": "Puerto",
        "mountpoint": "Punto de montaje",
        "username": "Usuario",
        "password": "Contraseña",
        "encoding": "Formato",
      },
      "encoding_or": "o",
      "password_reset": "Restablecer",
      "links": {
        "title": "URLs de transmisión",
        "main": "PRINCIPAL",
      },
      "notifier": {
        "copied_to_clipboard": "Copiado al portapapeles",
        "mount_password_reset": "Contraseña reseteada",
      },
      "dialogs": {
        "reset_password": {
          "title": "Resetar la contraseña del punto de montaje",
          "message": "¿Estás seguro que deseas restablecer la contraseña del punto de montaje?",
          "cancel": "Cacelar",
          "submit": "Restablecer contraseña",
        }
      }
    },

    "station/settings": {
      "head": {
        "title": "Configuración",
      },
      "title": "Configuración",
      "actions": {
        "title": "Acciones",
        "delete_station": "Eliminar estación",
      },
      "validate": {
        "station_name": "El nombre de la estación no coincide",
      },
      "notifier": {
        "station_deleted": "Estación eliminada",
      },
      "dialogs": {
        "delete_station": {
          "title": "Eliminar estación @name",
          "message_html": "La eliminación de una estación es una acción permanente, no podŕas acceder nuevamente a la información de la estación, por lo que asegurate de estar seguro de proceder.<br /><br />Si realmente quieres eliminar la estación@name ingresa el nombre de la estación en el siguiente campo: <b>@name</b><br />",
          "field_label": "Nombre de la estación",
          "cancel": "Cancelar",
          "submit": "Eliminar",
        }
      }
    },

    "me": {
      "title": "Perfil",
      "fields": {
        "email": "Tu email",
        "first_name": "Tu nombre",
        "last_name": "Tu apellido",
        "phone": "Tu teléfono",
        "current_password": "Contraseña actual",
        "new_password": "Nueva contraseña",
        "confirm_password": "Confirmar contraseña",
        "language": "Idioma preferido",
      },
      "submit": {
        "profile": "Guardar",
        "password": "Guardar",
      },
      "change_password": {
        "title": "Cambia tu contraseña",
      },
      "more": {
        "title": "Mas",
        "connected_devices": "Dispositivos conectados",
      },
      "notifier": {
        "no_changes": "Sin cambios para guardar",
        "profile_updated": "Perfil actualizado",
        "password_updated": "Contraseña actualizada",
      }
    },

    "me/devices": {
      "head": {
        "title": "Dispositivos",
      },
      "title": "Dispositivos conectados",
      "note": "El mismo dispositivo puede aparecer mas de una vez en esta lista. Los dispositivos serán desconectados después de 7 días sin actividad.",
      "dialogs": {
        "disconnect": {
          "title": "Desconectar dispositivo",
          "message": "Esta acción es permanente.",
          "cancel": "Cancelar",
          "submit": "Desconectar",
        },
      },

      "notifier": {
        "device_disconnected": "Dispositivo desconectado",
      },

      "device": {
        "browser": "Navegador",
        "os": "Sistema",
        "ip": "IP",
        "last_used": "Usado por última vez",
        "connected": "Conectado",
        "unkown": "Desconocido",
        "tooltips": {
          "disconnect": "Desconectar",
        }
      }
    },

    "account/members": {
      "head": {
        "title": "Miembros"
      },
      "title": "Miembros",

      "no_owner_message_p1": "Esta sección esta disponible solo para los administradores de la cuenta.",
      "no_owner_message_p2": "Contáctate con los administradores de la cuenta si necesitas invitar gente a participar en esta cuenta.",

      "Pending_invitations": "Invitaciones pendientes",
      "no_pending_invitations_message": "No hay invitaciones pendientes",
      "invite_btn_text": "Invitar personas",

      "validate": {
        "user_account_exists": "El usuario con email @email ya forma parte de esta cuenta",
      },

      "notifier": {
        "invitation_sent": "Invitación enviada",
        "member_access_revoked": "Acceso revocado",
        "member_role_changed": "Rol de acceso actualizado",
      },

      "actions": {
        "set_role_to": "Establecer rol a @role",
        "revoke_access": "Revocar acceso",
        "delete": "Eliminar",
      },

      "dialogs": {
        "invite": {
          "title": "Invita personas a participar de este cuenta con rol @role",
          "submit": "Invitar",
          "Email": "Email",
        }
      }
    },

    "email_invitation": {
      "head_page_title": {
        "not_found": "Invitaciónno encontrada",
        "expired": "La invitaciónha expirado",
        "accepted": "La invitation ya fue aceptada",
        "rejected": "La invitación ya fue rechazada",
        "ok": "Invitación pendiente",
      },

      "error_message": {
        "not_found": "El link que usaste para acceder a esta página ya no existe o fue eliminado",
        "expired": "La invitación ha expirado, contáctate con los administradores de la cuenta para que te envíen una nueva invitación",
        "accepted": "La invitación ya fue aceptada",
        "rejected": "La invitación ya fue rechazada, si fue un error, contáctate con los administradores de la cuenta para que te envíen una nueva invitación",
      },

      "description": {
        "with_sender_name_html": "<b>@sender</b> te está invitando a que te unas a <b>@account</b> en Openstream.",
        "without_sender_name_html": "Fuiste invitado a unirte a <b>@account</b> en Openstream",
      },

      "login_as_btn_html": "Ingresa como <b>@email</b> para aceptar la invitación",

      "form": {
        "fields": {
          "first_name": "Tu nombre",
          "last_name": "Tu apellido",
          "email": "Tu email",
          "password": "Constraseña",
          "confirm_password": "Confirmar contraseña",
        },
        "pre_message_html": "Para <b>aceptar</b> la invitación, completa el formulario.",
        "title": "Registarme",
        "submit": "Enviar",
      },

      "notifier": {
        "accept_error": "Hubo un error aceptando la invitacion: @error"
      }
    },

    "me/invitations": {
      "head": {
        "title": "Invitaciones pendientes",
      },
      "title": "Invitaciones pendientes",

      "no_items_message": "No tienes invitaciones pendientes",

      "notifier": {
        "accept_error": "Hubo un error aceptando la invtación: @error",
        "accepted": "Invitación aceptada",
        "rejected": "Invitación rechazada",
      },

      "actions": {
        "reject": "Rechazar",
        "accept": "Aceptar",
      },

      "item_message_with_sender_html": "<b>@sender</b> te esta invitando a que te unas a <b>@account</b>",
      "item_message_without_sender_html": "Fuiste invitado a unirte a <b>@account</b>",

      "dialogs": {
        "reject": {
          "title": "Rechazar invitación",
          "message": "¿Estás seguro de que quieres rechazar la invitación?",
          "cancel": "Cancelar",
          "reject": "Rechazar invitación",
        }
      }
    }
  },

  "user_menu": {
    "profile": "Perfil",
    "invitations": "Invitaciones",
    "accounts": "Cuentas",
    "stations": "Estaciones",
    "sign_out": "Cerrar sesión",
  }
}

export default locale;