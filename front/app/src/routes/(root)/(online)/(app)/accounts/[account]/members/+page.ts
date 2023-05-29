import { load_get } from "$lib/load";

export const load = (async ({ fetch, url, depends, params }) => {
  
  depends("api:accounts/:id/members");
  
  const { members } = await load_get<import("$api/accounts/[account]/members/GET/Output").Output>(`/api/accounts/${params.account}/members`, { fetch, url });
  
  return { 
    members,
  }

}) satisfies import("./$types").PageLoad;