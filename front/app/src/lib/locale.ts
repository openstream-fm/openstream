import en from "$server/locale/en";
import { readable } from "svelte/store";
import type { Locale } from "$server/locale/locale";
import type { Readable } from "svelte/store";

export const locale: Readable<Locale> = readable(es);