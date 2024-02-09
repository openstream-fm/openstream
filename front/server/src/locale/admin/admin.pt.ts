import stats_map from "../share/stats-map/stats-map.pt.js";
import validate from "../share/validate/validate.pt.js";
import analytics from "../share/analytics/analytics.pt.js";
import countries from "../share/countries/countries.pt.js";
import langs from "../share/langs/langs.pt.js";
import misc from "../misc/misc.pt.js";
import language from "../share/language/language.pt.js";

const locale: import("./admin.locale.js").AdminLocale = {

  "lang": "pt",
  "region": null,

  // @notranslate
  "logo_text": "openstream",

  // @notranslate
  "app_name": "Openstream Admin",

  "show_password": "Mostrar senha",
  "hide_password": "Ocultar senha",

  "validate": validate,
  "stats_map": stats_map,
  "analytics": analytics,
  "countries": countries,
  "langs": langs,
  "misc": misc,
  "language": language,

  "pages": {
    "me": {
      "title": "Perfil",
      "fields": {
        "email": "Seu email",
        "first_name": "Seu nome",
        "last_name": "Seu sobrenome",
        "phone": "Seu telefone",
        "current_password": "Senha atual",
        "new_password": "Nova senha",
        "confirm_password": "Confirmar senha",
        "language": "Idioma preferido",
      },
      "submit": {
        "profile": "Salvar",
        "password": "Salvar",
      },
      "change_password": {
        "title": "Mude sua senha",
      },
      "more": {
        "title": "Mais",
        "connected_devices": "Dispositivos conectados",
      },
      "notifier": {
        "no_changes": "Sem alterações para salvar",
        "profile_updated": "Perfil atualizado",
        "password_updated": "Senha atualizada",
      }
    },

    "me/devices": {
      "head": {
        "title": "Dispositivos",
      },
      "title": "Dispositivos conectados",
      "note": "O mesmo dispositivo pode aparecer mais de uma vez nesta lista. Os dispositivos serão desconectados após 7 dias sem atividade.",
      "dialogs": {
        "disconnect": {
          "title": "Desconectar dispositivo",
          "message": "Esta ação é permanente.",
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
        "last_used": "Usado pela última vez",
        "connected": "Conectado",
        "unkown": "Desconhecido",
        "tooltips": {
          "disconnect": "Desconectar",
        }
      }
    }
  }
}

export default locale;