import { derived, type Readable } from "svelte/store";
import type { AdminLocale } from "$server/locale/admin/admin.locale";
import { readable } from "svelte/store";
import _locale from "$server/locale/admin/admin.en";

export const locale: Readable<AdminLocale> = readable(_locale);

export const lang = derived(locale, $locale => {
  return $locale.region == null ? $locale.lang : `${$locale.lang}-${$locale.region}`
})