import { load_get } from "$lib/load";

export const load = (async ({ fetch, url }) => {
  
  const { stats } = await load_get<import("$server/defs/api/stream-stats/GET/Output").Output>("/api/stream-stats", { fetch, url });

  return { stats }

}) satisfies import("./$types").PageLoad;