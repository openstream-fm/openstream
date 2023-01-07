 import { load_get } from "$lib/load.server";
 import { redirect } from "@sveltejs/kit";

 export const load = (async ({ request, getClientAddress, parent, depends }) => {

  depends("account:limits")
   const { maybe_user } = await parent();
   if(maybe_user == null) throw redirect(302, "/login");
   // TODO: implement pagination
   const accounts: import("$server/defs/api/accounts/GET/Output").Output = await load_get("/api/accounts?limit=10000", { request, getClientAddress });
   return { user: maybe_user, accounts }

}) satisfies import("./$types").LayoutServerLoad;