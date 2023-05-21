import _locale from "$server/locale/studio/studio.en";
import { readable } from "svelte/store";
import type { StudioLocale } from "$server/locale/studio/studio.locale";
import type { Readable } from "svelte/store";

export const locale: Readable<StudioLocale> = readable(_locale);