import { load_get } from "$lib/load.server";
import type { LayoutServerLoad } from "./$types";

export const load = (async ({ getClientAddress, parent, request, params, depends }) => {

  depends("account:limits");

  const { accounts } = await parent();
  const account = accounts.items.find(item => item._id === params.account);

  if(account != null) {
    return { account }
  } 

  const helper: import("$server/defs/api/accounts/[account]/GET/Output").Output = await load_get(`/api/accounts/${params.account}`, { getClientAddress, request });
  return { account: helper.account }

}) satisfies LayoutServerLoad;