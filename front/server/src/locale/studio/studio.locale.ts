export type StudioLocale = typeof import("./studio.en.js").default;

import en from "./studio.en.js";
import es from "./studio.es.js";
import es_AR from "./studio.es-AR.js";
import pt from "./studio.pt.js";
import de from "./studio.de.js";
import fr from "./studio.fr.js";
import it from "./studio.it.js";
import zh from "./studio.zh.js";
import ar from "./studio.ar.js";

export const studio_locales: StudioLocale[] = [ en, es, es_AR, pt, de, fr, it, zh, ar ];
export const studio_locales_map = new Map<string, StudioLocale>(Object.entries({
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

export const default_studio_locale = en;