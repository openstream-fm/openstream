import { load_call, client } from "$lib/load";

export const load = (async ({ params, fetch, depends }) => {
  depends("resource:stations")
  depends("api:stations/:id/now-playing");
  depends("api:stations/:id/files");
  // TODO: implement pagination
  const [
    { files, playlist_is_randomly_shuffled },
    now_playing, 
  ] = await Promise.all([
    load_call(() => client.GET("/stations/{station}/files", { params: { path: { station: params.station }, query: { limit: 10_000 } } })),
    load_call(() => client.GET("/stations/{station}/now-playing", { params: { path: { station: params.station } }, fetch }))
  ]);

  return {
    files,
    playlist_is_randomly_shuffled,
    now_playing,
    current_page: "playlist",
  }
}) satisfies import("./$types").PageServerLoad;