import { load_call, client } from "$lib/load";

export const load = (async ({ depends, fetch, params }) => {
  depends("resource:plans");
  depends("api:plans/by-slug/:slug")
  const { plan } = await load_call(() => client.GET("/plans/by-slug/{slug}", { params: { path: { slug: params.plan } }, fetch })); 
  return { plan };
}) satisfies import("./$types").PageServerLoad;