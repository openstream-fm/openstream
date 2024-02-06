export type AdminLocale = typeof import("./admin.en").default;

import en from "./admin.en";
import es from "./admin.es";
import es_AR from "./admin.es-AR";
import pt from "./admin.pt";
import de from "./admin.de";
import fr from "./admin.fr";
import it from "./admin.it";
import zh from "./admin.zh";
import ar from "./admin.ar";

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