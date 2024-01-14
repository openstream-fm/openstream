import stats_map from "../share/stats-map/stats-map.en"
import validate from "../share/validate/validate.en";
import analytics from "../share/analytics/analytics.en";
import countries from "../share/countries/countries.en";
import langs from "../share/langs/langs.en";
import misc from "../misc/misc.en";

const locale = {

  "lang": "en",
  "region": null,

  // @notranslate
  "logo_text": "openstream",

  // @notranslate
  "app_name": "Openstream Admin",

  "show_password": "Show password",
  "hide_password": "Hide password",

  "validate": validate,
  "stats_map": stats_map,
  "analytics": analytics,
  "countries": countries,
  "langs": langs,
  "misc": misc,

  "pages": {
    "me": {
      "title": "Profile",
      "fields": {
        "email": "Your email",
        "first_name": "Your first name",
        "last_name": "Your last name",
        "phone": "Your phone number",
        "current_password": "Current password",
        "new_password": "New password",
        "confirm_password": "Confirm password",
        "language": "Preferred language",
      },
      "submit": {
        "profile": "Save",
        "password": "Save",
      },
      "change_password": {
        "title": "Change your password",
      },
      "more": {
        "title": "More",
        "connected_devices": "Connected devices",
      },
      "notifier": {
        "no_changes": "No changes to save",
        "profile_updated": "Profile updated",
        "password_updated": "Password updated",
      }
    },

    "me/devices": {
      "head": {
        "title": "Devices",
      },
      "title": "Connected devices",
      "note": "The same device may appear more than once in this list. Devices will be disconnected after 7 days without usage.",
      "dialogs": {
        "disconnect": {
          "title": "Disconnect device",
          "message": "This action is permanent",
          "cancel": "Cancel",
          "submit": "Disconnect",
        },
      },

      "notifier": {
        "device_disconnected": "Device disconnected",
      },

      "device": {
        "browser": "Browser",
        "os": "System",
        "ip": "IP",
        "last_used": "Last used",
        "connected": "Connected",
        "unkown": "Unknown",
        "tooltips": {
          "disconnect": "Disconnect",
        }
      }
    }
  }
}

export default locale;