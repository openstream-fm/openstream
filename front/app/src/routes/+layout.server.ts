import { load_get, load_get_me } from "$lib/load.server";
import { dev } from "$app/environment";

export const ssr = !dev;

export const load = (async ({ request, getClientAddress, depends }) => {
  depends("user:me");
  const maybe_user = await load_get_me({ request, getClientAddress });
  const config: import("$server/config").Config["public"] = await load_get("/api/config", { request, getClientAddress });
  return { config, maybe_user };
}) satisfies import("./$types").LayoutServerLoad;