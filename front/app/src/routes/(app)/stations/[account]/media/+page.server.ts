import { load_get } from "$lib/load.server";

export const load = (async ({ params, getClientAddress, request }) => {
  const now_playing: import("$server/defs/api/accounts/[account]/now-playing/GET/Output").Output = await load_get(`/api/accounts/${params.account}/now-playing`, { request, getClientAddress })
  return { now_playing }
}) satisfies import("./$types").PageServerLoad;