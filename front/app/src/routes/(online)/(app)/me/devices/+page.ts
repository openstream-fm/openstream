import { load_get } from "$lib/load";

export const load = (async ({ fetch, url, depends }) => {

   depends("me:devices")
   
   // TODO: implement pagination
   const devices = await load_get<import("$server/defs/api/devices/GET/Output").Output>(`/api/devices?limit=10000`, { fetch, url });
   
   return { devices }

}) satisfies import("./$types").PageLoad;