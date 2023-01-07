import { load_get } from "$lib/load.server";

export const load = (async ({ depends, params, getClientAddress, request }) => {
  depends("account:files");
  // TODO: implement pagination
  const files: import("$server/defs/api/accounts/[account]/files/GET/Output").Output = await load_get(`/api/accounts/${params.account}/files?limit=10000`, { request, getClientAddress })
  return { files };
}) satisfies import("./$types").LayoutServerLoad;