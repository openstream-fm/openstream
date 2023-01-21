import { load_get } from "$lib/load.server";

export const load = (async ({ getClientAddress, request, params, depends }) => {
  
  depends("station:dashboard");
  depends("station:limits");
  
  const [
    dashboard_stats,
    now_playing
  ] = await Promise.all([
    load_get<import("$server/defs/api/stations/[station]/dashboard-stats/GET/Output").Output>(`/api/stations/${params.station}/dashboard-stats`, { getClientAddress, request }),
    load_get<import("$server/defs/api/stations/[station]/now-playing/GET/Output").Output>(`/api/stations/${params.station}/now-playing`, { getClientAddress, request }),
  ]);
  
  return { dashboard_stats, now_playing }

}) satisfies import("./$types").PageServerLoad;