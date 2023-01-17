import { load_get } from "$lib/load.server";

export const load = (async ({ getClientAddress, request, params, depends }) => {
  depends("station:dashboard-stats");
  const dashboard_stats: import("$server/defs/api/stations/[station]/dashboard-stats/GET/Output").Output = await load_get(`/api/stations/${params.station}/dashboard-stats`, { getClientAddress, request });
  return { dashboard_stats }

}) satisfies import("./$types").PageServerLoad;