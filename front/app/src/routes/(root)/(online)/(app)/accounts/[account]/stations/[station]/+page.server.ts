import { load_call, client } from "$lib/load";

export const load = (async ({ fetch, depends, params }) => {
  
  depends("resource:stations")
  // depends("api:stations/:id/dashboard-stats");
  depends("api:stations/:id/stream-stats");
  depends("api:stations/:id/now-playing");

  const [
    now_playing,
    { stats },
  ] = await Promise.all([
    load_call(() => client.GET(`/stations/{station}/now-playing`, { params: { path: { station: params.station } }, fetch })),
    load_call(() => client.GET("/stations/{station}/stream-stats", { params: { path: { station: params.station } }, fetch })),
  ]);
  
  return { 
    now_playing,
    stats,
    current_page: "dashboard",
  }

}) satisfies import("./$types").PageServerLoad;