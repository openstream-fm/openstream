import stats_map from "../share/stats-map/stats-map.it";
import validate from "../share/validate/validate.it";
import analytics from "../share/analytics/analytics.it";
import countries from "../share/countries/countries.it";
import langs from "../share/langs/langs.it";
import misc from "../misc/misc.it";
import language from "../share/language/language.it";

const locale: import("./admin.locale").AdminLocale = {

  "lang": "it",
  "region": null,

  // @notranslate
  "logo_text": "openstream",

  // @notranslate
  "app_name": "Openstream Admin",

  "show_password": "Mostra password",
  "hide_password": "Nascondi password",

  "validate": validate,
  "stats_map": stats_map,
  "analytics": analytics,
  "countries": countries,
  "langs": langs,
  "misc": misc,
  "language": language,

  "pages": {
    "me": {
      "title": "Profilo",
      "fields": {
        "email": "La tua email",
        "first_name": "Il tuo nome",
        "last_name": "Il tuo cognome",
        "phone": "Il tuo telefono",
        "current_password": "Password attuale",
        "new_password": "Nuova password",
        "confirm_password": "Conferma password",
        "language": "Lingua preferita",
      },
      "submit": {
        "profile": "Salva",
        "password": "Salva",
      },
      "change_password": {
        "title": "Cambia la tua password",
      },
      "more": {
        "title": "Altro",
        "connected_devices": "Dispositivi connessi",
      },
      "notifier": {
        "no_changes": "Nessun cambiamento da salvare",
        "profile_updated": "Profilo aggiornato",
        "password_updated": "Password aggiornata",
      }
    },

    "me/devices": {
      "head": {
        "title": "Dispositivi",
      },
      "title": "Dispositivi connessi",
      "note": "Lo stesso dispositivo può apparire più di una volta in questa lista. I dispositivi saranno disconnessi dopo 7 giorni di inattività.",
      "dialogs": {
        "disconnect": {
          "title": "Disconnetti dispositivo",
          "message": "Questa azione è permanente.",
          "cancel": "Annulla",
          "submit": "Disconnetti",
        },
      },

      "notifier": {
        "device_disconnected": "Dispositivo disconnesso",
      },

      "device": {
        "browser": "Browser",
        "os": "Sistema",
        "ip": "IP",
        "last_used": "Ultimo utilizzo",
        "connected": "Connesso",
        "unkown": "Sconosciuto",
        "tooltips": {
          "disconnect": "Disconnetti",
        }
      }
    }
  }
}

export default locale;