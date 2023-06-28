import { load_get } from "$lib/load";

export const load = (async ({ depends, fetch, url, params }) => {
  depends("resource:plans");
  depends("api:plans/by-slug/:slug")
  const { plan } = await load_get<import("$api/plans/[plan]/GET/Output").Output>(`/api/plans/by-slug/${params.plan}`, { fetch, url }); 
  const payment_methods = await load_get<import("$api/payment-methods/GET/Output").Output>("/api/payment-methods", { fetch, url });
  return { plan, payment_methods };
}) satisfies import("./$types").PageServerLoad;