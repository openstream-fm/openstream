import { load_get, load_get_me } from "$lib/load";

export const load = (async ({ depends, fetch, url }) => {
  depends("resource:admins", "resource:config");
  depends("api:config", "api:admins/me");
  
  const [
    config,
    { locale },
    maybe_admin,
  ] = await Promise.all([
    load_get<import("$server/api/studio-api").PublicConfig>("/api/config", { fetch, url }),
    load_get<import("$server/api/studio-api").LocalePayload>("/api/locale", { fetch, url }),
    load_get_me({ fetch, url }),
  ])

  return { 
    config, locale, maybe_admin,
  }
}) satisfies import("./$types").LayoutLoad;