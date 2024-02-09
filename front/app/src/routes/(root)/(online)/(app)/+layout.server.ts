import { load_call, client } from "$lib/load";
import { redirect } from "@sveltejs/kit";

export const load = (async ({ fetch, url, parent, depends }) => {

   depends("resource:accounts", "resource:stations");
   depends("api:accounts", "api:stations");
   
   const { maybe_user } = await parent();
   if (maybe_user == null) {
      const to = `${url.pathname}${url.search}`;
      const login_url = to === "/" ? "/login" : `/login#${to}`
      redirect(302, login_url);
   }

   // TODO: implement pagination
   const [
      accounts,
      stations,
   ] = await Promise.all([
      load_call(() => client.GET("/accounts", { params: { query: { limit: 10_000 } }, fetch })),
      load_call(() => client.GET("/stations", { params: { query: { limit: 10_000 } }, fetch })),
   ])


   return { user: maybe_user, accounts, stations }

}) satisfies import("./$types").LayoutServerLoad;