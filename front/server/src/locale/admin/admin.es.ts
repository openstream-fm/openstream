import stats_map from "../share/stats-map/stats-map.es";
import validate from "../share/validate/validate.es";
import analytics from "../share/analytics/analytics.es";
import countries from "../share/countries/countries.es";
import langs from "../share/langs/langs.es";
import misc from "../misc/misc.es";

const locale: import("./admin.locale").AdminLocale = {

  "lang": "es",
  "region": null,

  // @notranslate
  "logo_text": "openstream",

  // @notranslate
  "app_name": "Openstream Admin",

  "show_password": "Mostrar contraseña",
  "hide_password": "Ocultar contraseña",

  "validate": validate,
  "stats_map": stats_map,
  "analytics": analytics,
  "countries": countries,
  "langs": langs,
  "misc": misc,

  "pages": {
    "me": {
      "title": "Perfil",
      "fields": {
        "email": "Tu email",
        "first_name": "Tu nombre",
        "last_name": "Tu apellido",
        "phone": "Tu teléfono",
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
    }
  }
}

export default locale;