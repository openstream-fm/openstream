import { load_get } from "$lib/load";

export const load = (async ({ fetch, url, depends, params }) => {

  depends("api:accounts/:id");

  const { account, is_owner }: import("$api/accounts/[account]/GET/Output").Output = await load_get(`/api/accounts/${params.account}`, { fetch, url });
  
  return {
    account,
    is_account_owner: is_owner,
    station: null,
  }

}) satisfies import("./$types").LayoutServerLoad;