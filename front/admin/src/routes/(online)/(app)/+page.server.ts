import { client, load_call } from "$lib/load";

export const load = (async ({ fetch }) => {
  const { stats } = await load_call(() => client.GET("/stream-stats", { fetch }));
  return { stats }
}) satisfies import("./$types").PageServerLoad;