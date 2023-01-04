import { load_get_me } from "$lib/load.server";
import { dev } from "$app/environment";

export const ssr = !dev;

export const load = (async ({ request, getClientAddress, depends }) => {
  depends("user:me");
  const maybeUser = await load_get_me({ request, getClientAddress });
  return { maybeUser }
}) satisfies import("./$types").LayoutServerLoad;