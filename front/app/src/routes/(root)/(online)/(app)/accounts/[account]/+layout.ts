import { load_get } from "$lib/load";

export const load = (async ({ fetch, url, parent, depends, params }) => {

  depends("api:accounts/:id");

  const { accounts } = await parent();
  
  let account: import("$api/accounts/[account]/GET/Output").Output["account"];

  let item = accounts.items.find(item => item._id === params.account);

  if(item != null) {
    account = item;
  } else {
    const res: import("$api/accounts/[account]/GET/Output").Output = await load_get(`/api/accounts/${params.account}`, { fetch, url });
    account = res.account;
  }

  return {
    account,
    station: null,
  }

}) satisfies import("./$types").LayoutLoad;