import { load_get } from "$lib/load";
import { error } from "@sveltejs/kit";

export const load = (async ({ parent, params, fetch, url }) => {
  
  const { users, stations } = await parent();

  const user = users.items.find(item => item._id === params.user);

  if(user == null) throw error(404, { status: 404, message: "User not found", code: "FRONT_RESOURCE_NOT_FOUND" })

  const query = ({
    user_id: user._id
  }) satisfies import("$server/defs/api/accounts/GET/Query").Query;

  const qs = new URLSearchParams();
  qs.append("user_id", query.user_id);

  const user_accounts = await load_get<import("$server/defs/api/accounts/GET/Output").Output>(`/api/accounts?${qs}`, { fetch, url })

  const user_stations = stations.items.filter(station => {
    return user_accounts.items.some(account => station.account_id === account._id);
  })

  return { user, user_accounts, user_stations }

}) satisfies import("./$types").PageLoad;
