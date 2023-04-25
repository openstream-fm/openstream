import { load_get } from "$lib/load";

export const load = (async ({ fetch, url, depends, params }) => {
  
  depends("api:accounts/:id/stream-stats");
  
  const { stats } = await load_get<import("$server/defs/api/accounts/[account]/stream-stats/GET/Output").Output>(`/api/accounts/${params.account}/stream-stats`, { fetch, url });
  
  return { 
    stats: stats,
  }

}) satisfies import("./$types").PageLoad;