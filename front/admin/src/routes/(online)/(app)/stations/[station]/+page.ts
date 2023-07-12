import { load_get } from "$lib/load";
import { error } from "@sveltejs/kit";

export const load = (async ({ parent, params, fetch, url }) => {
  
  const { all_stations, all_accounts } = await parent();

  const station = all_stations.find(item => item._id === params.station);
  
  if(station == null) throw error(404, { status: 404, message: "Station not found", code: "FRONT_RESOURCE_NOT_FOUND" })

  const account = all_accounts.find(account => station.account_id === account._id);

  const [
    now_playing,
    { stats },
  ] = await Promise.all([
    load_get<import("$api/stations/[station]/now-playing/GET/Output").Output>(`/api/stations/${params.station}/now-playing`, { fetch, url }),
    load_get<import("$api/stations/[station]/stream-stats/GET/Output").Output>(`/api/stations/${params.station}/stream-stats`, { fetch, url }),
  ]);

  return {  station, account, now_playing, stats }

}) satisfies import("./$types").PageLoad;
