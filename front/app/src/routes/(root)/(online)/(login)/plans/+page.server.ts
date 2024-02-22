import { load_call, client } from '$lib/load.js';

export const load = (async ({ depends, fetch }) => {
  depends("api:plans");
  depends("resource:plans");
  const items = await load_call(() => client.GET("/plans", { params: { query: { limit: 1_000 } }, fetch }));
  return { plans: items };
}) satisfies import("./$types").PageServerLoad;