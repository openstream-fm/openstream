export type AdminLocale = typeof import("./admin.en.js").default;

import en from "./admin.en.js";
import es from "./admin.es.js";
import es_AR from "./admin.es-AR.js";
import pt from "./admin.pt.js";
import de from "./admin.de.js";
import fr from "./admin.fr.js";
import it from "./admin.it.js";
import zh from "./admin.zh.js";
import ar from "./admin.ar.js";

export const admin_locales: AdminLocale[] = [ en, es, es_AR, pt, de, fr, it, zh, ar ];
export const admin_locales_map = new Map<string, AdminLocale>(Object.entries({
  en,
  es, 
  "es-AR": es_AR,
  pt,
  de,
  fr,
  it,
  zh,
  ar,
}))

export const default_admin_locale = en;