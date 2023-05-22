export type StudioLocale = typeof import("./studio.en").default;

import en from "./studio.en";
import es from "./studio.es";
import es_AR from "./studio.es-AR";
import pt from "./studio.pt";
import de from "./studio.de";
import fr from "./studio.fr";

export const studio_locales = { en, es, es_AR, pt, de, fr };