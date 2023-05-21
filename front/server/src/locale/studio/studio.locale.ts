export type StudioLocale = typeof import("./studio.en").default;

import en from "./studio.en";
import es from "./studio.es";
import pt from "./studio.pt";

export const studio_locales = { en, es, pt };