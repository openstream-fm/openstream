import { load_get } from "$lib/load";
import { error, redirect } from "@sveltejs/kit";

export const load = (async ({ fetch, url, parent, depends, params }) => {

  depends("api:stations/:id");

  let current_page: null | "dashboard" | "profile" | "playlist" | "broadcast" | "settings" = null;
  
  const { stations, account } = await parent();
  const station = stations.items.find(item => item._id === params.station);
  

  if(station != null) {
    if(station.account_id !== account._id) {
      throw new error(404, {
        status: 404,
        code: "CLIENT_STATION_ACCOUNT_MISMATCH",
        message: `Station with id ${station._id} doesn't belong to this account`,
      })
    }
    return { station, current_page }
  } 

  const helper: import("$api/stations/[station]/GET/Output").Output = await load_get(`/api/stations/${params.station}`, { fetch, url });
  
  if(helper.station.account_id !== account._id) {
    throw new error(404, {
      status: 404,
      code: "CLIENT_STATION_ACCOUNT_MISMATCH",
      message: `Station with id ${station._id} doesn't belong to this account`,
    })
  }

  return {
    station: helper.station,
    current_page,
  }

}) satisfies import("./$types").LayoutLoad;