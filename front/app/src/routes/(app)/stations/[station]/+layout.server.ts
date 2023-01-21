import { load_get } from "$lib/load.server";

export const load = (async ({ getClientAddress, parent, request, params, depends }) => {

  depends("station:limits");
  
  const { stations } = await parent();
  const station = stations.items.find(item => item._id === params.station);

  if(station != null) {
    return { station }
  } 

  const helper: import("$server/defs/api/stations/[station]/GET/Output").Output = await load_get(`/api/stations/${params.station}`, { getClientAddress, request });
  
  return {
    station: helper.station,
  }

}) satisfies import("./$types").LayoutServerLoad;