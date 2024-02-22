import { client, load_call } from "$lib/load";

export const load = (async ({ fetch, depends }) => {

   depends("resource:devices");
   depends("api:devices")

   // TODO: implement pagination
   const devices = await load_call(
      () => client.GET("/me/devices", { params: { query: { limit: 100_000 } }, fetch })
   );
   
   return { devices }

}) satisfies import("./$types").PageServerLoad;