import { load_get } from "$lib/load";

export const load = (async ({ fetch, url, depends }) => {

   depends("resource:api-keys");
   depends("api:api-keys")

   // TODO: implement pagination
   const api_keys = await load_get<import("$api/me/api-keys/GET/Output").Output>(`/api/me/api-keys?limit=10000`, { fetch, url });
   
   return { api_keys }

}) satisfies import("./$types").PageServerLoad;