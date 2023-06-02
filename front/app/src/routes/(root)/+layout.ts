export const ssr = true;

import { load_get, load_get_me } from "$lib/load";

export const load = (async ({ depends, fetch, url }) => {
  depends("resource:config", "resource:locale");
  depends("api:config", "api:locale");
  
  const [
    config,
    { locale },
  ] = await Promise.all([
    load_get<import("$server/api/studio-api").PublicConfig>("/api/config", { fetch, url }),
    load_get<import("$server/api/studio-api").LocalePayload>("/api/locale", { fetch, url }),
  ])

  return { config, locale }

}) satisfies import("./$types").LayoutLoad;