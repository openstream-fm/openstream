import { load_get } from "$lib/load";

export const load = (async ({ fetch, url, parent, depends, params }) => {

  depends("station:limits");
  
  const { stations } = await parent();
  const station = stations.items.find(item => item._id === params.station);

  if(station != null) {
    return { station }
  } 

  const helper: import("$server/defs/api/stations/[station]/GET/Output").Output = await load_get(`/api/stations/${params.station}`, { fetch, url });
  
  return {
    station: helper.station,
  }

}) satisfies import("./$types").LayoutLoad;