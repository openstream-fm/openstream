import { client, load_call } from "$lib/load";

export const load = (async ({ depends, fetch, params }) => {
  depends("resource:plans");
  depends("api:plans/by-slug/:slug")
  const { plan } = await load_call(() => client.GET("/plans/by-slug/{slug}", { params: { path: { slug: params.plan } }, fetch })); 
  const payment_methods = await load_call(() => client.GET("/payment-methods", { fetch }))
  return { plan, payment_methods };
}) satisfies import("./$types").PageServerLoad;