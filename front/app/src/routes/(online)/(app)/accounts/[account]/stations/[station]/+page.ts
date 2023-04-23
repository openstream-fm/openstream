import { load_get } from "$lib/load";

export const load = (async ({ fetch, url, depends, params }) => {
  
  depends("resource:stations")
  // depends("api:stations/:id/dashboard-stats");
  depends("api:stations/:id/now-playing");
  
  const [
    // dashboard_stats,
    now_playing
  ] = await Promise.all([
    // load_get<import("$server/defs/api/stations/[station]/dashboard-stats/GET/Output").Output>(`/api/stations/${params.station}/dashboard-stats`, { fetch, url }),
    load_get<import("$server/defs/api/stations/[station]/now-playing/GET/Output").Output>(`/api/stations/${params.station}/now-playing`, { fetch, url }),
  ]);
  
  return { 
    // dashboard_stats,
    now_playing
  }

}) satisfies import("./$types").PageLoad;