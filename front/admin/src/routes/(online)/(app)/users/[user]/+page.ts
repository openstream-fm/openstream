import { load_get } from "$lib/load";
import { qss } from "$share/qs";
import { error } from "@sveltejs/kit";

export const load = (async ({ parent, params, fetch, url }) => {
  
  const { all_users, all_stations } = await parent();

  const user = all_users.find(item => item._id === params.user);

  if(user == null) error(404, { status: 404, message: "User not found", code: "FRONT_RESOURCE_NOT_FOUND" });

  const query = ({
    show: "all",
    user_id: user._id
  }) satisfies import("$server/defs/api/accounts/GET/Query").Query;

  const user_accounts = await load_get<import("$server/defs/api/accounts/GET/Output").Output>(`/api/accounts?${qss(query)}`, { fetch, url })
  
  const user_stations = all_stations.filter(station => {
    return user_accounts.items.some(account => station.account_id === account._id);
  })

  return { user, user_accounts, user_stations }

}) satisfies import("./$types").PageLoad;
