import { load_get } from "$lib/load";

export const load = (async ({ depends, params, fetch, url }) => {
  depends("resource:stations")
  depends("api:stations/:id/files");
  // TODO: implement pagination
  const { files, playlist_is_randomly_shuffled }: import("$server/defs/api/stations/[station]/files/GET/Output").Output = await load_get(`/api/stations/${params.station}/files?limit=10000`, { fetch, url })
  return { files, playlist_is_randomly_shuffled };
}) satisfies import("./$types").LayoutLoad;