import { load_get } from '$lib/load.js';

export const load = (async ({ depends, url, fetch }) => {
  depends("api:plans");
  depends("resource:plans");
  const items = await load_get<import("$server/defs/api/plans/GET/Output").Output>(`/api/plans`, { url, fetch });
  return { plans: items };
}) satisfies import("./$types").PageLoad;