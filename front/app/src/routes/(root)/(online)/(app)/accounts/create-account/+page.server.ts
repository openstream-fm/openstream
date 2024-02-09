import { client, load_call } from "$lib/load";

export const load = (async ({ fetch, depends }) => {
  depends("api:plans")
  depends("resource:plans");
  const plans = await load_call(() => client.GET("/plans", { fetch }));
  return { plans }
}) satisfies import("./$types").PageServerLoad;