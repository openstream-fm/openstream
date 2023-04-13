import { load_get, load_get_me } from "$lib/load";

export const load = (async ({ depends, fetch, url }) => {
  depends("user:me");
  const config: import("$server/api/app-api").PublicConfig = await load_get("/api/config", { fetch, url }) 
  const maybe_user = await load_get_me({ fetch, url });
  return { config, maybe_user };
}) satisfies import("./$types").LayoutLoad;