import { load_get } from "$lib/load";

export const load = (async ({ depends, fetch, url, params }) => {
  depends("resource:plans");
  depends("api:plans/:id")
  const { plan } = await load_get<import("$api/plans/[plan]/GET/Output").Output>(`/api/plans/${params.plan}`, { fetch, url }); 
  return { plan };
}) satisfies import("./$types").PageLoad;