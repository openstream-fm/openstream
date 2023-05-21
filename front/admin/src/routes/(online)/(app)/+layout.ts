import { load_get } from "$lib/load";
import { redirect } from "@sveltejs/kit";

export const load = (async ({ fetch, url, parent, depends }) => {

   depends("resource:plans", "resource:admins", "resource:users", "resource:accounts", "resource:stations");
   depends("api:admins", "api:users","api:accounts", "api:stations");
   
   const { maybe_admin } = await parent();
   if (maybe_admin == null) {
      const to = `${url.pathname}${url.search}`;
      const login_url = to === "/" ? "/login" : `/login#${encodeURIComponent(to)}`
      throw redirect(302, login_url);
   }

   return {
      admin: maybe_admin,
      plans: load_get<import("$api/plans/GET/Output").Output>(`/api/plans?show=all`, { fetch, url }),
      admins: load_get<import("$api/admins/GET/Output").Output>(`/api/admins?limit=100000`, { fetch, url }),
      users: load_get<import("$api/users/GET/Output").Output>(`/api/users?limit=100000`, { fetch, url }),
      accounts: load_get<import("$api/accounts/GET/Output").Output>(`/api/accounts?limit=100000`, { fetch, url }),
      stations: load_get<import("$api/stations/GET/Output").Output>(`/api/stations?limit=100000`, { fetch, url }),
   }

}) satisfies import("./$types").LayoutLoad;