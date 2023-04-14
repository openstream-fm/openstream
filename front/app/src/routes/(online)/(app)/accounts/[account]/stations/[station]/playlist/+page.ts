import { load_get } from "$lib/load";

export const load = (async ({ params, fetch, depends, url }) => {
  depends("resource:stations")
  depends("api:stations/:id/now-playing");
  const now_playing: import("$server/defs/api/stations/[station]/now-playing/GET/Output").Output = await load_get(`/api/stations/${params.station}/now-playing`, { fetch, url })
  return { now_playing }
}) satisfies import("./$types").PageLoad;