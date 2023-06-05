import { load_get } from "$lib/load";
import { error } from "@sveltejs/kit";

export const load = (async ({ parent, params, fetch, url }) => {
  
  const { stations, accounts, plans } = await parent();

  const account = accounts.items.find(item => item._id === params.account);

  if(account == null) throw error(404, { status: 404, message: "Account not found", code: "FRONT_RESOURCE_NOT_FOUND" })

  const account_stations = stations.items.filter(item => {
    return item.account_id === account._id;
  })

  const { members } = await load_get<import("$server/defs/api/accounts/[account]/members/GET/Output").Output>(`/api/accounts/${account._id}/members`, { fetch, url })

  const plan = plans.items.find(plan => plan._id === account.plan_id);

  return { account, plan, members, account_stations }

}) satisfies import("./$types").PageLoad;
