import type { StudioLocale } from "$server/locale/studio/studio.locale";
import type { AdminLocale } from "$server/locale/admin/admin.locale";

// @ts-ignore
import { locale as _locale } from "$lib/locale";
import type { Readable } from "svelte/store";

export type ShareLocale = StudioLocale | AdminLocale;

const locale = _locale as Readable<ShareLocale>;

export { locale }