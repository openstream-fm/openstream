export type StudioLocale = typeof import("./studio.en").default;

import en from "./studio.en";
import es from "./studio.es";
import pt from "./studio.pt";
import fr from "./studio.fr";

export const studio_locales = { en, es, pt, fr };