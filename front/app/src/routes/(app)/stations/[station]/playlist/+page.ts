import { load_get } from "$lib/load";

export const load = (async ({ params, fetch, url }) => {
  const now_playing: import("$server/defs/api/stations/[station]/now-playing/GET/Output").Output = await load_get(`/api/stations/${params.station}/now-playing`, { fetch, url })
  return { now_playing }
}) satisfies import("./$types").PageLoad;