import { load_get } from "$lib/load";

export const load = (async ({ fetch, url, depends }) => {

  depends("api:plans")
  depends("resource:plans");

  const plans = await load_get<import("$server/defs/api/plans/GET/Output").Output>("/api/plans", { fetch, url });

  return { plans }

}) satisfies import("./$types").PageLoad;