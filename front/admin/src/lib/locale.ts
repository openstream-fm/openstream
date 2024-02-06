import { derived } from "svelte/store";
import type { Readable } from "svelte/store";
import { page } from "$app/stores";
import { default_admin_locale, type AdminLocale } from "$server/locale/admin/admin.locale";

export const locale: Readable<AdminLocale> = derived(page, $page => {
  return $page?.data?.locale ?? default_admin_locale;
})

export const lang: Readable<string> = derived(locale, $locale => {
  if($locale.region != null) return `${$locale.lang}-${$locale.region}`
  else return $locale.lang;
})

export const dir: Readable<string> = derived(lang, $lang => {
  if($lang === "ar") return "rtl";
  else return "ltr";
})