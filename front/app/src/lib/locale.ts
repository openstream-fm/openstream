// import _locale from "$server/locale/studio/studio.en";
import { derived } from "svelte/store";
import type { StudioLocale } from "$server/locale/studio/studio.locale";
import type { Readable } from "svelte/store";
import { page } from "$app/stores";

export const locale: Readable<StudioLocale> = derived(page, $page => {
  return $page.data.locale
})

export const lang: Readable<String> = derived(page, $page => {
  return $page.data.lang
})