import { load_get } from "$lib/load.server";

export const load = (async ({ getClientAddress, request, params, depends }) => {
  depends("account:dashboard-stats");
  const dashboard_stats: import("$server/defs/api/accounts/[account]/dashboard-stats/GET/Output").Output = await load_get(`/api/accounts/${params.account}/dashboard-stats`, { getClientAddress, request });
  return { dashboard_stats }

}) satisfies import("./$types").PageServerLoad;