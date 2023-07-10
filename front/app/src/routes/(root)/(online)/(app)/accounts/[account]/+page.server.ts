import { load_get } from "$lib/load";

export const load = (async ({ fetch, url, parent, depends, params }) => {
  
  depends("api:accounts/:id/stream-stats");
  depends("api:accounts/:id/stream-stats/now/count-by-station");
  depends("api:stations/:id/now-playing");
  
  const [
    { stats },
    { by_station: sessions_by_station },
    now_playing_record,
  ] = await Promise.all([
    load_get<import("$api/accounts/[account]/stream-stats/GET/Output").Output>(`/api/accounts/${params.account}/stream-stats`, { fetch, url }),
    load_get<import("$api/accounts/[account]/stream-stats/now/count-by-station/GET/Output").Output>(`/api/accounts/${params.account}/stream-stats/now/count-by-station`, { fetch, url }),
    (async () => {
      const { account, stations } = await parent();
      const now_playing_record: Record<string, import("$api/stations/[station]/now-playing/GET/Output").Output | undefined> = {};
      const current_account_stations = stations.items.filter(item => item.account_id === account._id);
      await Promise.all(current_account_stations.map(async station => {
        const now_playing = await load_get<import("$api/stations/[station]/now-playing/GET/Output").Output>(`/api/stations/${station._id}/now-playing`, { url, fetch });
        now_playing_record[station._id] = now_playing;
      }))
      return now_playing_record;
    })()
  ])


  return { 
    stats,
    sessions_by_station,
    now_playing_record,
  }

}) satisfies import("./$types").PageServerLoad;