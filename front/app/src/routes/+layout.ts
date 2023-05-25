export const ssr = true;

import { load_get, load_get_me } from "$lib/load";

export const load = (async ({ depends, fetch, url }) => {
  depends("resource:users", "resource:config", "resource:locale");
  depends("api:config", "api:locale", "api:users/me");
  
  const [
    config,
    maybe_user,
    { locale },
  ] = await Promise.all([
    load_get<import("$server/api/studio-api").PublicConfig>("/api/config", { fetch, url }),
    load_get_me({ fetch, url }),
    load_get<import("$server/api/studio-api").LocalePayload>("/api/locale", { fetch, url }),
  ])

  return { config, locale, maybe_user };

}) satisfies import("./$types").LayoutLoad;