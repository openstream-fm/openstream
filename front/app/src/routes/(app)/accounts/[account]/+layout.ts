import { load_get } from "$lib/load";

export const load = (async ({ fetch, url, parent, depends, params }) => {

  depends("accounts:item");
  depends("account:stations:list");

  const { accounts } = await parent();
  
  let account: import("$server/defs/api/accounts/[account]/GET/Output").Output["account"];

  let item = accounts.items.find(item => item._id === params.account);

  if(item != null) {
    account = item;
  } else {
    const res: import("$server/defs/api/accounts/[account]/GET/Output").Output = await load_get(`/api/accounts/${params.account}`, { fetch, url });
    account = res.account;
  }

  const stations: import("$server/defs/api/stations/GET/Output").Output = await load_get(`/api/stations?limit=10000&account_id=${account._id}`, { fetch, url });

  return {
    account,
    stations,
    station: null,
  }

}) satisfies import("./$types").LayoutLoad;