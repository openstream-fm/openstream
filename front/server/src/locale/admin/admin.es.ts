import stats_map from "../share/stats-map/stats-map.es";
import validate from "../share/validate/validate.es";
import analytics from "../share/analytics/analytics.es";
import countries from "../share/countries/countries.es";

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

}

export default locale;