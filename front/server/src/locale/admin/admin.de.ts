import stats_map from "../share/stats-map/stats-map.de.js";
import validate from "../share/validate/validate.de.js";
import analytics from "../share/analytics/analytics.de.js";
import countries from "../share/countries/countries.de.js";
import langs from "../share/langs/langs.de.js";
import misc from "../misc/misc.de.js";
import language from "../share/language/language.de.js";

const locale: import("./admin.locale.js").AdminLocale = {

  "lang": "de",
  "region": null,

  // @notranslate
  "logo_text": "openstream",

  // @notranslate
  "app_name": "Openstream Admin",

  "show_password": "Passwort anzeigen",
  "hide_password": "Passwort verbergen",

  "validate": validate,
  "stats_map": stats_map,
  "analytics": analytics,
  "countries": countries,
  "langs": langs,
  "misc": misc,
  "language": language,

  "pages": {
    "me": {
      "title": "Profil",
      "fields": {
        "email": "Deine E-Mail",
        "first_name": "Dein Vorname",
        "last_name": "Dein Nachname",
        "phone": "Deine Telefonnummer",
        "current_password": "Aktuelles Passwort",
        "new_password": "Neues Passwort",
        "confirm_password": "Passwort bestätigen",
        "language": "Bevorzugte Sprache",
      },
      "submit": {
        "profile": "Speichern",
        "password": "Speichern",
      },
      "change_password": {
        "title": "Ändere dein Passwort",
      },
      "more": {
        "title": "Mehr",
        "connected_devices": "Verbundene Geräte",
      },
      "notifier": {
        "no_changes": "Keine Änderungen zum Speichern",
        "profile_updated": "Profil aktualisiert",
        "password_updated": "Passwort aktualisiert",
      }
    },

    "me/devices": {
      "head": {
        "title": "Geräte",
      },
      "title": "Verbundene Geräte",
      "note": "Dasselbe Gerät kann mehrmals in dieser Liste erscheinen. Geräte werden nach 7 Tagen Inaktivität getrennt.",
      "dialogs": {
        "disconnect": {
          "title": "Gerät trennen",
          "message": "Diese Aktion ist dauerhaft.",
          "cancel": "Abbrechen",
          "submit": "Trennen",
        },
      },

      "notifier": {
        "device_disconnected": "Gerät getrennt",
      },

      "device": {
        "browser": "Browser",
        "os": "System",
        "ip": "IP",
        "last_used": "Zuletzt verwendet",
        "connected": "Verbunden",
        "unkown": "Unbekannt",
        "tooltips": {
          "disconnect": "Trennen",
        }
      }
    }
  }
}

export default locale;