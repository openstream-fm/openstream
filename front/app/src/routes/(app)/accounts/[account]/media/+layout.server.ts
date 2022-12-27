import { load_get } from "$lib/load.server";
import type { LayoutServerLoad } from "./$types";

export const load = (async ({ depends, params, getClientAddress, request }) => {
  depends("api:files");
  const files: import("$server/defs/api/accounts/[account]/files/GET/Output").Output = await load_get(`/api/accounts/${params.account}/files?limit=1000`, { request, getClientAddress })
  return { files };
}) satisfies LayoutServerLoad;