import { load_get } from "$lib/load.server";

export const load = (async ({ params, getClientAddress, request }) => {
  const now_playing: import("$server/defs/api/stations/[station]/now-playing/GET/Output").Output = await load_get(`/api/stations/${params.station}/now-playing`, { request, getClientAddress })
  return { now_playing }
}) satisfies import("./$types").PageServerLoad;