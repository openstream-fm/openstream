import { load_call, client } from "$lib/load";

export const load = (async ({ fetch, depends }) => {

   depends("resource:devices");
   depends("api:devices")

   // TODO: implement pagination
   const devices = await load_call(() => client.GET("/me/devices", { params: { query: { limit: 10_000 } }, fetch }));
   
   return { devices }

}) satisfies import("./$types").PageServerLoad;