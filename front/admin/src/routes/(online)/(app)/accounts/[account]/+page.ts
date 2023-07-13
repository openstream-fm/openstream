import { load_get } from "$lib/load";
import { error } from "@sveltejs/kit";

export const load = (async ({ parent, params, fetch, url }) => {
  
  const { stations, all_accounts, all_plans } = await parent();

  const account = all_accounts.find(item => item._id === params.account);

  if(account == null) throw error(404, { status: 404, message: "Account not found", code: "FRONT_RESOURCE_NOT_FOUND" })

  const plan = all_plans.find(plan => plan._id === account.plan_id);

  const account_stations = stations.filter(item => {
    return item.account_id === account._id;
  })

  const [
    { members },
    { stats }
  ] = await Promise.all([
    load_get<import("$server/defs/api/accounts/[account]/members/GET/Output").Output>(`/api/accounts/${account._id}/members`, { fetch, url }),
    load_get<import("$api/accounts/[account]/stream-stats/GET/Output").Output>(`/api/accounts/${params.account}/stream-stats`, { fetch, url }),
  ]);

  return { account, plan, members, stats, account_stations }

}) satisfies import("./$types").PageLoad;
