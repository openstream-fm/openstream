import { load_call, client } from "$lib/load";

export const load = (async ({ fetch, depends }) => {

   depends("resource:api-keys");
   depends("api:api-keys")

   // TODO: implement pagination
   const api_keys = await load_call(() => client.GET("/me/api-keys", { params: { query: { limit: 10_000 } }, fetch }));;
   return { api_keys }

}) satisfies import("./$types").PageServerLoad;