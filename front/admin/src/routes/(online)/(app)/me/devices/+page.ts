import { load_get } from "$lib/load";
import { qss } from "$share/qs";

export const load = (async ({ fetch, url, depends }) => {

   depends("resource:devices");
   depends("api:devices")

   // TODO: implement pagination
   const devices = await load_get<import("$api/me/devices/GET/Output").Output>(
      `/api/me/devices${qss<import("$api/me/devices/GET/Query").Query>({ limit: 100_000 })}`, 
      { fetch, url }
   );
   
   return { devices }

}) satisfies import("./$types").PageLoad;