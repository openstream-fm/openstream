 import type { LayoutServerLoad } from "./$types";
 import { load_get } from "$lib/load.server";

 export const load = (async ({ request, getClientAddress }) => {
   const { user }: import("$server/defs/api/users/[user]/GET/Output").Output = await load_get("/api/users/me", { request, getClientAddress });
   const accounts: import("$server/defs/api/accounts/GET/Output").Output = await load_get("/api/accounts?limit=1000", { request, getClientAddress });
   return { user, accounts }
}) satisfies LayoutServerLoad;