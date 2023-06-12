import { load_get } from "$lib/load";
import { redirect } from "@sveltejs/kit";

export const load = (async ({ fetch, url, parent, depends }) => {

   depends("resource:accounts", "resource:stations");
   depends("api:accounts", "api:stations");
   
   const { maybe_user } = await parent();
   if (maybe_user == null) {
      const to = `${url.pathname}${url.search}`;
      const login_url = to === "/" ? "/login" : `/login#${to}`
      throw redirect(302, login_url);
   }

   // TODO: implement pagination
   const [
      accounts,
      stations,
   ] = await Promise.all([
      load_get<import("$api/accounts/GET/Output").Output>(`/api/accounts?limit=10000`, { fetch, url }),
      load_get<import("$api/stations/GET/Output").Output>(`/api/stations?limit=10000`, { fetch, url }),
   ])


   return { user: maybe_user, accounts, stations }

}) satisfies import("./$types").LayoutLoad;