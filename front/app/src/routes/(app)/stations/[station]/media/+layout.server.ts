import { load_get } from "$lib/load.server";

export const load = (async ({ depends, params, getClientAddress, request }) => {
  depends("station:files");
  // TODO: implement pagination
  const files: import("$server/defs/api/stations/[station]/files/GET/Output").Output = await load_get(`/api/stations/${params.station}/files?limit=10000`, { request, getClientAddress })
  return { files };
}) satisfies import("./$types").LayoutServerLoad;