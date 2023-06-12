import { load_get } from "$lib/load";

export const load = (async ({ fetch, url, depends }) => {

   depends("resource:invitations");
   depends("api:invitations");

   // TODO: implement pagination
   const invitations = await load_get<import("$api/invitations/GET/Output").Output>(`/api/invitations?limit=10000`, { fetch, url });
   
   return { invitations }

}) satisfies import("./$types").PageLoad;