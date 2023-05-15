import { load_get } from "$lib/load";

export const load = (async ({ fetch, url, depends }) => {

   depends("resource:devices");
   depends("api:devices")

   // TODO: implement pagination
   const devices = await load_get<import("$api/devices/GET/Output").Output>(`/api/devices?limit=10000`, { fetch, url });
   
   return { devices }

}) satisfies import("./$types").PageLoad;