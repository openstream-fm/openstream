import { load_get } from "$lib/load";

export const load = (async ({ fetch, url, depends, parent }) => {
  
  depends("api:accounts/:id/members");
  depends("api:invitations");
  
  const { account, is_account_owner } = await parent();

  type Members = import("$api/accounts/[account]/members/GET/Output").Output["members"];
  type Invitations = import("$api/invitations/GET/Output").Output;

  type Access = 
    | { is_owner: true, members: Members, invitations: Invitations }
    | { is_owner: false, members: null, invitations: null };

  let access: Access;

  if (is_account_owner) {  
    const { members } = (await load_get<import("$api/accounts/[account]/members/GET/Output").Output>(
      `/api/accounts/${account._id}/members`,
      { fetch, url }, 
    ))

    const query = { account_id: account._id, limit: 10000 } satisfies import("$api/invitations/GET/Query").Query;
    const qs = new URLSearchParams();
    qs.append("account_id", query.account_id);
    qs.append("limit", String(query.limit));

    const invitations = await load_get<import("$api/invitations/GET/Output").Output>(
      `/api/invitations?${qs}`,
      { fetch, url }
    )

    access = { is_owner: true, members, invitations }
  } else {
    access = { is_owner: false, members: null, invitations: null }
  }

  return { access }
}) satisfies import("./$types").PageLoad;