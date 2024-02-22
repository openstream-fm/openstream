import { client, load_call } from "$lib/load";
import { error } from "@sveltejs/kit";

export const load = (async ({ parent, params, fetch }) => {
  
  const { all_stations, all_accounts } = await parent();

  const station = all_stations.find(item => item._id === params.station);
  
  if(station == null) error(404, { status: 404, message: "Station not found", code: "FRONT_RESOURCE_NOT_FOUND" });

  const account = all_accounts.find(account => station.account_id === account._id);

  const [
    now_playing,
    { stats },
  ] = await Promise.all([
    load_call(() => client.GET("/stations/{station}/now-playing", { params: { path: { station: params.station } }, fetch })),
    load_call(() => client.GET("/stations/{station}/stream-stats", { params: { path: { station: params.station } }, fetch })),
  ]);

  return {  station, account, now_playing, stats }

}) satisfies import("./$types").PageServerLoad;
