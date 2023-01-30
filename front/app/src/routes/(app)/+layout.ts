import { load_get } from "$lib/load";
import { redirect } from "@sveltejs/kit";

export const load = (async ({ fetch, url, parent, depends }) => {

   depends("station:limits")
   const { maybe_user } = await parent();
   if (maybe_user == null) throw redirect(302, "/login");
   // TODO: implement pagination
   const stations: import("$server/defs/api/stations/GET/Output").Output = await load_get("/api/stations?limit=10000", { fetch, url });
   return { user: maybe_user, stations }

}) satisfies import("./$types").LayoutLoad;