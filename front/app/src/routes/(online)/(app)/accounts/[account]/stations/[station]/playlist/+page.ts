import { load_get } from "$lib/load";

export const load = (async ({ params, fetch, depends, url }) => {
  depends("resource:stations")
  depends("api:stations/:id/now-playing");
  depends("api:stations/:id/files");
  // TODO: implement pagination
  const [
    { files, playlist_is_randomly_shuffled },
    now_playing, 
  ] = await Promise.all([
    load_get<import("$server/defs/api/stations/[station]/files/GET/Output").Output>(`/api/stations/${params.station}/files?limit=10000`, { fetch, url }),
    load_get<import("$server/defs/api/stations/[station]/now-playing/GET/Output").Output>(`/api/stations/${params.station}/now-playing`, { fetch, url })
  ]);

  return {
    files,
    playlist_is_randomly_shuffled,
    now_playing,
    current_page: "playlist",
  }
}) satisfies import("./$types").PageLoad;