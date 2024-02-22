import { load_call, client } from "$lib/load";

export const load = (async ({ fetch, depends }) => {

   depends("resource:invitations");
   depends("api:invitations");

   // TODO: implement pagination
   const invitations = await load_call(() => client.GET("/invitations", { params: { query: { limit: 10_000 } }, fetch }));
   return { invitations }
   
}) satisfies import("./$types").PageServerLoad;