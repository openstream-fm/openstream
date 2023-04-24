import { load_get } from "$lib/load";

export const load = (async ({ fetch, url, depends, params }) => {
  
  depends("resource:stations")
  // depends("api:stations/:id/dashboard-stats");
  depends("api:stations/:id/stream-stats");
  depends("api:stations/:id/now-playing");

  const [
    now_playing,
    stats,
  ] = await Promise.all([
    load_get<import("$server/defs/api/stations/[station]/now-playing/GET/Output").Output>(`/api/stations/${params.station}/now-playing`, { fetch, url }),
    load_get<import("$server/defs/api/stations/[station]/stream-stats/GET/Output").Output>(`/api/stations/${params.station}/stream-stats`, { fetch, url }),
  ]);
  
  return { 
    now_playing,
    stats: stats.stats,
  }

}) satisfies import("./$types").PageLoad;