import { load_get } from "$lib/load";

export const load = (async ({ fetch, url, parent, depends, params }) => {

  depends("api:stations/:id");

  let current_page: null | "dashboard" | "profile" | "playlist" | "broadcast" = null;
  
  const { stations } = await parent();
  const station = stations.items.find(item => item._id === params.station);

  if(station != null) {
    return { station, current_page }
  } 

  const helper: import("$api/stations/[station]/GET/Output").Output = await load_get(`/api/stations/${params.station}`, { fetch, url });
  
  return {
    station: helper.station,
    current_page,
  }

}) satisfies import("./$types").LayoutLoad;