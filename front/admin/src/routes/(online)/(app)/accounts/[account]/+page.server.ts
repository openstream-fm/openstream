import { client, load_call } from "$lib/load";
import { error } from "@sveltejs/kit";

export const load = (async ({ parent, params, fetch }) => {
  
  const { stations, all_accounts, all_plans } = await parent();

  const account = all_accounts.find(item => item._id === params.account);

  if(account == null) error(404, { status: 404, message: "Account not found", code: "FRONT_RESOURCE_NOT_FOUND" });

  const plan = all_plans.find(plan => plan._id === account.plan_id);

  const account_stations = stations.filter(item => {
    return item.account_id === account._id;
  })

  const [
    { members },
    { stats }
  ] = await Promise.all([
    load_call(() => client.GET("/accounts/{account}/members", { params: { path: { account: account._id } }, fetch })),
    load_call(() => client.GET("/accounts/{account}/stream-stats", { params: { path: { account: account._id } }, fetch })),
  ]);

  return { account, plan, members, stats, account_stations }

}) satisfies import("./$types").PageServerLoad;
