import { client, load_call } from "$lib/load";

export const load = (async ({ fetch, depends, params }) => {

  depends("api:accounts/:id");

  const { account, is_owner } = await load_call(async () => await client.GET("/accounts/{account}", {
    params: { path: { account: params.account } },
    fetch,
  }));
  
  return {
    account,
    is_account_owner: is_owner,
    station: null,
  }

}) satisfies import("./$types").LayoutServerLoad;