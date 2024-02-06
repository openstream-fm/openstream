import { derived } from "svelte/store";
import { default_studio_locale, type StudioLocale } from "$server/locale/studio/studio.locale";
import type { Readable } from "svelte/store";
import { page } from "$app/stores";

export const locale: Readable<StudioLocale> = derived(page, $page => {
  return $page?.data?.locale ?? default_studio_locale;
})

export const lang: Readable<string> = derived(locale, $locale => {
  if($locale.region != null) return `${$locale.lang}-${$locale.region}`
  else return $locale.lang;
})

export const dir: Readable<string> = derived(lang, $lang => {
  if($lang === "ar") return "rtl";
  else return "ltr";
})