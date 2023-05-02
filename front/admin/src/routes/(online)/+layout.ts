import { load_get, load_get_me } from "$lib/load";

export const load = (async ({ depends, fetch, url }) => {
  depends("resource:admins", "resource:config");
  depends("api:config", "api:admins/me");
  return { 
    config: load_get<import("$server/api/admin-api").PublicConfig>("/api/config", { fetch, url }),
    maybe_admin: load_get_me({ fetch, url }),
  }
}) satisfies import("./$types").LayoutLoad;