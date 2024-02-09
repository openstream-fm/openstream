import { client, load_call } from "$lib/load";

export const load = (async ({ fetch, depends, parent }) => {
  
  depends("api:accounts/:id/members");
  depends("api:invitations");
  
  const { account, is_account_owner } = await parent();

  if (is_account_owner) {  
    const { members } = await load_call(() => client.GET("/accounts/{account}/members", { params: { path: { account: account._id } }, fetch }));
    const invitations = await load_call(() => client.GET("/invitations", { params: { query: { account_id: account._id, limit: 10_000 } }, fetch }))

    const access = { is_owner: true, members, invitations }
    return { access }
  } else {
    const access = { is_owner: false, members: null, invitations: null }
    return { access }
  }
}) satisfies import("./$types").PageServerLoad;