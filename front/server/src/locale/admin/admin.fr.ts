import stats_map from "../share/stats-map/stats-map.fr.js";
import validate from "../share/validate/validate.fr.js";
import analytics from "../share/analytics/analytics.fr.js";
import countries from "../share/countries/countries.fr.js";
import langs from "../share/langs/langs.fr.js";
import misc from "../misc/misc.fr.js";
import language from "../share/language/language.fr.js";

const locale: import("./admin.locale.js").AdminLocale = {

  "lang": "fr",
  "region": null,

  // @notranslate
  "logo_text": "openstream",

  // @notranslate
  "app_name": "Openstream Admin",

  "show_password": "Afficher le mot de passe",
  "hide_password": "Cacher le mot de passe",

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
        "email": "Votre email",
        "first_name": "Votre prénom",
        "last_name": "Votre nom",
        "phone": "Votre téléphone",
        "current_password": "Mot de passe actuel",
        "new_password": "Nouveau mot de passe",
        "confirm_password": "Confirmer le mot de passe",
        "language": "Langue préférée",
      },
      "submit": {
        "profile": "Enregistrer",
        "password": "Enregistrer",
      },
      "change_password": {
        "title": "Changez votre mot de passe",
      },
      "more": {
        "title": "Plus",
        "connected_devices": "Appareils connectés",
      },
      "notifier": {
        "no_changes": "Aucun changement à enregistrer",
        "profile_updated": "Profil mis à jour",
        "password_updated": "Mot de passe mis à jour",
      }
    },

    "me/devices": {
      "head": {
        "title": "Appareils",
      },
      "title": "Appareils connectés",
      "note": "Le même appareil peut apparaître plus d'une fois dans cette liste. Les appareils seront déconnectés après 7 jours d'inactivité.",
      "dialogs": {
        "disconnect": {
          "title": "Déconnecter l'appareil",
          "message": "Cette action est définitive.",
          "cancel": "Annuler",
          "submit": "Déconnecter",
        },
      },

      "notifier": {
        "device_disconnected": "Appareil déconnecté",
      },

      "device": {
        "browser": "Navigateur",
        "os": "Système",
        "ip": "IP",
        "last_used": "Dernière utilisation",
        "connected": "Connecté",
        "unkown": "Inconnu",
        "tooltips": {
          "disconnect": "Déconnecter",
        }
      }
    }
  }
}

export default locale;